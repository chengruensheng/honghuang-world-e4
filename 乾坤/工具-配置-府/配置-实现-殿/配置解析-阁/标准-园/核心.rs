use std::collections::HashMap;

/// 标准 TOML 风格配置解析
/// 支持 `键 = "值"` 格式，自动忽略空行与 # 注释行
pub fn 解析(内容: &str) -> Result<HashMap<String, String>, String> {
    let mut 结果 = HashMap::new();
    for (行号, 行) in 内容.lines().enumerate() {
        let 修剪 = 行.trim();
        if 修剪.is_empty() || 修剪.starts_with('#') {
            continue;
        }
        match 修剪.split_once('=') {
            Some((键, 值)) => {
                let 键 = 键.trim().to_string();
                let 值 = 值.trim().trim_matches('"').to_string();
                if 键.is_empty() {
                    return Err(format!("第 {} 行：缺少键名", 行号 + 1));
                }
                结果.insert(键, 值);
            }
            None => {
                return Err(format!(
                    "第 {} 行：格式错误，无法识别 ' {} '",
                    行号 + 1,
                    修剪
                ));
            }
        }
    }
    Ok(结果)
}
