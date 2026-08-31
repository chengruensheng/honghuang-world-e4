//! 服务-实现-园 - 实时 mock HTTP 服务 + 路由常量 + 错误类型

use super::super::super::处理_请求;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

pub const 路由_聊天补全: &str = "/v1/chat/completions";

/// 实时-服务
pub struct 实时服务 {
    pub 端口: u16,
    pub(crate) 句柄: Option<thread::JoinHandle<()>>,
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
