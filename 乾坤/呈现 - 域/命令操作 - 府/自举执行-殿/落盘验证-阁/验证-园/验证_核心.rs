//! 落盘验证-阁 - 确定性执行器：写文件 + 跑验收命令
//!
//! 治理铁律 1「确定程序是治理操作唯一执行者」落地：
//! LLM 产代码意图，本确定性函数走 gongju_fu 工具（路径白名单 + 命令白名单）落盘并验证，
//! 越界写盘 / 越界命令被白名单拦截，验证结果作为准圣/终裁的真实验收证据。
//! 决策锚：260830 第一版自举规划（阶段 2 流水线接工具循环）。

use gongju_fu::{写文件工具, 工具, 执行命令工具, 调用输入};

/// 自举执行结果：落盘 + 验证的证据（供准圣验收、道祖终裁）
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 自举执行结果 {
    pub 落盘结果: String,
    pub 验证结果: String,
    pub 验证通过: bool,
}

/// 归一化验收命令：自举验收语义 = 编译通过性检查。
///
/// 生产 CLI 自举（shijie 二进制自举自己）时，`cargo build` 会重建运行中的 shijie.exe，
/// 触发 Windows 文件锁 os error 5（拒绝访问）→ 退出码 101 假失败。
/// `cargo check` 类型检查等价（编译期错误全部暴露），但不产二进制、免疫自指锁。
/// 只归一化 `cargo build`；`cargo test`/`cargo fmt`/`cargo clippy`/`cargo check` 原样保留。
pub fn 归一化验收命令(命令: &str) -> String {
    命令.replace("cargo build", "cargo check")
}

/// 自举执行：代码落盘到目标文件（写路径白名单）→ 跑验收命令（命令白名单）
pub fn 自举执行(代码: &str, 目标文件: &str, 验收命令: &str) -> 自举执行结果 {
    // 空代码保护（治理铁律：不写空文件——大罗产出无代码围栏时提取为空，落空文件=破坏既有资产）
    if 代码.trim().is_empty() {
        return 自举执行结果 {
            落盘结果: "FAIL: 大罗产出未提取到代码（无代码围栏），拒绝写空文件".to_string(),
            验证结果: "落盘失败，跳过验证".to_string(),
            验证通过: false,
        };
    }
    // 1. 写文件（gongju_fu 写文件工具，路径白名单拦截 .env/逃逸）
    let 写工具 = 写文件工具::新建();
    let mut 写输入 = 调用输入::default();
    写输入.参数.insert("路径".to_string(), 目标文件.to_string());
    写输入.参数.insert("内容".to_string(), 代码.to_string());
    let 写输出 = 写工具.执行(&写输入);
    let 落盘结果 = 写输出.结果.clone();
    if !写输出.副作用已发生 {
        return 自举执行结果 {
            落盘结果,
            验证结果: "落盘失败，跳过验证".to_string(),
            验证通过: false,
        };
    }
    // 2. 确定性格式规整：cargo fmt --all（LLM 产出格式不可靠，确定性程序规整——铁律 1「确定程序是唯一执行者」；
    //    fmt 失败不阻断流水线：语法错误由验收命令暴露并打回）
    let 执行工具 = 执行命令工具::新建();
    let mut 格式输入 = 调用输入::default();
    格式输入
        .参数
        .insert("命令".to_string(), "cargo fmt --all".to_string());
    let 格式输出 = 执行工具.执行(&格式输入);
    let 格式结果 = format!("格式规整（cargo fmt --all）：{}", 格式输出.结果);
    // 3. 跑验收命令（gongju_fu 执行命令工具，命令白名单只放行 cargo 构建/测试类）
    //    cargo build 归一化为 cargo check：自举自指锁免疫（shijie 自举自己时 build 会锁 exe）
    let 归一化命令 = 归一化验收命令(验收命令);
    let 命令标注 = if 归一化命令 != 验收命令 {
        format!("（归一化：{} → {}）", 验收命令, 归一化命令)
    } else {
        String::new()
    };
    let mut 执输入 = 调用输入::default();
    执输入.参数.insert("命令".to_string(), 归一化命令);
    let 执输出 = 执行工具.执行(&执输入);
    let 验证通过 = 执输出.副作用已发生 && !执输出.结果.starts_with("FAIL");
    自举执行结果 {
        落盘结果,
        验证结果: format!("{}\n验收命令{}{}", 格式结果, 命令标注, 执输出.结果),
        验证通过,
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 落盘成功且验证命令可达() {
        let 临时 = "工具府测试_自举落盘.tmp";
        let r = 自举执行("// 临时代码\n", 临时, "cargo fmt --check");
        // 临时路径非治理资产，落盘应成功
        assert!(!r.落盘结果.contains("FAIL"), "落盘应成功：{}", r.落盘结果);
        // cargo fmt --check 真实执行：通过或失败都返回结果，不 panic
        assert!(!r.验证结果.is_empty());
        std::fs::remove_file(临时).ok();
    }

    #[test]
    fn 空代码不写空文件() {
        let 临时 = "工具府测试_自举空代码.tmp";
        std::fs::remove_file(临时).ok();
        let r = 自举执行("", 临时, "cargo fmt --check");
        assert!(!r.验证通过, "空代码应验证不通过");
        assert!(
            r.落盘结果.contains("FAIL"),
            "空代码应拒绝落盘：{}",
            r.落盘结果
        );
        assert!(!std::path::Path::new(临时).exists(), "空代码不得创建文件");
    }

    #[test]
    fn 治理资产目标被拦截() {
        let r = 自举执行("恶意", ".env", "cargo fmt --check");
        assert!(
            r.落盘结果.contains("FAIL"),
            ".env 应被写路径白名单拦截：{}",
            r.落盘结果
        );
        assert!(!r.验证通过, "落盘失败则验证不通过");
        assert_eq!(r.验证结果, "落盘失败，跳过验证");
    }

    #[test]
    fn 危险验收命令被拦截() {
        let 临时 = "工具府测试_自举危险.tmp";
        let r = 自举执行("// x\n", 临时, "rm -rf .");
        // 写文件可能成功，但验收命令被命令白名单拦截
        assert!(
            r.验证结果.contains("FAIL"),
            "危险命令应被命令白名单拦截：{}",
            r.验证结果
        );
        assert!(!r.验证通过);
        std::fs::remove_file(临时).ok();
    }

    #[test]
    fn 归一化_cargo_build_替换为_check() {
        assert_eq!(归一化验收命令("cargo build"), "cargo check");
        assert_eq!(
            归一化验收命令("cargo build --workspace"),
            "cargo check --workspace"
        );
    }

    #[test]
    fn 归一化_保留test_fmt_clippy() {
        assert_eq!(归一化验收命令("cargo test"), "cargo test");
        assert_eq!(归一化验收命令("cargo fmt --all"), "cargo fmt --all");
        assert_eq!(
            归一化验收命令("cargo clippy --all-targets"),
            "cargo clippy --all-targets"
        );
    }
}
