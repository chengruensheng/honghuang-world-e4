use std::fs;

pub fn 检查轮转(路径: &str, 最大字节: u64) -> std::io::Result<()> {
    let 元数据 = fs::metadata(路径)?;
    if 元数据.len() >= 最大字节 {
        let 备份 = format!("{}.old", 路径);
        let _ = fs::remove_file(&备份);
        fs::rename(路径, &备份)?;
    }
    Ok(())
}
