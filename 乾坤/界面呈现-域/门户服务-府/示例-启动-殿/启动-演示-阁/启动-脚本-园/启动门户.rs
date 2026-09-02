//! 启动门户 - 运行入口（example）
//!
//! 用法：
//!   cargo run -p menhu_fuwu_fu --example 启动门户
//! 环境变量（ASCII 优先，中文名备选）：
//!   MENHU_PORT / 门户端口    （默认 8020）
//!   MEMORY_DB  / 记忆库路径  （默认 ./洪荒记忆库.sq3）

use menhu_fuwu_fu::{启动门户, 默认门户端口};

fn main() {
    let 端口: u16 = std::env::var("MENHU_PORT")
        .or_else(|_| std::env::var("门户端口"))
        .ok()
        .and_then(|值| 值.parse().ok())
        .unwrap_or(默认门户端口);
    let 记忆库路径 = std::env::var("MEMORY_DB")
        .or_else(|_| std::env::var("记忆库路径"))
        .unwrap_or_else(|_| "洪荒记忆库.sq3".to_string());

    println!("=== 洪荒 · 智能体工坊 — 门户服务 ===");
    println!("数据源：{}", 记忆库路径);
    let 服务 = match 启动门户(端口, 记忆库路径.clone()) {
        Ok(服务) => 服务,
        Err(错误) => {
            eprintln!("启动失败：{}", 错误);
            std::process::exit(1);
        }
    };
    println!("门户已启动：{}", 服务.首页地址());
    println!("接口：/api/总览  /api/任务  /api/事件  /api/记忆  /api/仙官  /api/切面");
    println!("按 Ctrl+C 停止");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
