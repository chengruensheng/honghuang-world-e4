//! 评估-府 - v4 阶段 15 性能基准 + 内存回归

#![allow(non_snake_case)]

#[path = "基线-数据-殿/模块.rs"]
pub mod 基线_数据_殿;
pub use 基线_数据_殿::基线值;

#[path = "基准-执行-殿/模块.rs"]
pub mod 基准_执行_殿;
pub use 基准_执行_殿::{内存_使用, 基准_全, 基准器};
