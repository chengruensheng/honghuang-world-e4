pub enum 级别 {
    调试,
    信息,
    警告,
    错误,
}

pub fn 解析(名称: &str) -> 级别 {
    match 名称 {
        "DEBUG" | "调试" => 级别::调试,
        "WARN" | "警告" => 级别::警告,
        "ERROR" | "错误" => 级别::错误,
        _ => 级别::信息,
    }
}

pub fn 名称(级: &级别) -> &'static str {
    match 级 {
        级别::调试 => "DEBUG",
        级别::信息 => "INFO",
        级别::警告 => "WARN",
        级别::错误 => "ERROR",
    }
}
