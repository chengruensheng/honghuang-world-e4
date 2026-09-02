//! 启动-实现-园 - 门户 HTTP 服务启动（绑定端口 + 接受循环）

use super::super::super::门户_路由_阁::路由_分发_园::处理_连接;
use std::net::TcpListener;
use std::thread;

/// 门户 HTTP 服务句柄
pub struct 门户服务 {
    /// 实际监听端口
    pub 端口: u16,
    pub(crate) 句柄: Option<thread::JoinHandle<()>>,
}

/// 启动门户服务：绑定端口并后台循环接受连接
pub fn 启动门户(请求端口: u16, 记忆库路径: String) -> Result<门户服务, String> {
    let 监听器 = TcpListener::bind(("127.0.0.1", 请求端口)).map_err(|错| 错.to_string())?;
    let 实际端口 = 监听器.local_addr().map_err(|错| 错.to_string())?.port();
    监听器.set_nonblocking(true).ok();
    let 句柄 = thread::spawn(move || loop {
        match 监听器.accept() {
            Ok((流, _)) => {
                let _ = 处理_连接(流, &记忆库路径);
            }
            Err(_) => {
                thread::sleep(std::time::Duration::from_millis(40));
            }
        }
    });
    Ok(门户服务 {
        端口: 实际端口,
        句柄: Some(句柄),
    })
}

impl 门户服务 {
    /// 门户首页地址
    pub fn 首页地址(&self) -> String {
        format!("http://127.0.0.1:{}", self.端口)
    }

    /// 等待服务线程结束（阻塞）
    pub fn 等到结束(self) {
        if let Some(句柄) = self.句柄 {
            let _ = 句柄.join();
        }
    }
}
