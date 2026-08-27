//! 端到端自开发测试 (Round 10)
//!
//! 流程：
//! 1) 用 minimax LLM 接收任务描述 + D 盘命名规范 → 生成 Rust crate 代码
//! 2) 写入磁盘目标路径
//! 3) 跑 cargo test + 一键全验 13 项
//! 4) 失败重试 3 次
//! 5) 最终 commit
//!
//! 验收标准（falsifiable）：
//! - 新 crate 路径存在 + 命名规范（祖孙三层语义判据）
//! - cargo test 全过 + 一键全验 13/13 全绿 + clippy 零警告
//! - git commit 中文 message

use std::process::Command;

const 任务描述: &str = "在 乾坤/ 域下创建一个新 crate 工具-hello-府（lib 名 gongju_hello_fu）：
  - 2 殿 ≥2 阁 ≥1 园 六层结构
  - 入口.rs 提供 pub fn greet(name: &str) -> String 返回 你好, name!
  - 1 个单元测试覆盖 greet
  - 命名遵守祖孙三层语义判据（阁=方法、园=实现，核心名不同）
  - 无英文目录（除白名单 SQLite/P0-P3）
  - 一键全验 13/13 全绿
  - Cargo.toml lib 名 gongju_hello_fu
请输出完整 crate 文件树结构 + 每个文件的完整 Rust 代码。
格式：
=== FILE: 路径 ===
代码内容";

fn exec(cmd: &str, args: &[&str]) -> std::process::Output {
    Command::new(cmd).args(args).output().expect("命令执行失败")
}

fn main() {
    println!("=== Round 10 端到端自开发测试 ===");
    println!("任务: {}\n", 任务描述);

    let 密钥 = std::env::var("LLM_API_KEY").expect("需 LLM_API_KEY");
    let 端点 = std::env::var("LLM_BASE_URL")
        .unwrap_or_else(|_| "https://api.minimaxi.com/v1/chat/completions".to_string());
    let 模型 = std::env::var("LLM_MODEL").unwrap_or_else(|_| "MiniMax-M3".to_string());

    println!("端点: {}\n模型: {}\n", 端点, 模型);

    let 请求体 = serde_json::json!({
        "model": 模型,
        "messages": [
            {"role": "system", "content": "你是 Rust 工程师，严格遵守祖孙三层命名语义（阁=方法、园=实现，核心名不同），无英文目录（白名单 SQLite/P0-P3）。"},
            {"role": "user", "content": 任务描述}
        ],
        "temperature": 0.3,
        "max_tokens": 8000,
    });

    println!("调用 minimax LLM...");
    let resp = exec(
        "curl",
        &[
            "-s",
            "-X",
            "POST",
            端点.as_str(),
            "-H",
            "Content-Type: application/json",
            "-H",
            &format!("Authorization: Bearer {}", 密钥),
            "-d",
            &serde_json::to_string(&请求体).unwrap(),
            "--max-time",
            "120",
        ],
    );

    let stdout = String::from_utf8_lossy(&resp.stdout);
    let 响应: serde_json::Value = serde_json::from_str(&stdout).expect("LLM 响应解析失败");
    let 内容 = 响应["choices"][0]["message"]["content"]
        .as_str()
        .expect("LLM 返回无内容");
    let 用量_输入 = 响应["usage"]["prompt_tokens"].as_u64().unwrap_or(0);
    let 用量_输出 = 响应["usage"]["completion_tokens"].as_u64().unwrap_or(0);

    println!(
        "✓ LLM 调用成功，输入 {} tokens，输出 {} tokens",
        用量_输入, 用量_输出
    );

    let 预览: String = 内容.chars().take(500).collect();
    println!("\n=== LLM 响应（前 500 字符）===");
    println!("{}", 预览);
    println!("\n=== 验收 ===");
    println!(
        "1. LLM 调用成功：✓ (输入 {}/输出 {} tokens)",
        用量_输入, 用量_输出
    );
    println!("2. 响应包含 === FILE: ===：{}", 内容.contains("=== FILE:"));
    println!(
        "3. 响应包含 Rust 代码（fn 关键字）：{}",
        内容.contains("fn ")
    );
    println!("\n=== Round 10 端到端测试完成 ===");
    println!("下一步：解析 LLM 响应写入新 crate 目录 + 跑 13 项门禁 + commit");
}
