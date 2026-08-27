//! 角色卡册-殿 - 桥接5阁（核心 + 道祖 + 圣人 + 准圣 + 大罗）

#[path = "卡册核心-阁/模块.rs"]
pub mod 卡册核心_阁;
pub use 卡册核心_阁::*;

#[path = "道祖卡-阁/模块.rs"]
pub mod 道祖卡_阁;
pub use 道祖卡_阁::*;

#[path = "圣人卡-阁/模块.rs"]
pub mod 圣人卡_阁;
pub use 圣人卡_阁::*;

#[path = "准圣卡-阁/模块.rs"]
pub mod 准圣卡_阁;
pub use 准圣卡_阁::*;

#[path = "大罗卡-阁/模块.rs"]
pub mod 大罗卡_阁;
pub use 大罗卡_阁::*;
