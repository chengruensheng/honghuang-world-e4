//! 任务调遣-府 - v4 阶段 18 多 agent 协同（道祖级调度器）

#![allow(non_snake_case)]

// === 殿阁园桥接（#[path] 连接非 ASCII 模块名 + 连字符目录 + 模块.rs 桥接）
#[path = "任务-数据-殿/模块.rs"]
pub mod 任务_数据_殿;
pub use 任务_数据_殿::{任务状态, 任务项};

#[path = "调度-执行-殿/模块.rs"]
pub mod 调度_执行_殿;
pub use 调度_执行_殿::调度器;
