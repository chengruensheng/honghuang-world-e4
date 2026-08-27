//! 工具-配置-府
//! TOML 配置解析工具集

#[path = "配置-接口-殿/模块.rs"]
pub mod 配置_接口_殿;

#[path = "配置-实现-殿/模块.rs"]
pub mod 配置_实现_殿;

pub use 配置_实现_殿::配置解析_阁::标准_园::解析;

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 测试基本解析() {
        let 输入 = "名称 = \"乾坤\"\n版本 = \"0.1.0\"\n作者 = \"测试\"\n";
        let 结果 = 解析(输入).unwrap();
        assert_eq!(结果.get("名称"), Some(&"乾坤".to_string()));
        assert_eq!(结果.get("版本"), Some(&"0.1.0".to_string()));
        assert_eq!(结果.get("作者"), Some(&"测试".to_string()));
        assert_eq!(结果.len(), 3);
    }
}
