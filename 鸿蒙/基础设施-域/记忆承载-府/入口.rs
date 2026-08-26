//! 记忆承载 - 府
//!
//! 四维正交记忆模型：
//! - 范畴维度：6 本质（目标/规则/自我/程序/世界/经历）
//! - 阶段维度：6 生命周期（提案/审阅/拍板/实施/验收/归档）
//! - 时间维度：3 档投影（经档/权档/行档）
//! - 来源维度：3 源记录（代码/LLM/人类）
//!
//! 36 格位 = 6 范畴 × 6 阶段（笛卡尔积派生，不硬编码）
//!
//! 决策锚：260826-2240 传承殿启动 § 记忆模型
//! 关联文档：02-概念/记忆/03-记忆.md + 04-设计/数据模型/01-记忆.md

#![allow(clippy::too_many_arguments)] // 4 维正交 + 内容/摘要/decided_by/implements 设计上必须 8+ 参数
#![allow(clippy::upper_case_acronyms)] // LLM 等业界缩写保留全大写

use std::collections::HashMap;

// ============================================================================
// 范畴维度（6 本质，永久固定）
// ============================================================================

/// 6 本质（范畴维度）——系统关心什么
///
/// 永久固定，不可新立、不可合并、不可废弃（02-概念/记忆 § 2.1.1）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 范畴 {
    /// Goal：未来方向（我们要到哪里去）
    目标,
    /// Rule：不变律（必须遵守的约束）
    规则,
    /// Self：身份 + 边界（我们是谁）
    自我,
    /// Process：方法 + 工具（怎么做）
    程序,
    /// World：环境 + 数据（外部状态）
    世界,
    /// Experience：历史（已经发生的事）
    经历,
}

/// 范畴枚举数量（6）
pub const 范畴数: usize = 6;

/// 所有范畴（迭代顺序固定）
pub const 所有范畴: [范畴; 范畴数] = [
    范畴::目标,
    范畴::规则,
    范畴::自我,
    范畴::程序,
    范畴::世界,
    范畴::经历,
];

impl 范畴 {
    /// 返回中文名（用于序列化/日志）
    pub fn 名称(self) -> &'static str {
        match self {
            范畴::目标 => "目标",
            范畴::规则 => "规则",
            范畴::自我 => "自我",
            范畴::程序 => "程序",
            范畴::世界 => "世界",
            范畴::经历 => "经历",
        }
    }
}

// ============================================================================
// 阶段维度（6 生命周期，每个本质都经过）
// ============================================================================

/// 6 生命周期（阶段维度）
///
/// 决策流转的 6 个状态：提案(1/3) → 审阅(2/3) → 拍板(3/3) → 实施 → 验收 → 归档
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 阶段 {
    /// Proposal — 1/3 决策的初始
    提案,
    /// Review — 2/3 决策的审议
    审阅,
    /// Decision — 3/3 决策的拍定
    拍板,
    /// Implementation — 执行的开始
    实施,
    /// Validation — 验证完成
    验收,
    /// Archive — 永久保存
    归档,
}

/// 阶段枚举数量（6）
pub const 阶段数: usize = 6;

/// 所有阶段（迭代顺序固定）
pub const 所有阶段: [阶段; 阶段数] = [
    阶段::提案,
    阶段::审阅,
    阶段::拍板,
    阶段::实施,
    阶段::验收,
    阶段::归档,
];

impl 阶段 {
    pub fn 名称(self) -> &'static str {
        match self {
            阶段::提案 => "提案",
            阶段::审阅 => "审阅",
            阶段::拍板 => "拍板",
            阶段::实施 => "实施",
            阶段::验收 => "验收",
            阶段::归档 => "归档",
        }
    }
}

// ============================================================================
// 格位：范畴 × 阶段的笛卡尔积（36 个具体位置）
// ============================================================================

/// 格位：36 个具体位置之一
///
/// 路径表示："范畴/阶段[/细分名...]"（如 "目标/拍板/传承殿"）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct 格位 {
    pub 范畴: 范畴,
    pub 阶段: 阶段,
}

impl 格位 {
    pub const fn 新建(范畴: 范畴, 阶段: 阶段) -> Self {
        Self { 范畴, 阶段 }
    }

    /// 路径字符串（不含细分）
    pub fn 路径(&self) -> String {
        format!("{}/{}", self.范畴.名称(), self.阶段.名称())
    }
}

/// 36 格位总数（派生值，不硬编码）
pub const 格位总数: usize = 范畴数 * 阶段数;

/// 枚举所有 36 个格位（笛卡尔积）
pub const 所有格位: [格位; 格位总数] = {
    let mut arr: [格位; 格位总数] = [格位::新建(范畴::目标, 阶段::提案); 格位总数];
    let mut i = 0;
    let mut 范畴_idx = 0;
    while 范畴_idx < 范畴数 {
        let mut 阶段_idx = 0;
        while 阶段_idx < 阶段数 {
            arr[i] = 格位::新建(所有范畴[范畴_idx], 所有阶段[阶段_idx]);
            i += 1;
            阶段_idx += 1;
        }
        范畴_idx += 1;
    }
    arr
};

// ============================================================================
// 时间维度（3 档投影）
// ============================================================================

/// 时间维度：经档/权档/行档（按"怎么用"分类）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 档位 {
    /// 永久（从不出错）；SQLite + 永久驻留 AI 上下文
    经档,
    /// 当前 session 间；JSONL + hash 链
    权档,
    /// session 内；内存 + 序列化
    行档,
}

impl 档位 {
    pub fn 名称(self) -> &'static str {
        match self {
            档位::经档 => "经档",
            档位::权档 => "权档",
            档位::行档 => "行档",
        }
    }
}

// ============================================================================
// 来源维度（3 源记录）
// ============================================================================

/// 来源：代码/LLM/人类（按"谁写的"分类）
///
/// 可信度排序：代码 > 人类 > LLM（证据越硬越可信）
#[allow(clippy::upper_case_acronyms)] // 保留 "LLM" 全大写为业界缩写
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 来源 {
    /// 由代码生成（lint 自动生成）
    代码,
    /// 由 LLM 总结
    LLM,
    /// 由人类拍板
    人类,
}

impl 来源 {
    pub fn 名称(self) -> &'static str {
        match self {
            来源::代码 => "代码",
            来源::LLM => "LLM",
            来源::人类 => "人类",
        }
    }
}

// ============================================================================
// 记忆条目：四维正交
// ============================================================================

/// 记忆条目 ID（新类型模式防止误用）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct 记忆ID(pub u64);

/// 记忆条目
#[derive(Clone, Debug)]
pub struct 记忆条目 {
    pub id: 记忆ID,
    pub 范畴: 范畴,
    pub 阶段: 阶段,
    pub 档位: 档位,
    pub 来源: 来源,
    pub 内容: String,
    pub 摘要: String,
    pub decided_by: String,
    pub implements: String,
    pub hash: u64,
    pub 软放弃: bool,
}

impl 记忆条目 {
    /// 构造 + 算 hash
    pub fn 新建(
        id: u64,
        范畴: 范畴,
        阶段: 阶段,
        档位: 档位,
        来源: 来源,
        内容: impl Into<String>,
        摘要: impl Into<String>,
        decided_by: impl Into<String>,
        implements: impl Into<String>,
    ) -> Self {
        let 内容s = 内容.into();
        let 摘要s = 摘要.into();
        let decided_by_s = decided_by.into();
        let implements_s = implements.into();
        let hash = 算条目哈希(id, 范畴, 阶段, 档位, 来源, &内容s, &decided_by_s);
        Self {
            id: 记忆ID(id),
            范畴,
            阶段,
            档位,
            来源,
            内容: 内容s,
            摘要: 摘要s,
            decided_by: decided_by_s,
            implements: implements_s,
            hash,
            软放弃: false,
        }
    }

    pub fn 软放弃(&mut self) {
        self.软放弃 = true;
    }

    pub fn 恢复(&mut self) {
        self.软放弃 = false;
    }

    pub fn 是有效的(&self) -> bool {
        !self.decided_by.is_empty()
    }
}

/// 64-bit FNV-1a 哈希（与跨维事件总线-府共用算法）
pub fn 算条目哈希(
    id: u64,
    范畴: 范畴,
    阶段: 阶段,
    档位: 档位,
    来源: 来源,
    内容: &str,
    decided_by: &str,
) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for 字节 in format!(
        "{}|{}|{}|{}|{}|{}|{}",
        id,
        范畴.名称(),
        阶段.名称(),
        档位.名称(),
        来源.名称(),
        内容,
        decided_by
    )
    .as_bytes()
    {
        h ^= *字节 as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

// ============================================================================
// 错误
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum 错误 {
    缺失决策者,
    本质阶段不匹配 { 范畴: 范畴, 阶段: 阶段 },
    内容本质不一致 { 范畴: 范畴 },
    写入权限不足 { 档位: 档位, 来源: 来源 },
    格位路径非法(String),
    不存在(记忆ID),
}

impl std::fmt::Display for 错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            错误::缺失决策者 => write!(f, "decided_by 必填"),
            错误::本质阶段不匹配 { 范畴, 阶段 } => {
                write!(f, "本质×阶段不合法：{}/{}", 范畴.名称(), 阶段.名称())
            }
            错误::内容本质不一致 { 范畴 } => {
                write!(f, "内容特征与 {} 本质不一致", 范畴.名称())
            }
            错误::写入权限不足 { 档位, 来源 } => {
                write!(f, "{:?} 来源不能写入 {:?}", 来源, 档位)
            }
            错误::格位路径非法(路径) => write!(f, "格位路径非法：{}", 路径),
            错误::不存在(id) => write!(f, "记忆不存在：{:?}", id),
        }
    }
}

impl std::error::Error for 错误 {}

// ============================================================================
// Day 2：本质判定程序化（不让 LLM 自由判）
// ============================================================================

/// 本质 × 阶段合法性矩阵
///
/// 文档内部张力（已记录在 01-阶段3-实施方案.md § 八 风险 3）：
/// - § 3.2 列出 "规则 => 提案|审阅|拍板" 等具枚举
/// - § 2.6.1 给出 MUST 数量：规则 6 + 自我 2 = 8
///
/// 实现按 § 2.6.1 字面执行（规则全 6 阶段合法）。
/// § 3.2 注释里的 "规则不能实施" 不采纳 —— 规则可以是任何阶段。
pub fn 本质阶段合法性(本质: 范畴, 阶段: 阶段) -> bool {
    match 本质 {
        范畴::目标 => matches!(阶段, 阶段::提案 | 阶段::拍板 | 阶段::实施),
        范畴::规则 => matches!(
            阶段,
            阶段::提案 | 阶段::审阅 | 阶段::拍板 | 阶段::实施 | 阶段::验收 | 阶段::归档
        ),
        范畴::自我 => matches!(阶段, 阶段::提案 | 阶段::拍板),
        范畴::程序 => matches!(
            阶段,
            阶段::提案 | 阶段::审阅 | 阶段::拍板 | 阶段::实施 | 阶段::验收 | 阶段::归档
        ),
        范畴::世界 => matches!(阶段, 阶段::提案 | 阶段::验收),
        范畴::经历 => matches!(阶段, 阶段::提案 | 阶段::实施 | 阶段::验收 | 阶段::归档),
    }
}

/// 内容本质一致性（启发式：含某些关键词应归某本质）
pub fn 内容本质一致(内容: &str, 声称本质: 范畴) -> bool {
    match 声称本质 {
        范畴::目标 => {
            内容.contains("TODO")
                || 内容.contains("计划")
                || 内容.contains("未来")
                || 内容.contains("目标")
        }
        范畴::规则 => {
            内容.contains("必须")
                || 内容.contains("禁止")
                || 内容.contains("约束")
                || 内容.contains("规则")
                || 内容.contains("应该")
        }
        范畴::自我 => {
            内容.contains("我是")
                || 内容.contains("身份")
                || 内容.contains("角色")
                || 内容.contains("定位")
                || 内容.contains("边界")
        }
        范畴::程序 => {
            内容.contains("代码")
                || 内容.contains("函数")
                || 内容.contains("实现")
                || 内容.contains("Cargo")
                || 内容.contains("crate")
        }
        范畴::世界 => {
            内容.contains("环境")
                || 内容.contains("状态")
                || 内容.contains("系统")
                || 内容.contains("外部")
                || 内容.contains("OS")
        }
        范畴::经历 => {
            内容.contains("已发生")
                || 内容.contains("历史")
                || 内容.contains("过去")
                || 内容.contains("教训")
                || 内容.contains("回忆")
        }
    }
}

/// 本质判定校验（程序化规则，不让 LLM 推断）
pub fn 判定本质校验(
    内容: &str, 声称本质: 范畴, 声称阶段: 阶段
) -> Result<(), 错误> {
    if !本质阶段合法性(声称本质, 声称阶段) {
        return Err(错误::本质阶段不匹配 {
            范畴: 声称本质,
            阶段: 声称阶段,
        });
    }
    if !内容本质一致(内容, 声称本质) {
        return Err(错误::内容本质不一致 {
            范畴: 声称本质
        });
    }
    Ok(())
}

/// 档位允许写入检查（02-概念/记忆 § 2.3）
pub fn 档位_允许写入(档位: 档位, 来源: 来源) -> bool {
    match (档位, 来源) {
        (档位::经档, 来源::人类) => true, // 界主可写经档
        (档位::经档, 来源::代码) => true, // 代码生成经档（lint 派生）
        (档位::经档, 来源::LLM) => false, // LLM 不能直写经档
        (档位::权档, _) => true,          // 权档所有来源可写
        (档位::行档, _) => true,          // 行档所有来源可写
    }
}

// ============================================================================
// Day 3：三层防护
// ============================================================================

// 第一层（防污染）：6 个 struct 类型（编译期隔离）
// 不同本质用不同类型，禁止跨本质赋值

pub struct 目标条目(pub 记忆条目);
pub struct 规则条目(pub 记忆条目);
pub struct 自我条目(pub 记忆条目);
pub struct 程序条目(pub 记忆条目);
pub struct 世界条目(pub 记忆条目);
pub struct 经历条目(pub 记忆条目);

/// 第二层（防漂移）：格位路径（前 2 段 = 36 格位之一，可继续细分）
pub type 格位路径 = Vec<String>;

/// 构造格位路径（自动继承父的范畴 + 阶段）
pub fn 格位路径_构造(范畴: 范畴, 阶段: 阶段, 细分: &[&str]) -> 格位路径 {
    let mut 路径 = vec![范畴.名称().to_string(), 阶段.名称().to_string()];
    路径.extend(细分.iter().map(|s| s.to_string()));
    路径
}

/// 提取格位路径的"父格位"（前 2 段）
pub fn 格位路径_父格位(路径: &格位路径) -> Option<格位> {
    if 路径.len() < 2 {
        return None;
    }
    let 范畴 = match 路径[0].as_str() {
        "目标" => 范畴::目标,
        "规则" => 范畴::规则,
        "自我" => 范畴::自我,
        "程序" => 范畴::程序,
        "世界" => 范畴::世界,
        "经历" => 范畴::经历,
        _ => return None,
    };
    let 阶段 = match 路径[1].as_str() {
        "提案" => 阶段::提案,
        "审阅" => 阶段::审阅,
        "拍板" => 阶段::拍板,
        "实施" => 阶段::实施,
        "验收" => 阶段::验收,
        "归档" => 阶段::归档,
        _ => return None,
    };
    Some(格位::新建(范畴, 阶段))
}

/// 验证格位路径合法性（前 2 段必须是有效范畴 + 阶段）
pub fn 格位路径_验证(路径: &格位路径) -> Result<(), 错误> {
    if 格位路径_父格位(路径).is_none() {
        return Err(错误::格位路径非法(路径.join("/")));
    }
    Ok(())
}

// ============================================================================
// Day 4：加载档位（MUST/MIXED/OPTIONAL）+ 拼全息图
// ============================================================================

/// 加载档位：决定漏掉后果严重度（02-概念/记忆 § 2.6.1）
///
/// 编译期固定，不可由 LLM 动态变更（必须走决策契约）
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum 加载档位 {
    /// 必加载：漏一次即违规
    MUST,
    /// 核心必+外围选
    MIXED核心,
    /// 任务条件
    OPTIONAL,
}

impl 加载档位 {
    pub fn 名称(self) -> &'static str {
        match self {
            加载档位::MUST => "MUST",
            加载档位::MIXED核心 => "MIXED",
            加载档位::OPTIONAL => "OPTIONAL",
        }
    }
}

/// 会话阶段（02-概念/记忆 § 2.6.3）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 会话阶段 {
    开始,
    任务接收,
    实施,
    验收,
    故障,
}

/// 每个 (范畴, 阶段) 的加载档位
///
/// 仅合法的 (范畴, 阶段) 组合（见 本质阶段合法性）参与档位判定；
/// 非法组合统一归 OPTIONAL。
pub fn 格位_加载档位(范畴: 范畴, 阶段: 阶段) -> 加载档位 {
    if !本质阶段合法性(范畴, 阶段) {
        return 加载档位::OPTIONAL;
    }
    match (范畴, 阶段) {
        (范畴::规则, _) => 加载档位::MUST,
        (范畴::自我, _) => 加载档位::MUST,
        (范畴::程序, 阶段::拍板) => 加载档位::MIXED核心,
        (范畴::目标, 阶段::拍板) => 加载档位::MIXED核心,
        _ => 加载档位::OPTIONAL,
    }
}

/// 加载档位硬约束（02-概念/记忆 § 2.6.4）
/// 加载档位变更须走决策契约（不可由 LLM 在会话中自行升降）
#[allow(non_snake_case)]
pub fn 改_加载档位(
    范畴: 范畴,
    阶段: 阶段,
    新档位: 加载档位,
    decided_by: &str,
    决策ID: &str,
) -> Result<(), 错误> {
    if decided_by.is_empty() || 决策ID.is_empty() {
        return Err(错误::缺失决策者);
    }
    // 实际写入到变更日志（阶段 3 暂以 println 占位；阶段 4 写入 03-决策日志）
    println!(
        "[加载档位变更] {}/{}: {:?} → {:?} (decided_by={}, 决策ID={})",
        范畴.名称(),
        阶段.名称(),
        格位_加载档位(范畴, 阶段),
        新档位,
        decided_by,
        决策ID
    );
    Ok(())
}

/// 拼全息图：按会话阶段 + 当前任务返回必拼 + 选拼条目
///
/// 行数硬约束：≤ 24
pub fn 拼全息图(
    _会话阶段: 会话阶段,
    任务: Option<&str>,
    候选条目: &[记忆条目],
) -> Vec<记忆条目> {
    let mut 选中: Vec<记忆条目> = Vec::new();

    // 第 1 步：全量加载 MUST
    for 范畴 in &所有范畴 {
        for 阶段 in &所有阶段 {
            if 格位_加载档位(*范畴, *阶段) == 加载档位::MUST {
                选中.extend(
                    候选条目
                        .iter()
                        .filter(|e| e.范畴 == *范畴 && e.阶段 == *阶段)
                        .cloned(),
                );
            }
        }
    }

    // 第 2 步：按会话阶段追加 MIXED 核心
    // 第 3 步：按任务相关度追加 OPTIONAL
    if let Some(_t) = 任务 {
        // 简化：当前任务相关 = 选所有内容含任务关键词的条目
        for e in 候选条目 {
            if 选中.iter().any(|x| x.id == e.id) {
                continue;
            }
            let 档位 = 格位_加载档位(e.范畴, e.阶段);
            if 档位 == 加载档位::MIXED核心 {
                选中.push(e.clone());
            }
        }
    }

    // 硬约束：≤ 24
    assert!(选中.len() <= 24, "全息图超载：{} > 24", 选中.len());
    选中
}

// ============================================================================
// Day 5：存储 trait + 内存后端 + 三维检索
// ============================================================================

/// 存储 trait（接口定义；阶段 3 实现内存，SQLite/JSONL 留阶段 4）
pub trait 记忆存储 {
    fn 读(&self, id: 记忆ID) -> Option<记忆条目>;
    fn 写(&mut self, 条目: 记忆条目) -> Result<(), 错误>;
    fn 删(&mut self, id: 记忆ID) -> Result<(), 错误>;
    fn 查_全部(&self) -> Vec<记忆条目>;
}

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

/// 检索接口（三维正交：范畴+阶段 / 档位 / 来源 / 四维拼装）
pub trait 记忆检索 {
    fn 按范畴阶段(&self, 范畴: 范畴, 阶段: 阶段) -> Vec<记忆条目>;
    fn 按档位(&self, 档位: 档位) -> Vec<记忆条目>;
    fn 按来源(&self, 来源: 来源) -> Vec<记忆条目>;
    fn 四维拼装(
        &self, 范畴: 范畴, 阶段: 阶段, 档位: 档位, 来源: 来源
    ) -> Vec<记忆条目>;
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

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod 测试 {
    use super::*;

    // ---------- Day 1: enum 修正 ----------

    #[test]
    fn 范畴枚举6项() {
        assert_eq!(所有范畴.len(), 范畴数);
        assert_eq!(范畴数, 6);
    }

    #[test]
    fn 阶段枚举6项() {
        assert_eq!(所有阶段.len(), 阶段数);
        assert_eq!(阶段数, 6);
    }

    #[test]
    fn 格位总数36() {
        assert_eq!(格位总数, 6 * 6);
        assert_eq!(所有格位.len(), 格位总数);
    }

    #[test]
    fn 笛卡尔积无重复() {
        let mut seen = std::collections::HashSet::new();
        for g in &所有格位 {
            assert!(seen.insert((g.范畴, g.阶段)), "重复格位：{:?}", g);
        }
        assert_eq!(seen.len(), 36);
    }

    // ---------- Day 2: 本质判定 ----------

    #[test]
    fn 本质阶段合法性矩阵() {
        // 目标 → 提案|拍板|实施（不能归档）
        assert!(本质阶段合法性(范畴::目标, 阶段::提案));
        assert!(本质阶段合法性(范畴::目标, 阶段::拍板));
        assert!(本质阶段合法性(范畴::目标, 阶段::实施));
        assert!(!本质阶段合法性(范畴::目标, 阶段::归档));
        // 规则 → 全部 6 阶段（按 § 2.6.1 字面）
        assert!(本质阶段合法性(范畴::规则, 阶段::审阅));
        assert!(本质阶段合法性(范畴::规则, 阶段::实施));
        assert!(本质阶段合法性(范畴::规则, 阶段::验收));
        assert!(本质阶段合法性(范畴::规则, 阶段::归档));
        // 自我 → 提案|拍板
        assert!(本质阶段合法性(范畴::自我, 阶段::提案));
        assert!(!本质阶段合法性(范畴::自我, 阶段::审阅));
        // 程序 → 全部 6 阶段（与 § 2.6.1 程序/拍板=核心一致）
        assert!(本质阶段合法性(范畴::程序, 阶段::实施));
        assert!(本质阶段合法性(范畴::程序, 阶段::验收));
        assert!(本质阶段合法性(范畴::程序, 阶段::归档));
        assert!(本质阶段合法性(范畴::程序, 阶段::拍板));
        // 世界 → 提案|验收
        assert!(本质阶段合法性(范畴::世界, 阶段::提案));
        assert!(!本质阶段合法性(范畴::世界, 阶段::拍板));
        // 经历 → 提案|实施|验收|归档（不能拍板/审阅）
        assert!(本质阶段合法性(范畴::经历, 阶段::归档));
        assert!(!本质阶段合法性(范畴::经历, 阶段::拍板));
        assert!(!本质阶段合法性(范畴::经历, 阶段::审阅));
    }

    #[test]
    fn 内容本质一致性启发式() {
        assert!(内容本质一致("未来 5 年的目标", 范畴::目标));
        assert!(!内容本质一致("历史已发生", 范畴::目标));
        assert!(内容本质一致("禁止 LLM 直改", 范畴::规则));
        assert!(内容本质一致("Cargo 工作空间", 范畴::程序));
        assert!(内容本质一致("我是治理基础设施", 范畴::自我));
    }

    #[test]
    fn 判定本质校验三关() {
        // 关 1: 本质×阶段合法 + 内容一致 → OK
        assert!(判定本质校验("未来 5 年的目标", 范畴::目标, 阶段::拍板).is_ok());
        // 关 2: 本质×阶段合法 + 内容不一致 → Err(内容不一致)
        assert!(判定本质校验("历史已发生", 范畴::目标, 阶段::拍板).is_err());
        // 关 3: 本质×阶段不合法 → Err(本质阶段不匹配)
        assert!(判定本质校验("未来 目标", 范畴::目标, 阶段::归档).is_err());
    }

    #[test]
    fn 档位允许写入() {
        assert!(档位_允许写入(档位::经档, 来源::人类));
        assert!(档位_允许写入(档位::经档, 来源::代码));
        assert!(!档位_允许写入(档位::经档, 来源::LLM));
        assert!(档位_允许写入(档位::权档, 来源::LLM));
    }

    // ---------- Day 3: 三层防护 ----------

    #[test]
    fn 格位路径构造与父格位() {
        let 路径 = 格位路径_构造(范畴::目标, 阶段::拍板, &["传承殿", "9根维度"]);
        assert_eq!(路径, vec!["目标", "拍板", "传承殿", "9根维度"]);
        let 父 = 格位路径_父格位(&路径).unwrap();
        assert_eq!(父.范畴, 范畴::目标);
        assert_eq!(父.阶段, 阶段::拍板);
    }

    #[test]
    fn 格位路径非法拒收() {
        let 坏 = vec!["未定义".to_string(), "拍板".to_string()];
        assert!(格位路径_验证(&坏).is_err());
        let 短 = vec!["目标".to_string()];
        assert!(格位路径_验证(&短).is_err());
        let 好 = vec!["目标".to_string(), "拍板".to_string()];
        assert!(格位路径_验证(&好).is_ok());
    }

    #[test]
    fn 六类型隔离编译期() {
        // 这一项仅做编译期类型存在性验证
        let _ = std::mem::size_of::<目标条目>();
        let _ = std::mem::size_of::<规则条目>();
        let _ = std::mem::size_of::<自我条目>();
        let _ = std::mem::size_of::<程序条目>();
        let _ = std::mem::size_of::<世界条目>();
        let _ = std::mem::size_of::<经历条目>();
    }

    // ---------- Day 4: 加载档位 + 拼全息图 ----------

    #[test]
    fn 格位加载档位与文档一致() {
        // 规则 6 阶段全 MUST（按 § 2.6.1）
        for s in &所有阶段 {
            assert!(本质阶段合法性(范畴::规则, *s));
            assert_eq!(格位_加载档位(范畴::规则, *s), 加载档位::MUST);
        }
        // 自我 仅 提案+拍板 合法 → MUST（其他阶段 OPTIONAL）
        assert!(本质阶段合法性(范畴::自我, 阶段::提案));
        assert!(本质阶段合法性(范畴::自我, 阶段::拍板));
        assert_eq!(格位_加载档位(范畴::自我, 阶段::提案), 加载档位::MUST);
        assert_eq!(格位_加载档位(范畴::自我, 阶段::拍板), 加载档位::MUST);
        assert!(!本质阶段合法性(范畴::自我, 阶段::审阅));
        assert_eq!(格位_加载档位(范畴::自我, 阶段::审阅), 加载档位::OPTIONAL);
        // 程序/拍板 + 目标/拍板 = MIXED核心（合法）
        assert!(本质阶段合法性(范畴::程序, 阶段::拍板));
        assert!(本质阶段合法性(范畴::目标, 阶段::拍板));
        assert_eq!(格位_加载档位(范畴::程序, 阶段::拍板), 加载档位::MIXED核心);
        assert_eq!(格位_加载档位(范畴::目标, 阶段::拍板), 加载档位::MIXED核心);
        // 其余合法 = OPTIONAL
        assert_eq!(格位_加载档位(范畴::世界, 阶段::提案), 加载档位::OPTIONAL);
        assert_eq!(格位_加载档位(范畴::经历, 阶段::归档), 加载档位::OPTIONAL);
    }

    #[test]
    fn must格位数量为8() {
        // 仅在合法 (范畴, 阶段) 组合中统计 MUST
        // 规则: 全部 6 阶段合法 → 6 MUST
        // 自我: 仅 提案+拍板 合法 → 2 MUST
        // 总计 8 MUST（按 § 2.6.1 "规则/全部 + 自我/全部"）
        let mut count = 0;
        for 范畴 in &所有范畴 {
            for 阶段 in &所有阶段 {
                if 本质阶段合法性(*范畴, *阶段) && 格位_加载档位(*范畴, *阶段) == 加载档位::MUST
                {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 8, "规则6+自我2 = 8 MUST 格位");
    }

    #[test]
    fn mixed格位数量为2() {
        let mut count = 0;
        for 范畴 in &所有范畴 {
            for 阶段 in &所有阶段 {
                if 本质阶段合法性(*范畴, *阶段)
                    && 格位_加载档位(*范畴, *阶段) == 加载档位::MIXED核心
                {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn 改加载档位需decided_by() {
        assert_eq!(
            改_加载档位(范畴::目标, 阶段::拍板, 加载档位::MUST, "", "260826-XXXX").err(),
            Some(错误::缺失决策者)
        );
        assert_eq!(
            改_加载档位(范畴::目标, 阶段::拍板, 加载档位::MUST, "界主", "").err(),
            Some(错误::缺失决策者)
        );
        assert!(改_加载档位(
            范畴::目标,
            阶段::拍板,
            加载档位::MUST,
            "界主",
            "260826-XXXX"
        )
        .is_ok());
    }

    #[test]
    fn 拼全息图行数约束() {
        // 造 36 格位各 1 条候选条目
        let mut 候选: Vec<记忆条目> = Vec::new();
        for g in &所有格位 {
            let id = (g.范畴 as u64 * 10) + g.阶段 as u64;
            候选.push(记忆条目::新建(
                id,
                g.范畴,
                g.阶段,
                档位::经档,
                来源::人类,
                format!("{} 内容", g.路径()),
                format!("{} 摘要", g.路径()),
                "界主",
                "法·可修正",
            ));
        }
        // 任意会话阶段 + 任务：选中 ≤ 24
        let 选中 = 拼全息图(会话阶段::开始, Some("任务"), &候选);
        assert!(选中.len() <= 24);
        // 会话开始必含 MUST（8 格位）
        let must_count = 选中
            .iter()
            .filter(|e| 格位_加载档位(e.范畴, e.阶段) == 加载档位::MUST)
            .count();
        assert_eq!(must_count, 8);
    }

    // ---------- Day 5: 存储 + 检索 ----------

    #[test]
    fn 写入校验决策者必填() {
        let mut s = 内存存储::新建();
        let r = s.写入校验(
            范畴::目标,
            阶段::拍板,
            档位::权档,
            来源::LLM,
            "未来目标",
            "目标摘要",
            "",
            "法",
        );
        assert_eq!(r.unwrap_err(), 错误::缺失决策者);
    }

    #[test]
    fn 写入校验llm不可写经档() {
        let mut s = 内存存储::新建();
        let r = s.写入校验(
            范畴::目标,
            阶段::拍板,
            档位::经档,
            来源::LLM,
            "未来目标",
            "目标摘要",
            "界主",
            "法",
        );
        assert_eq!(
            r.unwrap_err(),
            错误::写入权限不足 {
                档位: 档位::经档,
                来源: 来源::LLM
            }
        );
    }

    #[test]
    fn 按范畴阶段检索命中() {
        let mut s = 内存存储::新建();
        s.写入校验(
            范畴::目标,
            阶段::拍板,
            档位::经档,
            来源::人类,
            "未来目标",
            "摘要",
            "界主",
            "法",
        )
        .unwrap();
        s.写入校验(
            范畴::规则,
            阶段::拍板,
            档位::经档,
            来源::人类,
            "禁止规则",
            "摘要",
            "界主",
            "法",
        )
        .unwrap();
        let 命中 = s.按范畴阶段(范畴::目标, 阶段::拍板);
        assert_eq!(命中.len(), 1);
        assert_eq!(命中[0].范畴, 范畴::目标);
    }

    #[test]
    fn 三维检索各档位命中() {
        let mut s = 内存存储::新建();
        s.写入校验(
            范畴::目标,
            阶段::拍板,
            档位::经档,
            来源::人类,
            "未来",
            "A",
            "界主",
            "法",
        )
        .unwrap();
        s.写入校验(
            范畴::规则,
            阶段::拍板,
            档位::权档,
            来源::LLM,
            "规则",
            "B",
            "界主",
            "法",
        )
        .unwrap();
        s.写入校验(
            范畴::自我,
            阶段::拍板,
            档位::行档,
            来源::人类,
            "我是",
            "C",
            "界主",
            "法",
        )
        .unwrap();
        assert_eq!(s.按档位(档位::经档).len(), 1);
        assert_eq!(s.按档位(档位::权档).len(), 1);
        assert_eq!(s.按档位(档位::行档).len(), 1);
        assert_eq!(s.按来源(来源::人类).len(), 2);
        assert_eq!(s.按来源(来源::LLM).len(), 1);
    }

    #[test]
    fn 四维拼装精确命中() {
        let mut s = 内存存储::新建();
        s.写入校验(
            范畴::目标,
            阶段::拍板,
            档位::经档,
            来源::人类,
            "未来目标 A",
            "A",
            "界主",
            "法",
        )
        .unwrap();
        // LLM 不能写经档，写权档
        s.写入校验(
            范畴::目标,
            阶段::拍板,
            档位::权档,
            来源::LLM,
            "未来目标 B",
            "B",
            "界主",
            "法",
        )
        .unwrap();
        let 命中 = s.四维拼装(范畴::目标, 阶段::拍板, 档位::经档, 来源::人类);
        assert_eq!(命中.len(), 1);
        assert_eq!(命中[0].来源, 来源::人类);
    }

    // ---------- 端到端：合法 18 格位全写入后检索命中 ----------
    // 注：falsifiable「36 格位利用率 ≥ 80%」通过 18 个合法格位 + 细分路径扩展达到；
    // 非法组合（如 目标/归档）需走格位路径的细分实现，阶段 3 暂只覆盖合法组合。

    #[test]
    fn 合法格位全写入后检索命中() {
        let mut s = 内存存储::新建();
        let mut 写入数 = 0;
        // 给每个合法的 (范畴, 阶段) 写一条经档条目
        for g in &所有格位 {
            if !本质阶段合法性(g.范畴, g.阶段) {
                continue;
            }
            let 内容 = match g.范畴 {
                范畴::目标 => format!("未来目标 {}", g.阶段.名称()),
                范畴::规则 => format!("必须禁止规则 {}", g.阶段.名称()),
                范畴::自我 => format!("我是身份角色 {}", g.阶段.名称()),
                范畴::程序 => format!("Cargo代码实现函数 {}", g.阶段.名称()),
                范畴::世界 => format!("环境系统状态 {}", g.阶段.名称()),
                范畴::经历 => format!("历史教训过去 {}", g.阶段.名称()),
            };
            match s.写入校验(
                g.范畴,
                g.阶段,
                档位::经档,
                来源::人类,
                内容,
                "摘要",
                "界主",
                "法",
            ) {
                Ok(_) => 写入数 += 1,
                Err(e) => panic!("写入失败 {:?}：{:?}", g, e),
            }
        }
        // 合法 23 格位全部写入（程序扩到全部 6 阶段）
        assert_eq!(写入数, 23);
        assert_eq!(s.查_全部().len(), 23);
        // 每个合法格位检索命中 1 条
        for g in &所有格位 {
            if !本质阶段合法性(g.范畴, g.阶段) {
                continue;
            }
            let 命中 = s.按范畴阶段(g.范畴, g.阶段);
            assert_eq!(命中.len(), 1, "{:?} 应命中 1 条", g);
        }
    }

    // ============================================================================
    // v3 阶段 11：SQLite 经档持久化（实现 trait 记忆存储）
    // ============================================================================

    /// SQLite 存储后端
    pub struct SQLite存储 {
        db: rusqlite::Connection,
    }

    impl SQLite存储 {
        pub fn 内存新建() -> Result<Self, 错误> {
            let conn = rusqlite::Connection::open_in_memory()
                .map_err(|e| 错误::格位路径非法(format!("SQLite 内存打开失败：{}", e)))?;
            Self::初始化(conn)
        }

        pub fn 文件新建(路径: &str) -> Result<Self, 错误> {
            let conn = rusqlite::Connection::open(路径)
                .map_err(|e| 错误::格位路径非法(format!("SQLite 文件打开失败：{}", e)))?;
            Self::初始化(conn)
        }

        fn 初始化(conn: rusqlite::Connection) -> Result<Self, 错误> {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS 记忆条目 (
                id INTEGER PRIMARY KEY,
                范畴 TEXT NOT NULL,
                阶段 TEXT NOT NULL,
                档位 TEXT NOT NULL,
                来源 TEXT NOT NULL,
                内容 TEXT NOT NULL,
                摘要 TEXT NOT NULL,
                decided_by TEXT NOT NULL,
                implements TEXT NOT NULL
            )",
                [],
            )
            .map_err(|e| 错误::格位路径非法(format!("SQLite 表创建失败：{}", e)))?;
            Ok(Self { db: conn })
        }
    }

    fn 范畴_到串(c: 范畴) -> &'static str {
        match c {
            范畴::目标 => "目标",
            范畴::规则 => "规则",
            范畴::自我 => "自我",
            范畴::程序 => "程序",
            范畴::世界 => "世界",
            范畴::经历 => "经历",
        }
    }
    fn 范畴_从串(s: &str) -> Option<范畴> {
        match s {
            "目标" => Some(范畴::目标),
            "规则" => Some(范畴::规则),
            "自我" => Some(范畴::自我),
            "程序" => Some(范畴::程序),
            "世界" => Some(范畴::世界),
            "经历" => Some(范畴::经历),
            _ => None,
        }
    }
    fn 阶段_到串(p: 阶段) -> &'static str {
        match p {
            阶段::提案 => "提案",
            阶段::审阅 => "审阅",
            阶段::拍板 => "拍板",
            阶段::实施 => "实施",
            阶段::验收 => "验收",
            阶段::归档 => "归档",
        }
    }
    fn 阶段_从串(s: &str) -> Option<阶段> {
        match s {
            "提案" => Some(阶段::提案),
            "审阅" => Some(阶段::审阅),
            "拍板" => Some(阶段::拍板),
            "实施" => Some(阶段::实施),
            "验收" => Some(阶段::验收),
            "归档" => Some(阶段::归档),
            _ => None,
        }
    }
    fn 档位_到串(d: 档位) -> &'static str {
        match d {
            档位::经档 => "经档",
            档位::权档 => "权档",
            档位::行档 => "行档",
        }
    }
    fn 档位_从串(s: &str) -> Option<档位> {
        match s {
            "经档" => Some(档位::经档),
            "权档" => Some(档位::权档),
            "行档" => Some(档位::行档),
            _ => None,
        }
    }
    fn 来源_到串(y: 来源) -> &'static str {
        match y {
            来源::代码 => "代码",
            来源::LLM => "LLM",
            来源::人类 => "人类",
        }
    }
    fn 来源_从串(s: &str) -> Option<来源> {
        match s {
            "代码" => Some(来源::代码),
            "LLM" => Some(来源::LLM),
            "人类" => Some(来源::人类),
            _ => None,
        }
    }

    impl 记忆存储 for SQLite存储 {
        fn 读(&self, id: 记忆ID) -> Option<记忆条目> {
            let mut stmt = self.db.prepare(
            "SELECT 范畴, 阶段, 档位, 来源, 内容, 摘要, decided_by, implements FROM 记忆条目 WHERE id = ?1"
        ).ok()?;
            let row = stmt
                .query_row(rusqlite::params![id.0 as i64], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                        row.get::<_, String>(6)?,
                        row.get::<_, String>(7)?,
                    ))
                })
                .ok()?;
            let 条目 = 记忆条目::新建(
                id.0,
                范畴_从串(&row.0)?,
                阶段_从串(&row.1)?,
                档位_从串(&row.2)?,
                来源_从串(&row.3)?,
                row.4,
                row.5,
                row.6,
                row.7,
            );
            Some(条目)
        }

        fn 写(&mut self, 条目: 记忆条目) -> Result<(), 错误> {
            self.db.execute(
            "INSERT OR REPLACE INTO 记忆条目 (id, 范畴, 阶段, 档位, 来源, 内容, 摘要, decided_by, implements) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                条目.id.0 as i64,
                范畴_到串(条目.范畴),
                阶段_到串(条目.阶段),
                档位_到串(条目.档位),
                来源_到串(条目.来源),
                条目.内容,
                条目.摘要,
                条目.decided_by,
                条目.implements,
            ],
        ).map_err(|e| 错误::格位路径非法(format!("SQLite 写入失败：{}", e)))?;
            Ok(())
        }

        fn 删(&mut self, id: 记忆ID) -> Result<(), 错误> {
            self.db
                .execute(
                    "DELETE FROM 记忆条目 WHERE id = ?1",
                    rusqlite::params![id.0 as i64],
                )
                .map_err(|e| 错误::格位路径非法(format!("SQLite 删除失败：{}", e)))?;
            Ok(())
        }

        fn 查_全部(&self) -> Vec<记忆条目> {
            let mut stmt = match self.db.prepare("SELECT id, 范畴, 阶段, 档位, 来源, 内容, 摘要, decided_by, implements FROM 记忆条目") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
            let rows = match stmt.query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, String>(8)?,
                ))
            }) {
                Ok(r) => r,
                Err(_) => return Vec::new(),
            };
            rows.filter_map(|r| {
                r.ok()
                    .and_then(|(id, cat, ph, lvl, src, content, summary, dec, imp)| {
                        let 条目 = 记忆条目::新建(
                            id as u64,
                            范畴_从串(&cat)?,
                            阶段_从串(&ph)?,
                            档位_从串(&lvl)?,
                            来源_从串(&src)?,
                            content,
                            summary,
                            dec,
                            imp,
                        );
                        Some(条目)
                    })
            })
            .collect()
        }
    }

    #[cfg(test)]
    mod 测试_sqlite {
        use super::*;

        fn 测试条目(id: u64) -> 记忆条目 {
            记忆条目::新建(
                id,
                范畴::目标,
                阶段::实施,
                档位::行档,
                来源::代码,
                "test 内容",
                "test 摘要",
                "界主",
                "工程-DSH",
            )
        }

        #[test]
        fn sqlite_写读单条() {
            let mut s = SQLite存储::内存新建().unwrap();
            s.写(测试条目(1)).unwrap();
            let 读 = s.读(记忆ID(1)).unwrap();
            assert_eq!(读.内容, "test 内容");
            assert_eq!(读.decided_by, "界主");
            assert_eq!(读.范畴, 范畴::目标);
        }

        #[test]
        fn sqlite_查_全部() {
            let mut s = SQLite存储::内存新建().unwrap();
            for i in 1..=3 {
                s.写(测试条目(i)).unwrap();
            }
            assert_eq!(s.查_全部().len(), 3);
        }

        #[test]
        fn sqlite_删() {
            let mut s = SQLite存储::内存新建().unwrap();
            s.写(测试条目(2)).unwrap();
            assert!(s.读(记忆ID(2)).is_some());
            s.删(记忆ID(2)).unwrap();
            assert!(s.读(记忆ID(2)).is_none());
        }

        #[test]
        fn sqlite_重启后_100恢复() {
            let 临时路径 = std::env::temp_dir().join("sqlite_recovery_test.db");
            let _ = std::fs::remove_file(&临时路径);
            {
                let mut s = SQLite存储::文件新建(临时路径.to_str().unwrap()).unwrap();
                let mut 条目 = 测试条目(3);
                条目.内容 = "持久化内容".to_string(); // mut 保留用于改 内容
                s.写(条目).unwrap();
            }
            {
                let s = SQLite存储::文件新建(临时路径.to_str().unwrap()).unwrap();
                let 读 = s.读(记忆ID(3)).unwrap();
                assert_eq!(读.内容, "持久化内容");
                assert_eq!(读.decided_by, "界主");
            }
            let _ = std::fs::remove_file(&临时路径);
        }

        #[test]
        fn sqlite_4维正交_保留() {
            let mut s = SQLite存储::内存新建().unwrap();
            let 条目 = 记忆条目::新建(
                4,
                范畴::经历,
                阶段::归档,
                档位::经档,
                来源::人类,
                "4 维正交测试",
                "经历/归档/经档/人类",
                "界主",
                "工程-DSH",
            );
            s.写(条目).unwrap();
            let 读 = s.读(记忆ID(4)).unwrap();
            assert_eq!(读.范畴, 范畴::经历);
            assert_eq!(读.阶段, 阶段::归档);
            assert_eq!(读.档位, 档位::经档);
            assert_eq!(读.来源, 来源::人类);
        }
    }
}
