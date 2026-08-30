//! 温度扫描 - 对 3 档终裁温度各跑 N 次真实任务，记录通过/打回，输出「温度→通过率」表
//!
//! 决策锚：100项任务 任务1 终裁温度系统性调优（真实通过可复现核心）
//! 用法：cargo run -p mingling_caozuo_fu --example 温度扫描 -- <任务标识> <每档次数>
//! 验收：产出一份「温度→通过率」实测表（选稳定档，可复现=同温度连跑通过率高且稳定）

use mingling_caozuo_fu::跑流水线_真实_llm;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let 任务标识 = args.first().map(|s| s.as_str()).unwrap_or("整数加法函数");
    let 每档次数: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    // 第三参数=温度列表（逗号分隔），默认 0.1,0.3,0.7；可单档聚焦（如「0.1」连跑验证可复现）
    let 温度列表: Vec<String> = args
        .get(2)
        .map(|s| s.split(',').map(|t| t.to_string()).collect())
        .unwrap_or_else(|| {
            ["0.1", "0.3", "0.7"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        });

    println!(
        "=== 终裁温度扫描：任务「{}」 每档 {} 次（温度 {:#?}）===",
        任务标识, 每档次数, 温度列表
    );
    for 温度 in 温度列表 {
        std::env::set_var("LLM_终裁温度", &温度);
        let mut 通过 = 0usize;
        let mut 打回 = 0usize;
        for 序 in 0..每档次数 {
            let 结果 = 跑流水线_真实_llm(任务标识);
            let 交付 = 结果.输出.contains("[交付] 登记并交付");
            if 交付 {
                通过 += 1;
            } else {
                打回 += 1;
            }
            println!(
                "  温度{} 第{}次: {}",
                温度,
                序 + 1,
                if 交付 { "通过" } else { "打回" }
            );
        }
        let 通过率 = (通过 * 100).checked_div(每档次数).unwrap_or(0);
        println!(
            "温度 {}: 通过 {}/{} = {}%（打回 {}）",
            温度, 通过, 每档次数, 通过率, 打回
        );
    }
}
