//! 实时-府 - v4 阶段 17 mock HTTP server（本地替 LLM API）

#![allow(non_snake_case)]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

pub const 路由_聊天补全: &str = "/v1/chat/completions";

/// 4 分类 LLM 响应（mock）
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 分类响应 {
    道祖,
    圣人,
    准圣,
    大罗,
}

impl 分类响应 {
    pub fn 名称(&self) -> &'static str {
        match self {
            Self::道祖 => "道祖",
            Self::圣人 => "圣人",
            Self::准圣 => "准圣",
            Self::大罗 => "大罗",
        }
    }
    pub fn 内容(&self) -> &'static str {
        match self {
            Self::道祖 => "[mock 道祖] 决策已下，目标对齐",
            Self::圣人 => "[mock 圣人] 设计完成，方案已固化",
            Self::准圣 => "[mock 准圣] 验收通过，质量达标",
            Self::大罗 => "[mock 大罗] 实现完成，可验收",
        }
    }
}

/// 实时-服务
pub struct 实时服务 {
    pub 端口: u16,
    句柄: Option<thread::JoinHandle<()>>,
}

impl 实时服务 {
    pub fn 启动(请求端口: u16) -> Result<Self, 错误> {
        let listener = TcpListener::bind(("127.0.0.1", 请求端口))
            .map_err(|e| 错误::绑定失败(e.to_string()))?;
        let 实际端口 = listener
            .local_addr()
            .map_err(|e| 错误::本地失败(e.to_string()))?
            .port();
        listener.set_nonblocking(true).ok();
        let 句柄 = thread::spawn(move || {
            let 截止 = std::time::Instant::now() + Duration::from_secs(2);
            loop {
                if std::time::Instant::now() > 截止 {
                    break;
                }
                match listener.accept() {
                    Ok((s, _)) => {
                        let _ = 处理_请求(s);
                    }
                    Err(_) => {
                        thread::sleep(Duration::from_millis(50));
                    }
                }
            }
        });
        Ok(Self {
            端口: 实际端口,
            句柄: Some(句柄),
        })
    }

    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}{}", self.端口, 路由_聊天补全)
    }

    pub fn 等到结束(self) {
        if let Some(h) = self.句柄 {
            let _ = h.join();
        }
    }
}

#[derive(Debug)]
pub enum 错误 {
    绑定失败(String),
    本地失败(String),
}

impl std::fmt::Display for 错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::绑定失败(s) => write!(f, "绑定失败：{}", s),
            Self::本地失败(s) => write!(f, "本地失败：{}", s),
        }
    }
}

impl std::error::Error for 错误 {}

fn 处理_请求(mut stream: TcpStream) -> std::io::Result<()> {
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

fn 解析_分类(请求: &str) -> 分类响应 {
    if 请求.contains("道祖") || 请求.contains("daozu") {
        return 分类响应::道祖;
    }
    if 请求.contains("圣人") || 请求.contains("shengren") {
        return 分类响应::圣人;
    }
    if 请求.contains("准圣") || 请求.contains("zhunsheng") {
        return 分类响应::准圣;
    }
    if 请求.contains("大罗") || 请求.contains("dalun") {
        return 分类响应::大罗;
    }
    分类响应::大罗
}

fn 构造_mock_json(分类: &分类响应) -> String {
    // JSON 字符串中转义双引号
    let mut s = String::new();
    s.push_str("{\"id\":\"chatcmpl-mock\",\"object\":\"chat.completion\",\"model\":\"");
    s.push_str(分类.名称());
    s.push_str("\",\"choices\":[{\"index\":0,\"message\":{\"role\":\"assistant\",\"content\":\"");
    s.push_str(分类.内容());
    s.push_str("\"}}]}");
    s
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    #[test]
    fn 测试_分类响应_名称() {
        assert_eq!(分类响应::道祖.名称(), "道祖");
        assert_eq!(分类响应::圣人.名称(), "圣人");
        assert_eq!(分类响应::准圣.名称(), "准圣");
        assert_eq!(分类响应::大罗.名称(), "大罗");
    }

    #[test]
    fn 测试_解析_分类() {
        assert_eq!(解析_分类("道祖"), 分类响应::道祖);
        assert_eq!(解析_分类("圣人"), 分类响应::圣人);
        assert_eq!(解析_分类("准圣"), 分类响应::准圣);
        assert_eq!(解析_分类("大罗"), 分类响应::大罗);
        assert_eq!(解析_分类(""), 分类响应::大罗);
    }

    #[test]
    fn 测试_构造_mock_json() {
        for c in [
            分类响应::道祖,
            分类响应::圣人,
            分类响应::准圣,
            分类响应::大罗,
        ] {
            let s = 构造_mock_json(&c);
            assert!(s.contains("\"choices\""));
            assert!(s.contains(c.名称()));
        }
    }

    #[test]
    fn 测试_url_格式() {
        let s = 实时服务 {
            端口: 8080,
            句柄: None,
        };
        assert_eq!(s.url(), "http://127.0.0.1:8080/v1/chat/completions");
    }

    #[test]
    fn 测试_错误显示() {
        let e = 错误::绑定失败("test".to_string());
        assert!(e.to_string().contains("test"));
    }

    #[test]
    fn 测试_e2e_启动_4分类() {
        let svc = 实时服务::启动(0).expect("启动失败");
        let 端口 = svc.端口;
        // 等 100ms 让线程起来
        std::thread::sleep(Duration::from_millis(100));
        for (query, expected) in [
            ("道祖", "决策已下"),
            ("圣人", "设计完成"),
            ("准圣", "验收通过"),
            ("大罗", "实现完成"),
        ] {
            let mut stream = TcpStream::connect(("127.0.0.1", 端口)).expect("连接失败");
            let req = format!("POST /v1/chat/completions HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: 0\r\n\r\n?q={}", query);
            stream.write_all(req.as_bytes()).expect("写失败");
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).expect("读失败");
            let resp = String::from_utf8_lossy(&buf);
            assert!(
                resp.contains(expected),
                "query={} resp={}",
                query,
                &resp[..200.min(resp.len())]
            );
        }
        svc.等到结束();
    }
}
