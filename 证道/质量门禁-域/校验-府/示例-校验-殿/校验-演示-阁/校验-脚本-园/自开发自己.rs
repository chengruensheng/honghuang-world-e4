//! 自己开发自己（v4 阶段 16+17 贯通实验）
//!
//! 四步端到端：诊断 → LLM → 写盘 → 验证。
//! 决策锚：260828-阶段B后续标准 § 全闭环 4 步贯通
//! falsifiable：单跑一次，通过数从 5/15 提升到 ≥10/15。
//!
//! 用法：`cargo run -p jianyan_gongju --example 自开发自己 -- "<工作空间根>"`
//! 注意：会向 MiniMax-M3 发一次真实 HTTP 请求，需设置 LLM_API_KEY。
//! 跑完即清；不入生产。

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use jianyan_gongju::脚本_校验_殿::*;

const LLM_BASE_URL: &str = "https://api.minimaxi.com/v1/chat/completions";
const LLM_MODEL: &str = "MiniMax-M3";
const TARGET_FILE: &str =
    "证道/质量门禁-域/校验-府/架构-校验-殿/结构-校验-阁/架构-实现-园/架构校验.rs";

// ---------- 工具函数 ----------

fn 必须_log(标题: &str) {
    println!("\n=== {} ===", 标题);
}

/// 加载工作空间根 .env（不依赖 dotenv crate，手工解析 KEY=VALUE）
fn 加载_env(根: &std::path::Path) {
    let p = 根.join(".env");
    if let Ok(s) = std::fs::read_to_string(&p) {
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                let k = k.trim();
                let v = v.trim().trim_matches('"');
                if std::env::var(k).is_err() {
                    std::env::set_var(k, v);
                }
            }
        }
    }
}
fn 通过(msg: &str) {
    println!("  [PASS] {}", msg);
}
fn 失败(msg: &str) {
    println!("  [FAIL] {}", msg);
}

// ---------- 步骤 1：诊断 ----------

fn 步骤1_诊断(根: &Path) -> Vec<(u8, String, String)> {
    必须_log("步骤 1/4：诊断（跑 15 项）");
    let items = 跑全部(根);
    let mut 失败列表 = Vec::new();
    for x in &items {
        let 状态 = if x.结果.是否通过() {
            "PASS".to_string()
        } else if x.结果.是否失败() {
            失败列表.push((x.编号, x.名称.clone(), format!("{:?}", x.结果)));
            "FAIL".to_string()
        } else {
            "WARN".to_string()
        };
        println!("  [{:>2}] {} {}", x.编号, 状态, x.名称);
    }
    let 通过 = items.iter().filter(|x| x.结果.是否通过()).count();
    println!(
        "  → 通过 {} / 失败 {} / 警告 {}",
        通过,
        失败列表.len(),
        items.iter().filter(|x| x.结果.是否警告()).count()
    );
    失败列表
}

// ---------- 步骤 2：LLM ----------

fn 任务_prompt(失败列表: &[(u8, String, String)], 当前源: &str) -> String {
    let mut s = String::new();
    s.push_str("你的任务：把下列失败项全部修掉。你面对的是一个 6 层中文命名 Rust crate。\n");
    s.push_str("**只准修改源文件**，不要动目录名、不要新增 crate、不要删除函数签名、不要动 PowerShell 脚本。\n\n");
    s.push_str(&format!("待修改文件：{}\n\n", TARGET_FILE));
    s.push_str("## 已知 bug 类型\n");
    s.push_str("1. Rust 字面量里出现 `\"传\\\\承\\\\殿\"` —— 在 Rust 里这是两个字面字符而非路径，应改成 `\"传承殿\"`。\n");
    s.push_str("2. 路径出现旧带空格写法 `\"道果树/质量门禁 - 域/门禁 - 府\"` —— 已按 260831 统一为无空格 `\"证道/质量门禁-域/门禁-府\"`，去掉空格。\n");
    s.push_str("3. 排除目录列表不完整（缺 `.arts`、`.codeartsdoer`、`.codebuddy`、`.codegraph`、`.agent-teams`、`.trae` 等）。\n");
    s.push_str("4. `max_depth(8)` 太深扫到缓存目录，与 PowerShell 的 Depth 4 不齐 —— 降到 4。\n");
    s.push_str("5. `检查_府级crate至少2殿` 表里本 crate 行结尾是 `\"工具\"` 而非 `\"府\"`，不影响结论但与命名不一致。\n\n");
    s.push_str("## 失败项（只需关注这些）\n");
    for (n, name, msg) in 失败列表 {
        s.push_str(&format!("- [{}] {}: {}\n", n, name, msg));
    }
    s.push_str("\n## 当前源（你要修改的文件全文，照着改）\n```rust\n");
    s.push_str(当前源);
    s.push_str("\n```\n\n");
    s.push_str("## 输出格式（必须严格遵守，无任何例外）\n");
    s.push_str("回复内容必须是且仅是下列结构，前后不允许任何 prose / 思考 / 注释：\n\n");
    s.push_str(
        "=== FILE: 证道/质量门禁-域/校验-府/架构-校验-殿/结构-校验-阁/架构-实现-园/架构校验.rs ===\n",
    );
    s.push_str("```rust\n<完整文件内容>\n```\n\n");
    s.push_str("禁止：\n");
    s.push_str("- 任何 <think> 或解释\n");
    s.push_str("- 任何 '好的，下面'、'Let me'、'分析如下' 等开场白\n");
    s.push_str("- 多个 FILE 块（只一个）\n");
    s.push_str("- 在 ```rust``` 之外另含任何代码片段\n\n");
    s.push_str("## 铁律\n");
    s.push_str("- 不要新增 crate 依赖。\n");
    s.push_str("- 不要删除现有 pub 函数。\n");
    s.push_str("- 不要改成 snake_case 标识符（中文命名是项目根基）。\n");
    s.push_str("- 只输出那个 === FILE: === 块。");
    s
}

fn 步骤2_llm(失败列表: &[(u8, String, String)], 当前源: &str) -> Result<String, String> {
    必须_log("步骤 2/4：调 MiniMax-M3");
    let prompt = 任务_prompt(失败列表, 当前源);
    let api_key = std::env::var("LLM_API_KEY").map_err(|_| "需 LLM_API_KEY")?;

    // 直接走 ureq（workspace 已有依赖），失败回退 std TCP 时间预算内送出
    let body = serde_json::json!({
        "model": LLM_MODEL,
        "messages": [
            {"role": "system", "content": "你是 Rust 代码修补机器人。不要思考过程，不要解释，直接给 === FILE: === 块。"},
            {"role": "user",   "content": prompt}
        ],
        "temperature": 0.1,
        "max_tokens": 16000
    });
    let body_str = body.to_string();
    let body_file = std::env::temp_dir().join("llm_selfdev_body.json");
    std::fs::write(&body_file, &body_str).map_err(|e| format!("写body: {e}"))?;
    let body_path = body_file.display().to_string().replace('\\', "/");

    println!("  → POST {} ({} bytes prompt)", LLM_BASE_URL, prompt.len());
    let out = Command::new("curl")
        .args([
            "-sS",
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
            &format!("@{}", body_path),
        ])
        .output()
        .map_err(|e| format!("curl 启动失败: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "curl 退出 {}：{}",
            out.status,
            String::from_utf8_lossy(&out.stdout)
                .chars()
                .take(500)
                .collect::<String>()
        ));
    }
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let j: serde_json::Value =
        serde_json::from_str(&stdout).map_err(|e| format!("解析 JSON 失败: {e}"))?;
    let 内容 = j["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("LLM 响应无 content")?
        .to_string();
    println!("  → 响应 {} 字符", 内容.len());
    Ok(内容)
}

fn 提取_file(响应: &str) -> Result<(String, String), String> {
    // MiniMax-M3 等模型把 <think>...</think> 前缀贴在回复前，需先剥除
    // 同时容忍前面的 prose（"好的，下面是 ..."）
    let resp: String = if let Some(idx) = 响应.find("</think>") {
        响应[idx + "</think>".len()..].to_string()
    } else if let Some(idx) = 响应.find("</thinking>") {
        响应[idx + "</thinking>".len()..].to_string()
    } else if let Some(idx) = 响应.find("<thinking>") {
        // 闭合标签不对称：尝试找后面的正文起点
        响应[idx..].to_string()
    } else {
        响应.to_string()
    };

    let marker = "=== FILE:";
    let pos = resp.find(marker).ok_or_else(|| {
        format!(
            "未找到 === FILE: === 块（响应前 300 字符：{}）",
            resp.chars().take(300).collect::<String>()
        )
    })?;
    let rest = &resp[pos + marker.len()..];
    let header_end = rest.find("===").ok_or("=== 块结束标记缺失")?;
    let 路径 = rest[..header_end].trim().to_string();
    if 路径 != TARGET_FILE {
        return Err(format!(
            "路径不匹配：期望 `{}`，实得 `{}`",
            TARGET_FILE, 路径
        ));
    }
    let after_header = &rest[header_end + 3..];
    let code_start = after_header.find("```rust").ok_or("缺 ```rust 起")?;
    let code_body = &after_header[code_start + 7..];
    // 直接截断到下一个 ``` 或文末（容忍 prose 跟尾）
    let code_end = code_body.find("```").unwrap_or(code_body.len());
    let 代码 = code_body[..code_end].trim().to_string();
    if 代码.is_empty() {
        return Err("代码块为空".to_string());
    }
    Ok((路径, 代码))
}

// ---------- 步骤 3：写盘 ----------

fn 步骤3_写盘(根: &Path, 代码: &str) -> Result<PathBuf, String> {
    必须_log("步骤 3/4：写盘 + 备份");
    let target = 根.join(TARGET_FILE);
    if !target.exists() {
        return Err(format!("目标不存在：{}", target.display()));
    }
    let backup = target.with_extension("rs.bak");
    std::fs::copy(&target, &backup).map_err(|e| format!("备份失败：{e}"))?;
    println!("  → 备份 → {}", backup.display());
    std::fs::write(&target, 代码).map_err(|e| format!("写盘失败：{e}"))?;
    println!("  → 写盘 → {}", target.display());
    println!("  → 写入 {} 字节", 代码.len());
    Ok(target)
}

// ---------- 步骤 4：编译 + 重跑 ----------

fn 步骤4_验证(根: &PathBuf, 初始失败数: usize, 初始通过数: usize) -> i32 {
    必须_log("步骤 4/4：编译 + 重跑 15 项");
    let ok = Command::new("cargo")
        .args(["build", "-p", "jianyan_gongju"])
        .current_dir(根)
        .status();
    match ok {
        Ok(s) if s.success() => 通过("编译通过"),
        Ok(s) => {
            失败(&format!("编译失败 exit={}", s));
            return -1;
        }
        Err(e) => {
            失败(&format!("编译启动失败：{e}"));
            return -1;
        }
    }
    let items = 跑全部(根);
    let 通过 = items.iter().filter(|x| x.结果.是否通过()).count();
    let 失败 = items.iter().filter(|x| x.结果.是否失败()).count();
    println!(
        "  → 通过 {} / 失败 {} / 警告 {}",
        通过,
        失败,
        items.iter().filter(|x| x.结果.是否警告()).count()
    );
    println!(
        "  → delta: 通过 {}{}，失败 {}{}",
        if 通过 > 初始通过数 { "+" } else { "" },
        通过 as i64 - 初始通过数 as i64,
        if 失败 < 初始失败数 { "-" } else { "+" },
        (失败 as i64 - 初始失败数 as i64).abs()
    );
    通过 as i32 - 初始通过数 as i32
}

// ---------- main ----------

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let 根 = args
        .get(1)
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .expect("需要工作空间根");
    println!("工作空间根 = {}", 根.display());
    加载_env(&根);

    // 步骤 1
    let 失败列表 = 步骤1_诊断(&根);
    let 初始通过数 = 15 - 失败列表.len();
    let 初始失败数 = 失败列表.len();

    let _ = Duration::from_secs(0);

    if 失败列表.is_empty() {
        必须_log("已全绿，无需修复");
        return;
    }

    // 读当前源
    let target = 根.join(TARGET_FILE);
    let 当前源 = std::fs::read_to_string(&target).expect("读架构校验.rs");
    println!("\n读了 {} ({} 字节)", target.display(), 当前源.len());

    // 步骤 2
    let resp = match 步骤2_llm(&失败列表, &当前源) {
        Ok(r) => r,
        Err(e) => {
            失败(&format!("LLM 调用失败：{e}"));
            std::process::exit(2);
        }
    };
    println!(
        "\nLLM 响应（前 300 字符）：\n{}\n...",
        resp.chars().take(300).collect::<String>()
    );

    // 解析
    let (路径, 代码) = match 提取_file(&resp) {
        Ok(x) => x,
        Err(e) => {
            失败(&format!("解析 FILE 块失败：{e}"));
            let dump = std::env::temp_dir().join("selfdev_response.txt");
            std::fs::write(&dump, &resp).ok();
            eprintln!("  → 响应已落盘：{}", dump.display());
            std::process::exit(3);
        }
    };
    println!("解析到 file: {} ({} 字节)", 路径, 代码.len());

    // 步骤 3
    if let Err(e) = 步骤3_写盘(&根, &代码) {
        失败(&e);
        std::process::exit(4);
    }

    // 步骤 4
    let _delta = 步骤4_验证(&根, 初始失败数, 初始通过数);
}
