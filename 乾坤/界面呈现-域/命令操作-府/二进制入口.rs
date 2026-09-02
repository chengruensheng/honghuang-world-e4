//! 二进制入口（生产 CLI，可执行名「洪荒」）
//!
//! 真实生产流程最小闭环：透传命令操作-府 CLI。
//! 决策锚：260828-真实生产流程优先（原「世界」bin 归位至九根内「乾坤·命令操作-府」）
//! 后端语义（后端-解析-园）：
//!   未设置 / real → 真实 API（严禁降级 mock；无 LLM_API_KEY 时 fail-loud 退出码 4）
//!   LLM_BACKEND=mock → 确定性 MockLLM（测试/演示，默认 3 轮打回上限）
//! 用法：
//!   洪荒                       帮助
//!   洪荒 run `--task=<id>`     跑真实流水线（默认真实后端；LLM_BACKEND=mock 走 mock）
//!   洪荒 status                健康检查（工作空间实时快照）
//!   洪荒 --health              部署健康检查（退出码 0=健康）
//!   洪荒 记忆 `<子命令>`       记忆命令

#![allow(non_snake_case)]

use mingling_caozuo_fu::{
    写入任务记忆, 分发, 命令结果, 工具永驻摘要, 跑流水线_mock_llm, 默认记忆库路径,
};

fn main() {
    // 生产 CLI 首次启动即加载工作区根 .env（LLM_API_KEY / LLM_BASE_URL / LLM_MODEL）。
    // 此前从未加载 .env，导致用户已配置密钥却仍报「LLM_API_KEY 未设置」——真实后端从未跑通。
    // 决策锚：260902-可用打磨（.env 加载缺陷根治）
    加载工作区_env();

    let 参数: Vec<String> = std::env::args().skip(1).collect();
    let 参数引用: Vec<&str> = 参数.iter().map(|s| s.as_str()).collect();

    let 结果 = 分发生产(&参数引用);

    print!("{}", 结果.输出);
    std::process::exit(结果.退出码);
}

/// 加载工作区根 .env（不依赖 dotenv crate，手工解析 KEY=VALUE）
///
/// 从当前目录逐级向上查找 .env，找到即解析；已存在的环境变量不覆盖（系统显式值优先）。
/// 覆盖 `cargo run`（cwd=根）与从子目录直接运行 exe 两种场景。
fn 加载工作区_env() {
    let 候选目录 = 工作区根候选();
    for 目录 in 候选目录 {
        let 路径 = 目录.join(".env");
        let Ok(内容) = std::fs::read_to_string(&路径) else {
            continue;
        };
        for 行 in 内容.lines() {
            let 行 = 行.trim();
            if 行.is_empty() || 行.starts_with('#') {
                continue;
            }
            if let Some((键, 值)) = 行.split_once('=') {
                let 键 = 键.trim();
                let 值 = 值.trim().trim_matches('"');
                // 不覆盖系统显式设置的值（命令行/环境注入优先于 .env）
                if std::env::var(键).is_err() {
                    std::env::set_var(键, 值);
                }
            }
        }
        return;
    }
}

/// 生成 .env 查找候选目录：当前目录 → 逐级父目录（最多 6 层）
fn 工作区根候选() -> Vec<std::path::PathBuf> {
    let mut 候选 = Vec::new();
    let Ok(当前) = std::env::current_dir() else {
        return 候选;
    };
    let mut 目录 = 当前.as_path();
    候选.push(目录.to_path_buf());
    for _ in 0..6 {
        match 目录.parent() {
            Some(父) => {
                目录 = 父;
                候选.push(目录.to_path_buf());
            }
            None => break,
        }
    }
    候选
}

/// 生产分发：run 走真实后端，--health 走就绪检测，其余透传命令操作-府
fn 分发生产(参数: &[&str]) -> 命令结果 {
    if 参数.is_empty() {
        return 分发(参数);
    }

    match 参数[0] {
        "--health" | "health" => {
            // 部署健康检查：就绪即健康（部署文档铁律：可观测）
            let 就绪 = 分发(&["就绪"]);
            if 就绪.退出码 == 0 {
                命令结果::成功("健康检查通过：工作空间就绪\n".to_string())
            } else {
                命令结果::失败(1, "健康检查失败：工作空间未就绪\n")
            }
        }
        "run" => {
            // 真实生产：run --task=<id> 按 LLM_BACKEND 选后端（默认真实，严禁降级 mock）
            let 任务 = 参数.iter().find_map(|a| a.strip_prefix("--task="));
            match 任务 {
                Some(t) => {
                    let mut 结果 = 跑流水线_mock_llm(t);
                    // 任务后写程序/实施记忆 + 刷新永驻摘要（真实生产留痕可审计）
                    if 结果.退出码 == 0 {
                        if let Err(错) =
                            写入任务记忆(默认记忆库路径, &format!("流水线执行完成：{}", t), t)
                        {
                            结果.输出.push_str(&format!("[记忆写入失败] {}\n", 错));
                        }
                        let 摘要 = 工具永驻摘要(默认记忆库路径);
                        let 有内容 = 摘要.iter().filter(|s| !s.ends_with("] ")).count();
                        结果.输出.push_str(&format!(
                            "任务后记忆闭环：永驻摘要 {} 行，其中 {} 行有内容\n",
                            摘要.len(),
                            有内容
                        ));
                    }
                    结果
                }
                None => {
                    命令结果::失败(2, "用法：run --task=<id>（真实 LLM / mock 自动选择）\n")
                }
            }
        }
        _ => 分发(参数),
    }
}
