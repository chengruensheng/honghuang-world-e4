//! 自举审计：输出「真实通过→交付→回填→债务清零」的完整证据链
//!
//! 决策锚：260829 阶段 C 自举闭环验收 = 真实通过→回填 1 块 + 债务清零
//! 用法：cargo run -p mingling_caozuo_fu --example 自举审计 -- <任务标识>
//! 证据链：执行×产出块数（回填证据）+ 账本债务（债务清零证据）+ 待补提炼（降级快照）

use jiyi_chengzai_fu::{总纲, 本质, 格位, 格位中枢, SQLite存储};
use mingling_caozuo_fu::默认记忆库路径;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let 任务标识 = args
        .first()
        .map(|s| s.as_str())
        .unwrap_or("四时机召回验证v2");

    // 审计 1：执行×产出 块（回填证据——真实通过后后端整理回填 1 块）
    let 存储 = SQLite存储::文件新建(默认记忆库路径).expect("记忆库打开失败");
    let 中枢 = 格位中枢::新建(存储);
    let 产出块 = 中枢.读取_格位_分区(格位 {
        总纲: 总纲::执行,
        本质: 本质::产出,
    });
    let 绑定产出: Vec<_> = 产出块
        .iter()
        .filter(|e| {
            e.块元数据
                .as_ref()
                .map(|m| m.绑定任务 == 任务标识)
                .unwrap_or(false)
        })
        .collect();
    println!(
        "[自举审计·回填] 执行×产出 绑定「{}」块数 = {}",
        任务标识,
        绑定产出.len()
    );
    for (序, e) in 绑定产出.iter().enumerate() {
        println!(
            "  块{} id={} 摘要={}（来源 {:?}，decided_by={}）",
            序, e.id.0, e.摘要, e.来源, e.decided_by
        );
    }

    // 审计 2：账本债务（债务清零证据——交付 +1 后后端还债归档 −1）
    let 存储2 = SQLite存储::文件新建(默认记忆库路径).expect("记忆库打开失败");
    let 双工 = jiyi_chengzai_fu::双工流水线::新建(存储2);
    let 债务 = 双工.债务().unwrap_or(-1);
    println!("[自举审计·债务] 当前账本债务 = {}", 债务);
    println!(
        "[自举审计·待补] 待补提炼快照 = {:?}",
        双工.待补提炼().unwrap_or_default()
    );
    println!(
        "[自举审计·结论] 回填{}块 + 债务{} = 自举闭环{}",
        绑定产出.len(),
        债务,
        if !绑定产出.is_empty() && 债务 == 0 {
            "达成"
        } else {
            "未达成"
        }
    );
}
