//! 世界 - 二进制入口
//!
//! Round 4 改用动态工作空间状态 API。
//!
//! 决策锚：260826-2240 传承殿启动
//! 关联文档：传承殿/README.md

fn main() {
    println!("洪荒 · 世界 v3 · {} · 工作空间就绪", shijie::阶段);
    println!("workspace 成员 = {} 个", shijie::成员清单().len());
    for (i, m) in shijie::成员清单().iter().enumerate() {
        println!("  {:2}. {}", i + 1, m);
    }
}
