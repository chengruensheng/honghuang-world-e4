//! 处理实现-园 - HTTP 请求处理（路由分发 + mock JSON 响应）

use super::super::super::{构造_mock_json, 解析_分类, 路由_聊天补全};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub fn 处理_请求(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_millis(200)))?;
    stream.set_write_timeout(Some(Duration::from_millis(200)))?;
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf)?;
    if n == 0 {
        return Ok(());
    }
    let 请求 = String::from_utf8_lossy(&buf[..n]).to_string();
    let 路径 = 请求
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .unwrap_or("")
        .to_string();
    let 响应体 = if 路径 == 路由_聊天补全 {
        let 分类 = 解析_分类(&请求);
        构造_mock_json(&分类)
    } else {
        "404".to_string()
    };
    let http = format!(
        "HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: {}

{}",
        响应体.len(),
        响应体
    );
    stream.write_all(http.as_bytes())?;
    stream.flush()?;
    Ok(())
}
