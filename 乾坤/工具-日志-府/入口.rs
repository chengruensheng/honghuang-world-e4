#![allow(dead_code)]
#![allow(non_snake_case)]

#[path = "文件日志-接口-殿/模块.rs"]
pub mod 文件日志_接口_殿;

#[path = "文件日志-实现-殿/模块.rs"]
pub mod 文件日志_实现_殿;

const 最大字节: u64 = 1_048_576;

pub fn 写日志(路径: &str, 级别: &str, 消息: &str) -> std::io::Result<()> {
    let 标准级别 = 文件日志_接口_殿::写日志_契约_阁::级别_园::核心::解析(级别);
    let 级别名 =
        文件日志_接口_殿::写日志_契约_阁::级别_园::核心::名称(&标准级别);
    let 时间 = 文件日志_实现_殿::写日志_方法_阁::时间戳_园::核心::格式(
        文件日志_实现_殿::写日志_方法_阁::时间戳_园::核心::现在(),
    );
    let 行 = 文件日志_实现_殿::写日志_方法_阁::拼接_园::核心::格式化(
        &时间, 级别名, 消息,
    );
    文件日志_实现_殿::文件_方法_阁::追加_园::核心::追加写入(路径, &行)?;
    文件日志_实现_殿::文件_方法_阁::轮转_实现_园::核心::检查轮转(
        路径,
        最大字节,
    )?;
    Ok(())
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 写日志_追加写入并保留内容() {
        let 临时 = std::env::temp_dir().join("gongju_rizhi_fu_test.log");
        let 路径 = 临时.to_str().unwrap().to_string();
        let 备份 = format!("{}.old", 路径);
        let _ = std::fs::remove_file(&路径);
        let _ = std::fs::remove_file(&备份);

        写日志(&路径, "INFO", "测试消息一").unwrap();
        写日志(&路径, "WARN", "测试消息二").unwrap();

        let 内容 = std::fs::read_to_string(&路径).unwrap();
        assert!(内容.contains("INFO"));
        assert!(内容.contains("WARN"));
        assert!(内容.contains("测试消息一"));
        assert!(内容.contains("测试消息二"));

        let _ = std::fs::remove_file(&路径);
        let _ = std::fs::remove_file(&备份);
    }
}
