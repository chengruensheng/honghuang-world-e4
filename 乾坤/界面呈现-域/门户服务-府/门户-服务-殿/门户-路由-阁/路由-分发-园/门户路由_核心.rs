//! 路由-分发-园 - 请求解析 + 路由分发 + 响应输出

use std::io::{Read, Write};
use std::net::TcpStream;

use crate::数据_查询_殿::{
    二十切面, 仙官名册, 读取_事件流, 读取_任务账本, 读取_总览, 读取_记忆条目,
};
use crate::界面_静态_殿::工作台页面;

/// 处理单条连接：读请求 → 路由 → 写响应
pub fn 处理_连接(mut 流: TcpStream, 记忆库路径: &str) -> std::io::Result<()> {
    流.set_read_timeout(Some(std::time::Duration::from_millis(400)))
        .ok();
    流.set_write_timeout(Some(std::time::Duration::from_millis(400)))
        .ok();
    let mut 缓冲 = [0u8; 16384];
    let 读入 = match 流.read(&mut 缓冲) {
        Ok(量) => 量,
        Err(_) => return Ok(()),
    };
    if 读入 == 0 {
        return Ok(());
    }
    let 请求 = String::from_utf8_lossy(&缓冲[..读入]).to_string();
    let 原始路径 = 请求
        .lines()
        .next()
        .and_then(|行| 行.split_whitespace().nth(1))
        .unwrap_or("/")
        .to_string();
    // 浏览器会把中文路径编码为 %XX，先百分号解码再路由
    let 路径 = 百分解码(&原始路径);
    let (状态, 内容类型, 响应体) = 路由_分发(&路径, 记忆库路径);
    let 响应 = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        状态,
        内容类型,
        响应体.len(),
        响应体
    );
    let _ = 流.write_all(响应.as_bytes());
    let _ = 流.flush();
    Ok(())
}

/// 百分号解码（浏览器把非 ASCII 路径编码为 %XX）
fn 百分解码(输入: &str) -> String {
    let 字节 = 输入.as_bytes();
    let mut 输出 = Vec::with_capacity(字节.len());
    let mut 序号 = 0;
    while 序号 < 字节.len() {
        if 字节[序号] == b'%' && 序号 + 2 < 字节.len() {
            if let Ok(值) = u8::from_str_radix(&输入[序号 + 1..序号 + 3], 16) {
                输出.push(值);
                序号 += 3;
                continue;
            }
        }
        输出.push(字节[序号]);
        序号 += 1;
    }
    String::from_utf8_lossy(&输出).to_string()
}

/// 路由分发：路径 → （状态行，内容类型，响应体）
fn 路由_分发(路径: &str, 记忆库路径: &str) -> (String, String, String) {
    let 纯路径 = 路径.split('?').next().unwrap_or(路径);
    match 纯路径 {
        "/" | "/index.html" | "/工作台.html" => (
            "200 OK".to_string(),
            "text/html; charset=utf-8".to_string(),
            工作台页面().to_string(),
        ),
        "/api/健康" => (
            "200 OK".to_string(),
            "application/json; charset=utf-8".to_string(),
            serde_json::json!({ "状态": "健康", "数据源": 记忆库路径 }).to_string(),
        ),
        "/api/总览" => 检查_查询(读取_总览(记忆库路径)),
        "/api/任务" => 检查_查询(读取_任务账本(记忆库路径)),
        "/api/事件" => 检查_查询(读取_事件流(记忆库路径)),
        "/api/记忆" => 检查_查询(读取_记忆条目(记忆库路径)),
        "/api/仙官" => (
            "200 OK".to_string(),
            "application/json; charset=utf-8".to_string(),
            仙官名册().to_string(),
        ),
        "/api/切面" => (
            "200 OK".to_string(),
            "application/json; charset=utf-8".to_string(),
            二十切面().to_string(),
        ),
        _ => (
            "404 Not Found".to_string(),
            "application/json; charset=utf-8".to_string(),
            serde_json::json!({ "错误": "未找到接口", "路径": 纯路径 }).to_string(),
        ),
    }
}

/// 查询类接口的统一包装：成功 → 200，失败 → 500
fn 检查_查询(结果: Result<serde_json::Value, String>) -> (String, String, String) {
    match 结果 {
        Ok(值) => (
            "200 OK".to_string(),
            "application/json; charset=utf-8".to_string(),
            值.to_string(),
        ),
        Err(错误) => (
            "500 Internal Server Error".to_string(),
            "application/json; charset=utf-8".to_string(),
            serde_json::json!({ "错误": 错误 }).to_string(),
        ),
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 测试_百分解码_中文() {
        assert_eq!(百分解码("/api/%E6%80%BB%E8%A7%88"), "/api/总览");
    }

    #[test]
    fn 测试_百分解码_混合() {
        assert_eq!(百分解码("/api/总览"), "/api/总览");
        assert_eq!(百分解码("/"), "/");
    }

    #[test]
    fn 测试_路由_首页与未知接口() {
        let (状态, _, _) = 路由_分发("/", "x.db");
        assert_eq!(状态, "200 OK");
        let (状态, _, 体) = 路由_分发("/api/不存在", "x.db");
        assert_eq!(状态, "404 Not Found");
        assert!(体.contains("未找到接口"));
    }
}
