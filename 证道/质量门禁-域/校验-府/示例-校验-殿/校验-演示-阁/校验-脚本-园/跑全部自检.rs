//! 跑全部自检 - 对工作区根实跑 15 项架构校验，任何失败项使进程退出码非 0。
//!
//! 用法：cargo run -p jianyan_gongju --example 跑全部自检 -- .
use jianyan_gongju::脚本_校验_殿::跑全部;
use std::path::PathBuf;

fn main() {
    let 根 = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .expect("需要工作空间根");
    let 项 = 跑全部(&根);
    let mut 失败 = 0u32;
    for x in &项 {
        if x.结果.是否失败() {
            失败 += 1;
            println!("[FAIL] [{}] {} {:?}", x.编号, x.名称, x.结果);
        } else if x.结果.是否警告() {
            println!("[WARN] [{}] {}", x.编号, x.名称);
        } else {
            println!("[PASS] [{}] {}", x.编号, x.名称);
        }
    }
    if 失败 > 0 {
        eprintln!("跑全部：{} 项失败", 失败);
        std::process::exit(1);
    }
    println!("跑全部：{} 项全过（警告不算失败）", 项.len());
}
