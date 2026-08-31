//! 内存阁 - 内存存储后端（HashMap）+ 记忆存储/记忆检索 trait 实现
//!
//! 决策锚：260826-2240 传承殿启动 § 记忆模型
//! 关联文档：02-概念/记忆/03-记忆.md + 04-设计/数据模型/01-记忆.md

use std::collections::HashMap;

// 跨殿引用：类型定义在类型定义殿，trait 在存储操作殿（六层返工后改用 crate:: 路径）
use crate::记忆_存储_殿::{记忆存储, 记忆检索};
use crate::记忆_类型_殿::{
    判定本质校验, 总纲, 本质, 来源, 档位, 档位_允许写入, 记忆ID, 记忆条目, 错误, 阶段,
};

// ============================================================================
// 内存存储后端
// ============================================================================

/// 内存存储后端（HashMap）
#[derive(Default)]
pub struct 内存存储 {
    数据: HashMap<记忆ID, 记忆条目>,
    /// 事件流（append-only：序号递增）
    事件: Vec<(i64, String, String, String)>,
    事件序号: i64,
    /// 任务账本（任务标识 → (已交付, 已归档) 两个独立累计标志）
    账本: HashMap<String, (bool, bool)>,
    /// 账本登记顺序（HashMap 不保序，FIFO 债务队列依赖此有序序列）
    账本顺序: Vec<String>,
    /// 降级快照（债务超上限降级归档的待补提炼任务标识，进程重启不丢语义）
    快照: Vec<String>,
}

impl 内存存储 {
    pub fn 新建() -> Self {
        Self::default()
    }

    /// 强制四维正交写入（含 decided_by 校验 + 本质判定 + 档位权限）
    pub fn 写入校验(
        &mut self,
        总纲: 总纲,
        本质: 本质,
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
        判定本质校验(&内容s, 总纲, 本质)?;
        if !档位_允许写入(档位, 来源) {
            return Err(错误::写入权限不足 { 档位, 来源 });
        }
        let id = self.下一个id();
        let 条目 = 记忆条目::新建(
            id,
            总纲,
            本质,
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

    fn 事件流_追加(&mut self, 事件类型: &str, 内容: &str) -> Result<i64, 错误> {
        self.事件序号 += 1;
        let 时间戳 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_else(|_| "0".to_string());
        self.事件.push((
            self.事件序号,
            时间戳,
            事件类型.to_string(),
            内容.to_string(),
        ));
        Ok(self.事件序号)
    }

    fn 事件流_区间(&self, 起: i64, 止: i64) -> Vec<(i64, String, String, String)> {
        self.事件
            .iter()
            .filter(|(n, _, _, _)| *n >= 起 && *n <= 止)
            .cloned()
            .collect()
    }

    fn 账本_登记(&mut self, 任务标识: &str) -> Result<(), 错误> {
        if !self.账本.contains_key(任务标识) {
            self.账本顺序.push(任务标识.to_string());
        }
        self.账本.insert(任务标识.to_string(), (false, false));
        Ok(())
    }

    fn 账本_标记交付(&mut self, 任务标识: &str) -> Result<(), 错误> {
        let 项 = self
            .账本
            .get_mut(任务标识)
            .ok_or_else(|| 错误::账本任务不存在(任务标识.to_string()))?;
        项.0 = true;
        Ok(())
    }

    fn 账本_标记归档(&mut self, 任务标识: &str) -> Result<(), 错误> {
        let 项 = self
            .账本
            .get_mut(任务标识)
            .ok_or_else(|| 错误::账本任务不存在(任务标识.to_string()))?;
        项.1 = true;
        Ok(())
    }

    fn 账本_债务(&self) -> Result<i64, 错误> {
        let 交付 = self.账本.values().filter(|(交, _)| *交).count() as i64;
        let 归档 = self.账本.values().filter(|(_, 归)| *归).count() as i64;
        Ok(交付 - 归档)
    }

    fn 账本_债务队列(&self) -> Result<Vec<String>, 错误> {
        Ok(self
            .账本顺序
            .iter()
            .filter(|标| {
                self.账本
                    .get(*标)
                    .map(|(交, 归)| *交 && !*归)
                    .unwrap_or(false)
            })
            .cloned()
            .collect())
    }

    fn 快照_登记(&mut self, 任务标识: &str) -> Result<(), 错误> {
        if !self.快照.iter().any(|标| 标 == 任务标识) {
            self.快照.push(任务标识.to_string());
        }
        Ok(())
    }

    fn 快照_全部(&self) -> Result<Vec<String>, 错误> {
        Ok(self.快照.clone())
    }

    fn 快照_清除(&mut self, 任务标识: &str) -> Result<(), 错误> {
        self.快照.retain(|标| 标 != 任务标识);
        Ok(())
    }
}

impl 记忆检索 for 内存存储 {
    fn 按格位(&self, 总纲: 总纲, 本质: 本质) -> Vec<记忆条目> {
        self.数据
            .values()
            .filter(|e| e.总纲 == 总纲 && e.本质 == 本质)
            .cloned()
            .collect()
    }
    fn 按阶段(&self, 阶段: 阶段) -> Vec<记忆条目> {
        self.数据
            .values()
            .filter(|e| e.阶段 == 阶段)
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
        &self,
        总纲: 总纲,
        本质: 本质,
        阶段: 阶段,
        档位: 档位,
        来源: 来源,
    ) -> Vec<记忆条目> {
        self.数据
            .values()
            .filter(|e| {
                e.总纲 == 总纲
                    && e.本质 == 本质
                    && e.阶段 == 阶段
                    && e.档位 == 档位
                    && e.来源 == 来源
            })
            .cloned()
            .collect()
    }
}
