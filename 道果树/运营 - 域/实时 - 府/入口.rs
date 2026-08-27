//! 实时-府 - v4 阶段 17 mock HTTP server（本地替 LLM API）

#![allow(non_snake_case)]

#[path = "实时服务-殿/模块.rs"]
pub mod 实时服务_殿;

pub use 实时服务_殿::{
    分类响应, 处理_请求, 实时服务, 构造_mock_json, 解析_分类, 路由_聊天补全, 错误,
};

#[cfg(test)]
mod 测试 {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::time::Duration;

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
