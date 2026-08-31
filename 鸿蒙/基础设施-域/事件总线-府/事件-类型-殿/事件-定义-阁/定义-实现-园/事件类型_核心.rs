//! 事件类型殿 - DSH 三类事件 + 两种分发模式 + 哈希链 + 事件结构 + 错误类型
//!
//! 决策锚：260826-2230 工程-DSH § Waterfall 事件 + frozen outcome
//! 关联文档：02-概念/事件流/04-事件流.md + 02-概念/不可逆结果/07-不可逆结果.md
//! 数据模型：04-设计/数据模型/02-事件流.md

// ============================================================================
// 枚举类型（DSH 三类事件 + 两种分发模式）
// ============================================================================

/// 事件分类（DSH 三类事件）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 事件类型 {
    /// 会话级事件（持久化）
    会话,
    /// 智能体级事件（live）
    智能体,
    /// 能力级事件（policy + adapter）
    能力,
}

/// 分发模式（DSH Waterfall/Serial）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 分发模式 {
    /// 责任链：每个监听器决定是否调 `下一步`
    瀑布,
    /// 串行广播：所有监听器必收，无 next 概念
    串行,
}

// ============================================================================
// 哈希链支持
// ============================================================================

/// 64-bit FNV-1a 哈希
pub type Hash = u64;

/// 链头 hash（第一个事件的 prev_hash）
pub const 零哈希: Hash = 0;

/// 计算字节载荷的 64-bit 哈希（FNV-1a）
pub fn 算哈希(载荷: &[u8]) -> Hash {
    let mut h: Hash = 0xcbf29ce484222325;
    for 字节 in 载荷 {
        h ^= *字节 as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// 计算事件的整体 hash（链式 = prev_hash × 内容）
pub fn 算事件哈希(
    prev_hash: Hash,
    类型: 事件类型,
    模式: 分发模式,
    id: u64,
    名称: &str,
    时间戳_毫秒: u64,
    决定者: &str,
) -> Hash {
    let mut h = prev_hash.wrapping_add(1);
    let key = format!(
        "{:?}|{:?}|{}|{}|{}|{}",
        类型, 模式, id, 名称, 时间戳_毫秒, 决定者
    );
    for 字节 in key.as_bytes() {
        h ^= *字节 as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

// ============================================================================
// 事件结构
// ============================================================================

/// 事件：append-only + hash 链的核心载体
#[derive(Clone, Debug)]
pub struct 事件 {
    pub id: u64,
    pub 类型: 事件类型,
    pub 模式: 分发模式,
    pub prev_hash: Hash,
    pub 名称: String,
    pub 时间戳_毫秒: u64,
    pub 决定者: String,
    pub hash: Hash,
    pub immutable: bool,
}

impl 事件 {
    /// 构造一个新事件（hash 留空，由事件流写入时填）
    pub fn 新建(
        类型: 事件类型,
        模式: 分发模式,
        名称: impl Into<String>,
        决定者: impl Into<String>,
        时间戳_毫秒: u64,
    ) -> Self {
        Self {
            id: 0,
            类型,
            模式,
            prev_hash: 零哈希,
            名称: 名称.into(),
            时间戳_毫秒,
            决定者: 决定者.into(),
            hash: 零哈希,
            immutable: false,
        }
    }

    /// 标记为不可改（frozen outcome）
    pub fn 标记不可改(&mut self) {
        self.immutable = true;
    }

    /// 修改事件名称——若已 frozen 则拒绝
    pub fn 改名(&mut self, 新名称: impl Into<String>) -> Result<(), 错误> {
        if self.immutable {
            return Err(错误::不可改("事件已 frozen，无法改名".to_string()));
        }
        self.名称 = 新名称.into();
        Ok(())
    }
}

// ============================================================================
// 错误类型
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum 错误 {
    哈希链断裂 {
        位置: usize,
        期望: Hash,
        实际: Hash,
    },
    哈希不匹配 {
        位置: usize,
        期望: Hash,
        实际: Hash,
    },
    不可改(String),
    IO错误(String),
    决策契约违反(Vec<String>),
}

impl std::fmt::Display for 错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            错误::哈希链断裂 {
                位置, 期望, 实际
            } => write!(f, "hash 链断裂 @{}：期望 {:x}，实际 {:x}", 位置, 期望, 实际),
            错误::哈希不匹配 {
                位置, 期望, 实际
            } => write!(f, "hash 不匹配 @{}：期望 {:x}，实际 {:x}", 位置, 期望, 实际),
            错误::不可改(msg) => write!(f, "frozen outcome：{}", msg),
            错误::IO错误(msg) => write!(f, "IO 错误：{}", msg),
            错误::决策契约违反(违规列表) => write!(
                f,
                "决策契约违反（{} 条）：{}",
                违规列表.len(),
                违规列表.join("; ")
            ),
        }
    }
}

impl std::error::Error for 错误 {}
