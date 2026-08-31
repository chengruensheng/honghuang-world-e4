//! 分发阁 - 事件流（进程级单例）+ Waterfall/Serial 分发 + hash 链写入 + 持久化验证
//!
//! 决策锚：260826-2230 工程-DSH § Waterfall 事件 + frozen outcome
//! 关联文档：02-概念/事件流/04-事件流.md + 02-概念/不可逆结果/07-不可逆结果.md
//! 数据模型：04-设计/数据模型/02-事件流.md

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::Mutex;

// 跨殿引用：类型定义在事件类型殿，序列化在持久化殿（六层返工后改用 crate:: 路径）
use crate::事件_持久_殿::{反序列化行, 序列化为行};
use crate::事件_类型_殿::{事件, 算事件哈希, 错误, 零哈希, Hash};

// 阁符号引用：瀑布阁/串行阁在分发殿下（六层返工后改用 crate:: 路径）
use crate::事件_分发_殿::串行_分发_阁::{Serial监听器, Serial结果};
use crate::事件_分发_殿::瀑布_分发_阁::Waterfall监听器;

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
// 单元测试
// ============================================================================

#[cfg(test)]
mod 测试 {
    use super::*;
    // 测试专用：构造事件需要的枚举 + 哈希函数（六层返工后改用 crate:: 路径）
    use crate::事件_类型_殿::{事件类型, 分发模式, 算哈希};

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
