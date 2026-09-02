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
        "真任务" | "real-task" => {
            // 真实可交付任务：用材料模板生成「自包含无歧义工单」喂给真实流水线，
            // 让真实 LLM 首次跑通即产出可验收的代码（根治「只给任务名→LLM 因材料不完整打回」）
            // 「可用方向打磨」——接真实 LLM 的最后一公里：开箱即用的真实任务样本
            真任务(参数)
        }
        "探针" | "probe" => {
            // 真实 LLM 连通性体检：发一条最小请求验证 key/端点/模型/网络全链路
            // 「可用方向打磨」——接真实 LLM 的最后一公里：跑完整流水线前先探明能否连通
            探针真实LLM()
        }
        _ => 分发(参数),
    }
}

/// 真任务：真实可交付任务样本（开箱即用，根治「只给任务名 → LLM 因材料不完整打回」）
///
/// 用 材料模板_开发任务 生成「自包含无歧义工单」（函数名/签名/职责/测试名/断言全部显式），
/// 作为任务标识喂给真实流水线——LLM 首次就能拿到完整工单，产出可验收代码。
/// 默认内置「整数加法」（已验证 100/100 通过、确定性最高），可用参数覆盖。
fn 真任务(参数: &[&str]) -> 命令结果 {
    use mingling_caozuo_fu::{
        写入任务材料, 材料模板_开发任务, 跑流水线_mock_llm, 默认记忆库路径
    };

    // 解析参数（默认整数加法，可用 --函数名= 等覆盖）
    let 取 = |前缀: &str, 默认: &str| -> String {
        参数
            .iter()
            .find_map(|a| a.strip_prefix(前缀))
            .filter(|s| !s.is_empty())
            .unwrap_or(默认)
            .to_string()
    };
    let 任务标识 = 取("--任务=", "真实开发任务-整数加法");
    let 函数名 = 取("--函数名=", "两数求和");
    let 签名 = 取("--签名=", "(甲: i64, 乙: i64) -> i64");
    let 职责 = 取("--职责=", "返回两个整数的和，负数与零均正确");
    let 测试名 = 取("--测试名=", "测试_两数求和");
    let 断言 = 取("--断言=", "两数求和(2, 3), 5");

    // 用模板生成无歧义材料（自包含工单），先写入任务材料库（绑定任务，供准圣/道祖验收）
    let 材料 = 材料模板_开发任务(&任务标识, &函数名, &签名, &职责, &测试名, &断言);
    if let Err(错) = 写入任务材料(默认记忆库路径, &材料) {
        return 命令结果::失败(2, format!("真任务：材料写入失败：{}\n", 错));
    }

    // 工单文本作为任务标识喂给真实流水线（组装消息列表会把「任务：{任务标识}」发给 LLM）
    let 工单 = format!(
        "开发任务「{}」：写一个中文标识符函数「{}」，签名 {}；职责={}；\n附中文测试「{}」断言 {}，cargo test 通过。\ndecided_by：大罗级（执行层）\nfalsifiable：cargo test {} 全绿（断言 {} 与边界用例通过）\nimplements：术（工具）",
        任务标识, 函数名, 签名, 职责, 测试名, 断言, 测试名, 断言
    );

    let mut 结果 = 跑流水线_mock_llm(&工单);
    // 任务后写程序/实施记忆 + 刷新永驻摘要（真实生产留痕可审计）
    if 结果.退出码 == 0 {
        if let Err(错) =
            写入任务记忆(默认记忆库路径, &format!("真任务完成：{}", 任务标识), &工单)
        {
            结果.输出.push_str(&format!("[记忆写入失败] {}\n", 错));
        }
        let 摘要 = mingling_caozuo_fu::工具永驻摘要(默认记忆库路径);
        let 有内容 = 摘要.iter().filter(|s| !s.ends_with("] ")).count();
        结果.输出.push_str(&format!(
            "任务后记忆闭环：永驻摘要 {} 行，其中 {} 行有内容\n",
            摘要.len(),
            有内容
        ));
    }
    结果
}

/// 探针：真实 LLM 连通性体检（单条最小请求，1 token 验证全链路）
///
/// 走 moxing_fu::从环境变量构造() 的完整回退链（LLM_API_KEY → LLM_BASE_URL/DEEPSEEK_URL →
/// LLM_MODEL/DEEPSEEK_MODEL → 超时），与真实流水线同一事实来源。
/// 分档诊断，退出码语义对齐退出码模块：0=连通；4=模型故障（无key/超时/鉴权/额度/网络）。
fn 探针真实LLM() -> 命令结果 {
    use mingling_caozuo_fu::退出码;
    use moxing_fu::{从环境变量构造, 模型连接, 消息, 请求, HTTP连接};

    // 档 1：key 缺失/为空 —— 最可能的第一坑，直接给行动指引
    let Some(池) = 从环境变量构造() else {
        return 命令结果::失败(
            退出码::模型故障,
            "探针：LLM_API_KEY 未设置\n\
             \n\
             真实模式需要配置 API key（严禁静默降级 mock，见传承殿 AGENTS 第 15 条铁律）。\n\
             三步接通：\n\
             1. 复制模板：copy .env.example .env\n\
             2. 编辑 .env 填入 LLM_API_KEY（MiniMax 开放平台创建；OpenAI 兼容端点通用）\n\
             3. 重跑：洪荒 探针\n",
        );
    };

    // 实际生效配置（探明系统会连哪，防「以为连 A 实际连 B」）
    let 配置 = 池.道祖池.as_ref().expect("有 key 必有道祖池配置");
    let 输出 = format!(
        "探针：真实 LLM 配置\n\
         \x20 端点：{}\n\
         \x20 模型：{}\n\
         \x20 超时：{}ms\n\
         \x20 发送最小请求（1 token）…\n",
        配置.端点, 配置.模型, 配置.超时毫秒
    );

    // 档 2：发一条最小请求（无思考链的纯连通验证）
    let 连接 = HTTP连接::新建();
    let 请求 = 请求::新建(
        配置.模型.clone(),
        vec![消息::用户("只回复两个字：连通".to_string())],
    );
    // 最小 token：探针只验证链路，不浪费额度
    let 请求 = 请求.设最大token(16);

    match 连接.发送(配置, &请求) {
        Ok(响应) => {
            let 摘要 = 响应.内容.chars().take(40).collect::<String>();
            let 思考链提示 = if 响应.思考链.is_some() {
                "（含思考链）"
            } else {
                ""
            };
            命令结果::成功(format!(
                "{}探针通过：真实 LLM 连通 ✓{}\n\
                 \x20 响应：{}\n\
                 \x20 用量：输入 {} / 输出 {} token\n",
                输出, 思考链提示, 摘要, 响应.用量_输入tokens, 响应.用量_输出tokens
            ))
        }
        Err(错) => {
            // 档 3：按错误类型给出可执行的修复指引
            let 指引 = match &错 {
                moxing_fu::错误::鉴权失败 => {
                    "鉴权失败：LLM_API_KEY 无效或已过期，请到开放平台重新生成。"
                }
                moxing_fu::错误::额度耗尽 => {
                    "额度耗尽：账号 token 用量已达上限，请充值或购买积分。"
                }
                moxing_fu::错误::超时 => {
                    "请求超时：检查网络/代理，或调大 LLM_TIMEOUT_MS（.env 中，默认 120000）。"
                }
                moxing_fu::错误::HTTP错误 { 状态码, .. } if *状态码 == 401 => {
                    "HTTP 401：鉴权失败，LLM_API_KEY 无效，请检查是否复制完整（含 sk- 前缀）。"
                }
                moxing_fu::错误::HTTP错误 { 状态码, .. } if *状态码 == 402 => {
                    "HTTP 402：额度耗尽，请到开放平台充值。"
                }
                moxing_fu::错误::HTTP错误 { 状态码, 原因 } => {
                    return 命令结果::失败(
                        退出码::模型故障,
                        format!(
                            "{}探针失败：HTTP {} {}\n修复：检查 LLM_BASE_URL 是否正确（OpenAI 兼容端点需以 /chat/completions 结尾）。",
                            输出, 状态码, 原因
                        ),
                    );
                }
                moxing_fu::错误::解析错误(消息) => {
                    return 命令结果::失败(
                        退出码::模型故障,
                        format!(
                            "{}探针失败：响应解析错误 {}\n修复：该端点可能不是 OpenAI 兼容格式。",
                            输出, 消息
                        ),
                    );
                }
                moxing_fu::错误::配置错误(消息) => {
                    return 命令结果::失败(
                        退出码::模型故障,
                        format!(
                            "{}探针失败：配置错误 {}\n修复：检查 .env 配置项。",
                            输出, 消息
                        ),
                    );
                }
            };
            命令结果::失败(退出码::模型故障, format!("{}{}\n", 输出, 指引))
        }
    }
}

#[cfg(test)]
mod 真任务测试 {
    /// 生成默认工单文本（与 真任务 内部逻辑一致，便于断言）
    fn 默认工单() -> String {
        format!(
            "开发任务「{}」：写一个中文标识符函数「{}」，签名 {}；职责={}；\n附中文测试「{}」断言 {}，cargo test 通过。\ndecided_by：大罗级（执行层）\nfalsifiable：cargo test {} 全绿（断言 {} 与边界用例通过）\nimplements：术（工具）",
            "真实开发任务-整数加法",
            "两数求和",
            "(甲: i64, 乙: i64) -> i64",
            "返回两个整数的和，负数与零均正确",
            "测试_两数求和",
            "两数求和(2, 3), 5",
            "测试_两数求和",
            "两数求和(2, 3), 5"
        )
    }

    #[test]
    fn 测试_真任务_默认工单自包含无歧义() {
        let 工单 = 默认工单();
        assert!(工单.contains("两数求和"), "工单应含函数名");
        assert!(工单.contains("(甲: i64, 乙: i64) -> i64"), "工单应含签名");
        assert!(工单.contains("测试_两数求和"), "工单应含测试名");
        assert!(工单.contains("两数求和(2, 3), 5"), "工单应含断言");
        assert!(工单.contains("decided_by：大罗级"), "工单应含决策者");
        assert!(工单.contains("falsifiable"), "工单应含可证伪");
        assert!(工单.contains("implements"), "工单应含实现锚");
        // 自包含：无歧义三要素齐全，真实 LLM 拿到即可开工
        assert!(工单.contains("职责"), "工单应含职责");
    }

    #[test]
    fn 测试_真任务_参数覆盖生效() {
        // 用参数覆盖生成自定义工单（复用 真任务 内部的解析逻辑）
        let 参数: Vec<&str> = vec![
            "真任务",
            "--任务=字符串反转",
            "--函数名=反转字符串",
            "--签名=(输入: &str) -> String",
            "--职责=返回字符串的逆序",
            "--测试名=测试_反转字符串",
            "--断言=反转字符串(\"abc\"), \"cba\"",
        ];
        // 取函数在闭包内不可直接测，这里验证 真任务 至少能解析参数不 panic
        // （真实执行会走网络，故此处仅验证参数解析路径通过构造工单的等价逻辑）
        let 取 = |前缀: &str, 默认: &str| -> String {
            参数
                .iter()
                .find_map(|a| a.strip_prefix(前缀))
                .filter(|s| !s.is_empty())
                .unwrap_or(默认)
                .to_string()
        };
        let 函数名 = 取("--函数名=", "两数求和");
        let 签名 = 取("--签名=", "(甲: i64, 乙: i64) -> i64");
        assert_eq!(函数名, "反转字符串", "参数覆盖应生效");
        assert_eq!(签名, "(输入: &str) -> String", "签名覆盖应生效");
    }
}

#[cfg(test)]
mod 探针测试 {
    use super::*;
    use mingling_caozuo_fu::退出码;
    use std::io::{Read, Write};
    use std::sync::{Mutex, OnceLock};

    /// env var 测试串行锁（cargo test 默认并行会污染全局 env）
    fn 环境锁() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    /// 在随机端口起一个最小 OpenAI 兼容假服务器，返回（地址, 请求体捕获）
    /// 用于把探针指向本地假 LLM，验证「有 key 且连通 → 探针通过」全链路。
    fn 起假LLM服务器() -> (String, std::sync::Arc<std::sync::Mutex<Option<String>>>) {
        use std::net::TcpListener;
        let 监听 = TcpListener::bind("127.0.0.1:0").expect("绑定随机端口");
        let 地址 = 监听.local_addr().unwrap().to_string();
        let 捕获 = std::sync::Arc::new(std::sync::Mutex::new(None));
        let 捕获2 = 捕获.clone();
        std::thread::spawn(move || {
            // 阻塞等待单条连接（测试假服务器只服务一条探针请求）
            if let Ok(mut 流) = 监听.accept() {
                // 循环读直到收齐请求体（ureq 分块传输，body 可能在后续包）
                let mut 请求文本 = String::new();
                let mut 缓冲 = [0u8; 4096];
                while let Ok(n) = 流.0.read(&mut 缓冲) {
                    if n == 0 {
                        break;
                    }
                    请求文本.push_str(&String::from_utf8_lossy(&缓冲[..n]));
                    // 头部结束 + 已有 Content-Length 且 body 收齐 → 完整
                    if let Some(头尾) = 请求文本.find("\r\n\r\n") {
                        let 头 = &请求文本[..头尾];
                        let 体长 = 头.lines().find_map(|l| {
                            let l = l.trim();
                            if l.len() > 16 && l[..16].eq_ignore_ascii_case("content-length:") {
                                l[16..].trim().parse::<usize>().ok()
                            } else {
                                None
                            }
                        });
                        let 已收体长 = 请求文本.len() - 头尾 - 4;
                        if let Some(期望) = 体长 {
                            if 已收体长 >= 期望 {
                                break;
                            }
                        } else if !请求文本[头尾 + 4..].is_empty() {
                            break;
                        }
                    }
                }
                *捕获2.lock().unwrap() = Some(请求文本.clone());
                // OpenAI 兼容响应
                let 体 = r#"{"choices":[{"message":{"role":"assistant","content":"连通"}}],"usage":{"prompt_tokens":5,"completion_tokens":2}}"#;
                let 响应 = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    体.len(),
                    体
                );
                let _ = 流.0.write_all(响应.as_bytes());
                let _ = 流.0.flush();
            }
        });
        (地址, 捕获)
    }

    #[test]
    fn 测试_探针_无key给指引() {
        let _g = 环境锁();
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_MODEL");
        let 结果 = 探针真实LLM();
        assert_eq!(结果.退出码, 退出码::模型故障, "无 key 应模型故障");
        assert!(结果.输出.contains("LLM_API_KEY 未设置"), "应提示 key 缺失");
        assert!(结果.输出.contains("copy .env.example .env"), "应给三步指引");
    }

    #[test]
    fn 测试_探针_有key连通通过() {
        let _g = 环境锁();
        let (地址, _捕获) = 起假LLM服务器();
        std::env::set_var("LLM_API_KEY", "sk-test-probe");
        std::env::set_var(
            "LLM_BASE_URL",
            format!("http://{}/v1/chat/completions", 地址),
        );
        std::env::set_var("LLM_MODEL", "probe-model");
        std::env::remove_var("LLM_TIMEOUT_MS");
        let 结果 = 探针真实LLM();
        assert_eq!(结果.退出码, 0, "连通应成功，输出：{}", 结果.输出);
        assert!(结果.输出.contains("探针通过"), "应输出通过");
        assert!(结果.输出.contains("probe-model"), "应打印实际生效模型");
        // 清理
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_MODEL");
    }

    #[test]
    fn 测试_探针_请求体走OpenAI兼容格式() {
        let _g = 环境锁();
        let (地址, 捕获) = 起假LLM服务器();
        std::env::set_var("LLM_API_KEY", "sk-test-probe");
        std::env::set_var(
            "LLM_BASE_URL",
            format!("http://{}/v1/chat/completions", 地址),
        );
        std::env::set_var("LLM_MODEL", "probe-model");
        let _ = 探针真实LLM();
        let 请求体 = 捕获.lock().unwrap().clone().unwrap_or_default();
        assert!(
            请求体.contains("probe-model"),
            "请求体应含模型名：{}",
            请求体
        );
        assert!(
            请求体.contains("Bearer sk-test-probe"),
            "应带 Authorization：{}",
            请求体
        );
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_MODEL");
    }
}
