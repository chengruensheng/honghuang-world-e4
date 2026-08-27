use std::fs::OpenOptions;
use std::io::Write;

pub fn 追加写入(路径: &str, 内容: &str) -> std::io::Result<()> {
    let mut 文件 = OpenOptions::new().create(true).append(true).open(路径)?;
    文件.write_all(内容.as_bytes())?;
    Ok(())
}
