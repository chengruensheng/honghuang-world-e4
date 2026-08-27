//! 内存阁 - 内存存储后端（HashMap）+ 记忆存储/记忆检索 trait 实现
//!
//! 决策锚：260826-2240 传承殿启动 § 记忆模型
//! 关联文档：02-概念/记忆/03-记忆.md + 04-设计/数据模型/01-记忆.md

use std::collections::HashMap;

// 跨殿引用：类型定义在类型定义殿，trait 在存储操作殿（六层返工后改用 crate:: 路径）
use crate::记忆存储_殿::{记忆存储, 记忆检索};
use crate::记忆类型_殿::{
    判定本质校验, 来源, 档位, 档位_允许写入, 范畴, 记忆ID, 记忆条目, 错误, 阶段,
};

// ============================================================================
// 内存存储后端
// ============================================================================

/// 内存存储后端（HashMap）
#[derive(Default)]
pub struct 内存存储 {
    数据: HashMap<记忆ID, 记忆条目>,
}

impl 内存存储 {
    pub fn 新建() -> Self {
        Self::default()
    }

    /// 强制 4 维正交写入（含 decided_by 校验 + 本质判定 + 档位权限）
    pub fn 写入校验(
        &mut self,
        范畴: 范畴,
        阶段: 阶段,
        档位: 档位,
        来源: 来源,
        内容: impl Into<String>,
        摘要: impl Into<String>,
        decided_by: impl Into<String>,
        implements: impl Into<String>,
    ) -> Result<记忆ID, 错误> {
        let 内容s = 内容.into();
        let decided_by_s = decided_by.into();
        if decided_by_s.is_empty() {
            return Err(错误::缺失决策者);
        }
        判定本质校验(&内容s, 范畴, 阶段)?;
        if !档位_允许写入(档位, 来源) {
            return Err(错误::写入权限不足 { 档位, 来源 });
        }
        let id = self.下一个id();
        let 条目 = 记忆条目::新建(
            id,
            范畴,
            阶段,
            档位,
            来源,
            内容s,
            摘要,
            decided_by_s,
            implements,
        );
        self.写(条目)?;
        Ok(记忆ID(id))
    }

    pub fn 下一个id(&self) -> u64 {
        self.数据.keys().map(|k| k.0).max().unwrap_or(0) + 1
    }
}

impl 记忆存储 for 内存存储 {
    fn 读(&self, id: 记忆ID) -> Option<记忆条目> {
        self.数据.get(&id).cloned()
    }
    fn 写(&mut self, 条目: 记忆条目) -> Result<(), 错误> {
        self.数据.insert(条目.id, 条目);
        Ok(())
    }
    fn 删(&mut self, id: 记忆ID) -> Result<(), 错误> {
        self.数据.remove(&id).map(|_| ()).ok_or(错误::不存在(id))
    }
    fn 查_全部(&self) -> Vec<记忆条目> {
        self.数据.values().cloned().collect()
    }
}

impl 记忆检索 for 内存存储 {
    fn 按范畴阶段(&self, 范畴: 范畴, 阶段: 阶段) -> Vec<记忆条目> {
        self.数据
            .values()
            .filter(|e| e.范畴 == 范畴 && e.阶段 == 阶段)
            .cloned()
            .collect()
    }
    fn 按档位(&self, 档位: 档位) -> Vec<记忆条目> {
        self.数据
            .values()
            .filter(|e| e.档位 == 档位)
            .cloned()
            .collect()
    }
    fn 按来源(&self, 来源: 来源) -> Vec<记忆条目> {
        self.数据
            .values()
            .filter(|e| e.来源 == 来源)
            .cloned()
            .collect()
    }
    fn 四维拼装(
        &self, 范畴: 范畴, 阶段: 阶段, 档位: 档位, 来源: 来源
    ) -> Vec<记忆条目> {
        self.数据
            .values()
            .filter(|e| e.范畴 == 范畴 && e.阶段 == 阶段 && e.档位 == 档位 && e.来源 == 来源)
            .cloned()
            .collect()
    }
}
