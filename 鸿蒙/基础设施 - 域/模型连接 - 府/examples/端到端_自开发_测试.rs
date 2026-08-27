//! Round 11.5 端到端自开发生成器
//!
//! 决策锚：260827-AI助手自给自足（Round 11.5）
//! 关联文档：26-可用阶段占位-实施方案.md

use serde_json::json;
use std::fs;
use std::process::Command;

const LLM_BASE_URL: &str = "https://api.minimaxi.com/v1/chat/completions";
const LLM_MODEL: &str = "MiniMax-M3";
const TEMPLATE: &str = "在 乾坤/  域下创建 crate {crate名}（lib 名 {lib名}）。严格遵循 6 层结构：府=crate根，殿≥2（如「接口-殿」+「实现-殿」），每殿≥2阁，每阁≥1园，祖孙三层判据（阁=方法契约，园=实现，核心名不同）。\n\nCargo.toml 模板：\n[package]\nname = \"{lib名}\"\nversion = \"0.1.0\"\nedition = \"2021\"\nlicense = \"MIT\"\n\n[lib]\nname = \"{lib名}\"\npath = \"入口.rs\"\n\n入口.rs：pub fn 主接口 + 1 个单元测试。\n\n描述：{描述}\n\n输出格式：=== FILE: 路径 ===\\n代码\n\n无思考，直接输出代码。token < 16000。无英文目录（白名单 SQLite/P0/P1/P2/P3）。";

fn 任务prompt(crate名: &str, lib名: &str, 描述: &str) -> String {
    TEMPLATE
        .replace("{crate名}", crate名)
        .replace("{lib名}", lib名)
        .replace("{描述}", 描述)
}

fn curl_minimax(prompt: &str, max_tokens: u32) -> Result<String, String> {
    let api_key = std::env::var("LLM_API_KEY").map_err(|_| "需 LLM_API_KEY")?;
    let body = json!({"model": LLM_MODEL, "messages": [{"role": "system", "content": "你是 Rust 代码生成器。直接输出代码，不要思考过程。"}, {"role": "user", "content": prompt}], "temperature": 0.1, "max_tokens": max_tokens});
    let body_file = std::env::temp_dir().join("llm_body_115.json");
    fs::write(&body_file, body.to_string()).map_err(|e| format!("写body：{}", e))?;
    let body_path_str = body_file.display().to_string().replace("\\", "/");
    let output = Command::new("curl")
        .args([
            "-s",
            "-X",
            "POST",
            "--max-time",
            "180",
            LLM_BASE_URL,
            "-H",
            "Content-Type: application/json",
            "-H",
            &format!("Authorization: Bearer {}", api_key),
            "--data-binary",
            &format!("@{}", body_path_str),
        ])
        .output()
        .map_err(|e| format!("curl启动：{}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if !output.status.success() {
        return Err(format!(
            "curl退出 {}：{}",
            output.status,
            &stdout[..stdout.len().min(500)]
        ));
    }
    let j: serde_json::Value = serde_json::from_str(&stdout).map_err(|e| format!("JSON：{}", e))?;
    let 内容 = j["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("无 content")?
        .to_string();
    Ok(内容)
}

fn 解析_file(响应: &str) -> Vec<(String, String)> {
    let mut 结果 = vec![];
    let mut 剩余 = 响应;
    while let Some(idx) = 剩余.find("=== FILE: ") {
        剩余 = &剩余[idx + "=== FILE: ".len()..];
        if let Some((path, rest)) = 剩余.split_once(" ===") {
            let path = path.trim().to_string();
            let end = rest.find("=== FILE: ").unwrap_or(rest.len());
            let 代码 = rest[..end].trim().to_string();
            结果.push((path, 代码));
            剩余 = &rest[end..];
        } else {
            break;
        }
    }
    结果
}

fn 写文件(crate路径: &str, 文件: &[(String, String)]) -> std::io::Result<()> {
    let normalized = crate路径.replace("\\", "/");
    for (路径, 内容) in 文件 {
        let full = format!("{}/{}", normalized, 路径);
        if let Some(idx) = full.rfind("/") {
            fs::create_dir_all(&full[..idx])?;
        }
        fs::write(&full, 内容)?;
    }
    Ok(())
}

fn 加_workspace(crate名: &str) -> std::io::Result<()> {
    let cargo_path = "E:/洪荒 - 世界/Cargo.toml";
    let c = fs::read_to_string(cargo_path)?;
    if c.contains(&format!("\"乾坤/{}\"", crate名)) {
        return Ok(());
    }
    let new_c = c.replace(
        "members = [",
        &format!("members = [\\n    \"\"\"乾坤/{}\"\"\",", crate名),
    );
    fs::write(cargo_path, new_c)?;
    Ok(())
}

fn 跑命令(cmd: &str, args: &[&str], cwd: &str) -> bool {
    Command::new(cmd)
        .args(args)
        .current_dir(cwd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn main() {
    println!("=== Round 11.5 端到端自开发生成器 ===");
    let crate名 = "工具-加密-府";
    let lib名 = "gongju_jiami_fu";
    let 描述 = "提供 SHA256 哈希函数 pub fn sha256(输入: &[u8]) -> [u8; 32]（用 sha2 依赖）+ 1 个单元测试覆盖已知输入输出。";
    let prompt = 任务prompt(crate名, lib名, 描述);
    println!("\n[1/4] 调 minimax LLM (max_tokens=16000)...");
    let 响应 = match curl_minimax(&prompt, 16000) {
        Ok(r) => r,
        Err(e) => {
            println!("✗ 失败：{}", e);
            return;
        }
    };
    println!("  ✓ 响应 {} 字符", 响应.len());
    println!("\n[2/4] 解析 === FILE: ===...");
    let 文件 = 解析_file(&响应);
    if 文件.is_empty() {
        println!("✗ 无 FILE 块");
        return;
    }
    println!("  ✓ {} 个文件", 文件.len());
    println!("\n[3/4] 写入 {}...", crate名);
    let crate路径 = format!("E:/洪荒 - 世界/乾坤/{}", crate名);
    if let Err(e) = fs::create_dir_all(&crate路径) {
        println!("✗ mkdir: {}", e);
        return;
    }
    if let Err(e) = 写文件(&crate路径, &文件) {
        println!("✗ write: {}", e);
        return;
    }
    if let Err(e) = 加_workspace(crate名) {
        println!("✗ ws: {}", e);
    }
    println!("\n[4/4] 跑门禁...");
    let root = "E:/洪荒 - 世界";
    let check = 跑命令("cargo", &["check", "-p", lib名], root);
    println!("  cargo check: {}", if check { "✓" } else { "✗" });
    let test = if check {
        跑命令(
            "cargo",
            &["test", "-p", lib名, "--lib", "--", "--test-threads=1"],
            root,
        )
    } else {
        false
    };
    println!("  cargo test: {}", if test { "✓" } else { "✗" });
    let clippy = test
        && 跑命令(
            "cargo",
            &["clippy", "-p", lib名, "--", "-D", "warnings"],
            root,
        );
    println!("  cargo clippy: {}", if clippy { "✓" } else { "✗" });
    println!(
        "\n=== 验收：{} {} ===",
        crate名,
        if check && test && clippy {
            "✓ 全部"
        } else {
            "✗ 部分"
        }
    );
}
