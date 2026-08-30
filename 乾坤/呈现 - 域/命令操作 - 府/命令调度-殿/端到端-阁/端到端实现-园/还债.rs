//! 双工还债闭环（契约 §六）：前端登记并交付（fire-and-forget）+ 后端整理 worker + 还债至清零
//!
//! 前端只记账产生债务，提炼回填交后端；债务=已交付−已归档，后端失败只记日志不影响前端成功。

use super::模拟_llm::{产出最大令牌, 脱敏, 调用_带重试};
use jiyi_chengzai_fu::{
    内容类别, 双工流水线, 回填器, 整理结果, 格位中枢, SQLite存储
};
use moxing_fu::{模型连接, 消息, 请求, LLM调用器};

/// 前端登记并交付（fire-and-forget）：只记账产生债务，提炼回填交后端整理
pub(crate) fn 前端_登记并交付(
    记忆库路径: &str, 任务标识: &str
) -> Result<(), String> {
    let 存储 = SQLite存储::文件新建(记忆库路径).map_err(|e| e.to_string())?;
    let mut 双工 = 双工流水线::新建(存储);
    双工.前端_登记(任务标识).map_err(|e| e.to_string())?;
    双工.前端_交付(任务标识).map_err(|e| e.to_string())
}

/// 后端整理 worker（契约 §六 后端闭环）：双工（账本）+ 回填器（格位）+ LLM 提炼 串成端到端
///
/// 前端 fire-and-forget 只登记/交付；本 worker 驱动后端整理——取最早债务任务，
/// 超限或提炼失败→降级归档（快照暂存待补），否则 LLM 提炼 → 回填器归类写格位 → 归档。
/// 同一 SQLite 文件两个连接：账本（双工）+ 格位（回填器），数据天然共享。
pub fn 后端整理_一个<C: 模型连接>(
    记忆库路径: &str,
    调用器: &LLM调用器<C>,
) -> Result<Option<整理结果>, String> {
    let 账本存储 = SQLite存储::文件新建(记忆库路径).map_err(|e| e.to_string())?;
    let mut 双工 = 双工流水线::新建(账本存储);
    let 格位存储 = SQLite存储::文件新建(记忆库路径).map_err(|e| e.to_string())?;
    let mut 回填器 = 回填器::新建(格位中枢::新建(格位存储));

    // 整理回调：LLM 提炼 → 回填器归类写格位；失败返回 false → 降级（快照暂存待补提炼）
    let mut 整理 = |任务: &str| -> bool {
        let 提炼消息 = vec![
            消息::系统(
                "你是大罗级（执行整理）。把任务上下文提炼为精炼记忆块：去噪、保留关键产出、格式化，只输出精炼块内容。铁律：关键产出代码块必须与实际实现语言字符级一致（Rust 中文标识符），严禁 Python 语法/英文标识符——语言漂移即拒收。",
            ),
            消息::用户(format!("任务：{}", 任务)),
        ];
        match 调用_带重试(
            调用器,
            "大罗",
            &请求::新建("", 提炼消息).设最大token(产出最大令牌),
        ) {
            Ok(响应) => {
                let 精炼 = 响应.内容;
                回填器
                    .回填(内容类别::代码, 任务, 任务, "大罗级", |_| {
                        精炼.clone()
                    })
                    .is_ok()
            }
            Err(_) => false,
        }
    };

    双工
        .后端_整理_一个_带整理(&mut 整理)
        .map_err(|e| e.to_string())
}

/// 后端还债循环（契约 §六 后端闭环）：读债务队列驱动整理，债务=0 时返回（无活不空转）
/// 前端 fire-and-forget 交付后由编排层调用；后端失败只记日志，不影响前端成功
pub fn 后端_还债_至清零<C: 模型连接>(
    记忆库路径: &str,
    调用器: &LLM调用器<C>,
    日志: &mut String,
) {
    loop {
        match 后端整理_一个(记忆库路径, 调用器) {
            Ok(Some(整理)) => {
                日志.push_str(&format!("[后端整理] {:?}\n", 整理));
            }
            Ok(None) => break,
            Err(e) => {
                日志.push_str(&format!("[后端整理失败·存储] {}\n", 脱敏(e)));
                break;
            }
        }
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 前端登记并交付_产生债务() {
        let 路径 = std::env::temp_dir().join(format!("洪荒还债测试_{}.sq3", std::process::id()));
        let 路径_str = 路径.to_str().unwrap();
        前端_登记并交付(路径_str, "任务甲").expect("登记并交付应成功");
        let 存储 = SQLite存储::文件新建(路径_str).unwrap();
        let 双工 = 双工流水线::新建(存储);
        let 债务 = 双工.债务().expect("读债务应成功");
        assert!(债务 > 0, "交付后应产生债务：{}", 债务);
        let _ = std::fs::remove_file(&路径);
    }
}
