//! 工具-hello-府 - 演示 crate（祖孙三层语义：阁=方法、园=实现）

pub fn greet(姓名: &str) -> String {
    format!("你好, {}!", 姓名)
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 测试_greet() {
        assert_eq!(greet("洪荒"), "你好, 洪荒!");
    }
}
