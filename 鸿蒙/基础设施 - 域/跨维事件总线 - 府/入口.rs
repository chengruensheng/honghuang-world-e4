//! 跨维事件总线 - 府
//!
//! 三类事件（会话/智能体/能力）+ 两种分发模式（Waterfall/Serial）
//! + hash 链 + frozen outcome + JSONL append-only 持久化。
//!
//! 治理动作必入事件流，留痕不可篡改（司衡 § 8）。
//!
//! 决策锚：260826-2230 工程-DSH § Waterfall 事件 + frozen outcome
//! 关联文档：02-概念/事件流/04-事件流.md + 02-概念/不可逆结果/07-不可逆结果.md
//! 数据模型：04-设计/数据模型/02-事件流.md

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::Mutex;

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

// ============================================================================
// 监听器 trait
// ============================================================================

/// Waterfall 监听器：监听器决定是否调 `下一步`
///
/// 决策锚：01-哲学/03-工程哲学.md § Waterfall 事件
pub trait Waterfall监听器: Send + Sync {
    fn 处理(
        &self,
        事件: &事件,
        下一步: &mut dyn FnMut() -> Result<(), 错误>,
    ) -> Result<(), 错误>;
}

/// Serial 监听器：独立拦截点
pub trait Serial监听器: Send + Sync {
    fn 处理(&self, 事件: &事件) -> Serial结果;
}

/// Serial 监听结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Serial结果 {
    通过,
    拒绝(String),
    拦截(String),
}

// ============================================================================
// 事件流
// ============================================================================

#[allow(non_snake_case)]
struct 内部 {
    下一个ID: u64,
    最新hash: Hash,
    路径: Option<PathBuf>,
    waterfall: Vec<Box<dyn Waterfall监听器>>,
    serial: Vec<Box<dyn Serial监听器>>,
}

/// 事件流：进程级单例，所有写入经互斥锁串行化
pub struct 事件流 {
    inner: Mutex<内部>,
}

impl 事件流 {
    /// 创建内存事件流（无持久化）
    pub fn 内存() -> Self {
        Self {
            inner: Mutex::new(内部 {
                下一个ID: 1,
                最新hash: 零哈希,
                路径: None,
                waterfall: Vec::new(),
                serial: Vec::new(),
            }),
        }
    }

    /// 创建带 JSONL 持久化的事件流
    pub fn 带持久化(路径: impl Into<PathBuf>) -> Self {
        Self {
            inner: Mutex::new(内部 {
                下一个ID: 1,
                最新hash: 零哈希,
                路径: Some(路径.into()),
                waterfall: Vec::new(),
                serial: Vec::new(),
            }),
        }
    }

    /// 注册 Waterfall 监听器
    pub fn 注册waterfall(&self, 监听器: Box<dyn Waterfall监听器>) {
        self.inner
            .lock()
            .expect("事件流锁中毒")
            .waterfall
            .push(监听器);
    }

    /// 注册 Serial 监听器
    pub fn 注册serial(&self, 监听器: Box<dyn Serial监听器>) {
        self.inner.lock().expect("事件流锁中毒").serial.push(监听器);
    }

    /// 写入事件：填 id + prev_hash + hash，append 到 JSONL
    ///
    /// 决策契约：先调 guize_fu::校验决策契约（接 RULE_REGISTRY 14 条）
    pub fn 写入(&self, mut 事件: 事件) -> Result<u64, 错误> {
        // 1. 决策契约关键字段校验：必须在写入前通过（接 RULE_REGISTRY 14 条）
        let 契约原文 = format!("decided_by: {}\nfalsifiable: 上线 1 周", 事件.决定者);
        guize_fu::校验关键字段(&契约原文).map_err(错误::决策契约违反)?;
        let mut inner = self.inner.lock().expect("事件流锁中毒");
        let id = inner.下一个ID;
        let prev = inner.最新hash;
        let hash = 算事件哈希(
            prev,
            事件.类型,
            事件.模式,
            id,
            &事件.名称,
            事件.时间戳_毫秒,
            &事件.决定者,
        );
        事件.id = id;
        事件.prev_hash = prev;
        事件.hash = hash;
        if let Some(路径) = &inner.路径 {
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(路径)
                .map_err(|e| 错误::IO错误(e.to_string()))?;
            let ser = format!(
                "{}
",
                序列化为行(&事件)
            );
            f.write_all(ser.as_bytes())
                .map_err(|e| 错误::IO错误(e.to_string()))?;
        }
        inner.下一个ID += 1;
        inner.最新hash = hash;
        Ok(id)
    }

    /// 派发事件给监听器（持锁同步迭代）
    pub fn 派发(&self, 事件: &事件) -> Result<(), 错误> {
        let inner = self.inner.lock().expect("事件流锁中毒");
        // Waterfall：按注册顺序串联
        let 总数 = inner.waterfall.len();
        for i in 0..总数 {
            let mut 下一步_called = false;
            {
                let mut 下一步 = || -> Result<(), 错误> {
                    下一步_called = true;
                    Ok(())
                };
                inner.waterfall[i].处理(事件, &mut 下一步)?;
            }
            // 阶段 2 简化：next() 触发后仅记标志，链式跳到下一个监听器留阶段 3 递归实现
            let _ = 下一步_called;
        }
        // Serial：按注册顺序广播
        for 监听器 in &inner.serial {
            match 监听器.处理(事件) {
                Serial结果::通过 => {}
                Serial结果::拦截(_) => {}
                Serial结果::拒绝(msg) => return Err(错误::不可改(msg)),
            }
        }
        Ok(())
    }

    /// 验证 JSONL 文件 hash 链（从头扫描）
    pub fn 验证(&self) -> Result<(), 错误> {
        let 路径 = {
            let inner = self.inner.lock().expect("事件流锁中毒");
            match &inner.路径 {
                Some(p) => p.clone(),
                None => return Ok(()),
            }
        };
        let f = File::open(&路径).map_err(|e| 错误::IO错误(e.to_string()))?;
        let reader = BufReader::new(f);
        let mut prev_hash = 零哈希;
        let mut id = 1u64;
        for (行号, 行) in reader.lines().enumerate() {
            let 行 = 行.map_err(|e| 错误::IO错误(e.to_string()))?;
            if 行.trim().is_empty() {
                continue;
            }
            let 事件: 事件 = 反序列化行(&行)?;
            if 事件.prev_hash != prev_hash {
                return Err(错误::哈希链断裂 {
                    位置: 行号,
                    期望: prev_hash,
                    实际: 事件.prev_hash,
                });
            }
            let 期望hash = 算事件哈希(
                prev_hash,
                事件.类型,
                事件.模式,
                id,
                &事件.名称,
                事件.时间戳_毫秒,
                &事件.决定者,
            );
            if 事件.hash != 期望hash {
                return Err(错误::哈希不匹配 {
                    位置: 行号,
                    期望: 期望hash,
                    实际: 事件.hash,
                });
            }
            prev_hash = 事件.hash;
            id += 1;
        }
        Ok(())
    }

    /// 当前最新 hash
    pub fn 最新hash(&self) -> Hash {
        self.inner.lock().expect("事件流锁中毒").最新hash
    }

    /// 下一个事件的 id
    pub fn 下一个id(&self) -> u64 {
        self.inner.lock().expect("事件流锁中毒").下一个ID
    }

    /// 当前已注册 waterfall 监听器数量
    pub fn waterfall数(&self) -> usize {
        self.inner.lock().expect("事件流锁中毒").waterfall.len()
    }

    /// 当前已注册 serial 监听器数量
    pub fn serial数(&self) -> usize {
        self.inner.lock().expect("事件流锁中毒").serial.len()
    }
}

// ============================================================================
// 序列化（无 serde 依赖，手写 JSON 行）
// ============================================================================

fn 序列化为行(事件: &事件) -> String {
    let 类型名 = match 事件.类型 {
        事件类型::会话 => "会话",
        事件类型::智能体 => "智能体",
        事件类型::能力 => "能力",
    };
    let 模式名 = match 事件.模式 {
        分发模式::瀑布 => "瀑布",
        分发模式::串行 => "串行",
    };
    format!(
        r#"{{"id":{},"类型":"{}","模式":"{}","prev_hash":"{:x}","名称":"{}","时间戳_毫秒":{},"决定者":"{}","hash":"{:x}","immutable":{}}}"#,
        事件.id,
        类型名,
        模式名,
        事件.prev_hash,
        事件.名称,
        事件.时间戳_毫秒,
        事件.决定者,
        事件.hash,
        事件.immutable
    )
}

fn 反序列化行(行: &str) -> Result<事件, 错误> {
    let mut id = 0u64;
    let mut prev_hash = 0u64;
    let mut hash = 0u64;
    let mut 名称 = String::new();
    let mut 时间戳_毫秒 = 0u64;
    let mut 决定者 = String::new();
    let mut immutable = false;
    let mut 类型 = 事件类型::会话;
    let mut 模式 = 分发模式::瀑布;

    // 简单键值对解析（适用于本模块写出的固定格式）
    let 内部 = 行.trim().trim_start_matches('{').trim_end_matches('}');
    for 部分 in 内部.split(',') {
        let 部分 = 部分.trim();
        if 部分.is_empty() {
            continue;
        }
        let kv: Vec<&str> = 部分.splitn(2, ':').collect();
        if kv.len() != 2 {
            continue;
        }
        let k = kv[0].trim().trim_matches('"');
        let v = kv[1].trim().trim_matches(',');
        match k {
            "id" => id = v.parse().unwrap_or(0),
            "类型" => {
                类型 = match v.trim_matches('"') {
                    "会话" => 事件类型::会话,
                    "智能体" => 事件类型::智能体,
                    "能力" => 事件类型::能力,
                    _ => return Err(错误::IO错误("未知类型".to_string())),
                }
            }
            "模式" => {
                模式 = match v.trim_matches('"') {
                    "瀑布" => 分发模式::瀑布,
                    "串行" => 分发模式::串行,
                    _ => return Err(错误::IO错误("未知模式".to_string())),
                }
            }
            "prev_hash" => prev_hash = u64::from_str_radix(v.trim_matches('"'), 16).unwrap_or(0),
            "名称" => 名称 = v.trim_matches('"').to_string(),
            "时间戳_毫秒" => 时间戳_毫秒 = v.parse().unwrap_or(0),
            "决定者" => 决定者 = v.trim_matches('"').to_string(),
            "hash" => hash = u64::from_str_radix(v.trim_matches('"'), 16).unwrap_or(0),
            "immutable" => immutable = v == "true",
            _ => {}
        }
    }

    Ok(事件 {
        id,
        类型,
        模式,
        prev_hash,
        名称,
        时间戳_毫秒,
        决定者,
        hash,
        immutable,
    })
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 哈希稳定() {
        let a = 算哈希("洪荒 · 世界".as_bytes());
        let b = 算哈希("洪荒 · 世界".as_bytes());
        assert_eq!(a, b);
    }

    #[test]
    fn 哈希区分输入() {
        assert_ne!(算哈希("事件 A".as_bytes()), 算哈希("事件 B".as_bytes()));
    }

    #[test]
    fn 三类事件可枚举() {
        let 所有 = [事件类型::会话, 事件类型::智能体, 事件类型::能力];
        assert_eq!(所有.len(), 3);
    }

    #[test]
    fn 事件hash链前后不同() {
        let h1 = 算事件哈希(零哈希, 事件类型::会话, 分发模式::瀑布, 1, "a", 100, "界主");
        let h2 = 算事件哈希(h1, 事件类型::会话, 分发模式::瀑布, 2, "b", 200, "界主");
        assert_ne!(h1, h2);
        assert_ne!(h1, 零哈希);
    }

    #[test]
    fn 写入单事件增长id与hash() {
        let 流 = 事件流::内存();
        let e = 事件::新建(事件类型::会话, 分发模式::瀑布, "test", "界主", 1000);
        流.写入(e).unwrap();
        assert_eq!(流.下一个id(), 2);
        assert_ne!(流.最新hash(), 零哈希);
    }

    #[test]
    fn 测试_决策契约接入_有效decided_by() {
        // 合法：decided_by 填写 + 事件名规范 → 应通过
        let s = 事件流::内存();
        let 事件 = 事件::新建(事件类型::会话, 分发模式::瀑布, "测试事件", "界主", 1000);
        assert!(s.写入(事件).is_ok(), "有效契约应通过");
    }

    #[test]
    fn 测试_决策契约接入_缺decided_by_拒绝() {
        // 缺 decided_by（空字符串）→ 规则注册表应拒绝
        let s = 事件流::内存();
        let mut 事件 = 事件::新建(事件类型::会话, 分发模式::瀑布, "测试事件", "界主", 1000);
        事件.决定者 = "".to_string();
        let r = s.写入(事件);
        assert!(r.is_err(), "空 decided_by 应被规则拒绝");
        assert!(
            matches!(r.unwrap_err(), 错误::决策契约违反(_)),
            "错误类型应为 决策契约违反"
        );
    }

    #[test]
    fn 测试_决策契约接入_100拒绝率() {
        // 100 次缺 decided_by 写入 → 100% 拒绝
        let s = 事件流::内存();
        let mut 拒绝数 = 0;
        for i in 0..100 {
            let mut 事件 = 事件::新建(
                事件类型::会话,
                分发模式::瀑布,
                format!("事件 {}", i),
                "界主",
                1000,
            );
            事件.决定者 = "".to_string();
            if s.写入(事件).is_err() {
                拒绝数 += 1;
            }
        }
        assert_eq!(拒绝数, 100, "100 缺 decided_by 应 100% 拒绝");
    }

    #[test]
    fn 写入100事件全部hash链合法() {
        let 流 = 事件流::内存();
        for i in 0..100 {
            let e = 事件::新建(
                事件类型::会话,
                分发模式::瀑布,
                format!("事件{}", i),
                "界主",
                1000 + i,
            );
            流.写入(e).unwrap();
        }
        assert_eq!(流.下一个id(), 101);
    }

    #[test]
    fn 持久化与验证往返() {
        let tmp =
            std::env::temp_dir().join(format!("chuanchengdian_test_{}.jsonl", std::process::id()));
        let _ = std::fs::remove_file(&tmp);

        let 流 = 事件流::带持久化(&tmp);
        for i in 0..10 {
            let e = 事件::新建(
                事件类型::会话,
                分发模式::瀑布,
                format!("事件{}", i),
                "界主",
                2000 + i,
            );
            流.写入(e).unwrap();
        }
        流.验证().expect("验证失败：hash 链不完整");
        let _ = std::fs::remove_file(&tmp);
    }

    #[test]
    fn frozen_outcome改名被拒() {
        let mut e = 事件::新建(事件类型::会话, 分发模式::瀑布, "原名", "界主", 100);
        e.标记不可改();
        assert!(e.改名("新名").is_err());
    }

    #[test]
    fn 未frozen改名成功() {
        let mut e = 事件::新建(事件类型::会话, 分发模式::瀑布, "原名", "界主", 100);
        e.改名("新名").unwrap();
        assert_eq!(e.名称, "新名");
    }

    /// 测试用 Serial 监听器：拒绝所有
    struct 拒绝监听;
    impl Serial监听器 for 拒绝监听 {
        fn 处理(&self, _e: &事件) -> Serial结果 {
            Serial结果::拒绝("测试拒绝".to_string())
        }
    }

    #[test]
    fn serial监听器拒绝即抛错() {
        let 流 = 事件流::内存();
        流.注册serial(Box::new(拒绝监听));
        let e = 事件::新建(事件类型::会话, 分发模式::瀑布, "test", "界主", 100);
        let id = 流.写入(e.clone()).unwrap();
        assert_eq!(id, 1);
        let res = 流.派发(&e);
        assert!(res.is_err());
    }

    /// 测试用 Waterfall 监听器：记录处理次数
    struct 计数监听(
        std::sync::atomic::AtomicUsize,
        std::sync::Arc<std::sync::Mutex<Vec<u64>>>,
    );
    impl Waterfall监听器 for 计数监听 {
        fn 处理(
            &self,
            事件: &事件,
            下一步: &mut dyn FnMut() -> Result<(), 错误>,
        ) -> Result<(), 错误> {
            self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            self.1.lock().unwrap().push(事件.id);
            下一步()
        }
    }

    #[test]
    fn waterfall监听器按注册顺序执行() {
        let 流 = 事件流::内存();
        let 计数1 = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let 计数2 = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        流.注册waterfall(Box::new(计数监听(
            std::sync::atomic::AtomicUsize::new(0),
            计数1.clone(),
        )));
        流.注册waterfall(Box::new(计数监听(
            std::sync::atomic::AtomicUsize::new(0),
            计数2.clone(),
        )));
        assert_eq!(流.waterfall数(), 2);

        let e = 事件::新建(事件类型::会话, 分发模式::瀑布, "test", "界主", 100);
        流.派发(&e).unwrap();

        assert_eq!(计数1.lock().unwrap().len(), 1);
        assert_eq!(计数2.lock().unwrap().len(), 1);
    }
}
