//! 预写任务 - 把实质任务描述写入持久库，供真实流水线召回（构造真实「通过」上下文）
//!
//! 决策锚：260829 真实通过端到端验证（Round 19）
//! 用法：cargo run -p mingling_caozuo_fu --example 预写任务 -- <任务标识> <任务描述>
//! 验证：写入后真实流水线读任务记忆召回该描述，道祖终裁获得实质上下文

use mingling_caozuo_fu::{写入任务记忆, 退出码, 默认记忆库路径};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("用法：预写任务 <任务标识> <任务描述>");
        std::process::exit(退出码::材料错误);
    }
    let 任务标识 = args[0].as_str();
    let 任务描述 = args[1..].join(" ");
    写入任务记忆(默认记忆库路径, &任务描述, 任务标识).expect("任务记忆写入失败");
    println!("已写入任务记忆：{} => {}", 任务标识, 任务描述);
}
