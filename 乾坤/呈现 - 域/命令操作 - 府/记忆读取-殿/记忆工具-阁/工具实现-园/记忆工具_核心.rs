//! 记忆读取-殿 · 记忆工具-阁 · 工具实现-园 - 记忆四分类工具（写/读/查/永驻摘要）
//!
//! 决策锚：260827-AI助手自给自足（传承殿/10-地基/25-AI助手自给自足-实施方案.md）
//! falsifiable：写入后读任务记忆命中；查全部记忆含新条目；工具永驻摘要 36 行
//!
//! 迁移：范畴×阶段 → 总纲×本质（格位=总纲×本质，阶段独立决策状态机）

use jiyi_chengzai_fu::{
    总纲, 所有格位, 本质, 来源, 查格位资产定位, 格位, 格位中枢, 档位, 记忆ID, 记忆存储, 记忆条目,
    错误, 阶段, SQLite存储,
};

/// 默认持久记忆库路径（调用方可覆盖；相对当前目录）
pub const 默认记忆库路径: &str = "洪荒记忆库.sq3";

/// 写记忆工具：向 执行×工具 格位写入一条任务记忆（来源 LLM，权档）
pub fn 写入任务记忆(
    记忆库路径: &str, 内容: &str, 摘要: &str
) -> Result<记忆ID, 错误> {
    let 存储 = SQLite存储::文件新建(记忆库路径)?;
    let mut 中枢 = 格位中枢::新建(存储);
    // 幂等：以「摘要」（任务标识）为绑定任务，重复执行覆盖旧块而非新增幻影记录
    中枢.写入_格位_幂等(
        格位 {
            总纲: 总纲::执行,
            本质: 本质::工具,
        },
        阶段::实施,
        内容,
        摘要,
        档位::权档,
        来源::LLM,
        "ai助手",
        "术·工具",
        摘要,
    )
}

/// 写记忆工具（按格位）：向指定 总纲×本质 格位写入一条记忆（阶段独立参数）
///
/// 三层防护复用 格位中枢::写入_格位（总纲×本质归属 / decided_by 必填 / LLM 不写经档）
pub fn 写入_按格位(
    记忆库路径: &str,
    总纲: 总纲,
    本质: 本质,
    阶段: 阶段,
    内容: &str,
    摘要: &str,
) -> Result<记忆ID, 错误> {
    let 存储 = SQLite存储::文件新建(记忆库路径)?;
    let mut 中枢 = 格位中枢::新建(存储);
    中枢.写入_格位(
        格位 { 总纲, 本质 },
        阶段,
        内容,
        摘要,
        档位::权档,
        来源::LLM,
        "ai助手",
        "术·工具",
    )
}

/// 确认格位记忆（盖玉玺）：人类（界主）确认后写入同格位新记录覆盖
///
/// 语义（对齐 D 盘 识海承载-府 玉玺机制）：LLM 生成内容无玉玺——可以执行、能防漂移，
/// 但权威性低（来源可信度 代码3 > 人类2 > LLM1）。人类看过确认正确 → 来源=人类 +
/// decided_by=界主 = 出玉玺；读取按 ID 降序，新记录即链头（读取_格位_分区 最近优先）。
pub fn 确认格位记忆(
    记忆库路径: &str,
    总纲: 总纲,
    本质: 本质,
    阶段: 阶段,
    内容: &str,
    摘要: &str,
) -> Result<记忆ID, 错误> {
    let 存储 = SQLite存储::文件新建(记忆库路径)?;
    let mut 中枢 = 格位中枢::新建(存储);
    中枢.写入_格位(
        格位 { 总纲, 本质 },
        阶段,
        内容,
        摘要,
        档位::权档,
        来源::人类,
        "界主",
        "术·工具",
    )
}

/// 登记世界事实（代码事实流）：向 外在×数据 格位 写入代码测得的客观事实
///
/// 语义（对齐 D 盘 识海承载-府）：外在 总纲只由代码写入——来源可信度 代码(3) 最高，
/// 事实无需怀疑；测试数/套件数/版本号等应落库登记而非人工维护（防漂移）。
/// decided_by=扫描（代码扫描器登记客观事实，无人类决策含义）。
pub fn 登记世界事实(
    记忆库路径: &str,
    阶段: 阶段,
    内容: &str,
    摘要: &str,
) -> Result<记忆ID, 错误> {
    let 存储 = SQLite存储::文件新建(记忆库路径)?;
    let mut 中枢 = 格位中枢::新建(存储);
    中枢.写入_格位(
        格位 {
            总纲: 总纲::外在,
            本质: 本质::数据,
        },
        阶段,
        内容,
        摘要,
        档位::行档,
        来源::代码,
        "扫描",
        "术·工具",
    )
}

/// 格位仓库深挖：精确检索单一格位（总纲×本质）内全部记忆文本（ID 降序，最近优先）
pub fn 读格位仓库(记忆库路径: &str, 总纲: 总纲, 本质: 本质) -> Vec<String> {
    let 存储 = SQLite存储::文件新建(记忆库路径).expect("记忆库打开失败");
    let 中枢 = 格位中枢::新建(存储);
    中枢
        .读取_格位_分区(格位 { 总纲, 本质 })
        .iter()
        .map(|条目| format!("[{}·{}] {}", 条目.总纲.名称(), 条目.本质.名称(), 条目.内容))
        .collect()
}

/// 三档投影读取：时间维度（档位）在读取侧生效——首因 / 近因 / 会话
///
/// 对齐 传承殿 记忆模型（格位=总纲×本质 × 档位 × 来源）：
/// - 首因：每个格位最早的 经档 条目（经典不变，ID 最小）
/// - 近因：每个格位最新的 权档 条目（因时制宜，ID 最大）
/// - 会话：全部 行档 条目（当下行动，按 ID 降序）
///
/// E 盘自主设计（不照搬 D 盘）：档位维度首次落到读取侧。
pub fn 读取_三档投影(记忆库路径: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    let 存储 = SQLite存储::文件新建(记忆库路径).expect("记忆库打开失败");
    let 全部 = 存储.查_全部();
    // 首因：每格位最早的经档（取 id.0 最小）
    let mut 首因: Vec<&记忆条目> = Vec::new();
    for 条目 in &全部 {
        if matches!(条目.档位, 档位::经档) {
            if let Some(已有) = 首因
                .iter_mut()
                .find(|e| e.总纲 == 条目.总纲 && e.本质 == 条目.本质)
            {
                if 条目.id.0 < 已有.id.0 {
                    *已有 = 条目;
                }
            } else {
                首因.push(条目);
            }
        }
    }
    // 近因：每格位最新的权档（取 id.0 最大）
    let mut 近因: Vec<&记忆条目> = Vec::new();
    for 条目 in &全部 {
        if matches!(条目.档位, 档位::权档) {
            if let Some(已有) = 近因
                .iter_mut()
                .find(|e| e.总纲 == 条目.总纲 && e.本质 == 条目.本质)
            {
                if 条目.id.0 > 已有.id.0 {
                    *已有 = 条目;
                }
            } else {
                近因.push(条目);
            }
        }
    }
    // 会话：行档按 ID 降序
    let mut 会话: Vec<&记忆条目> = 全部
        .iter()
        .filter(|e| matches!(e.档位, 档位::行档))
        .collect();
    会话.sort_by_key(|e| std::cmp::Reverse(e.id.0));
    let 格位键 = |e: &&记忆条目| format!("{}·{}", e.总纲.名称(), e.本质.名称());
    let 格式化 = |v: &mut Vec<&记忆条目>| {
        v.sort_by_key(|e| 格位键(e));
        v.iter()
            .map(|条目| format!("[{}] {}", 格位键(条目), 条目.内容))
            .collect()
    };
    let 首因文本 = 格式化(&mut 首因);
    let 近因文本 = 格式化(&mut 近因);
    let 会话文本 = 格式化(&mut 会话);
    (首因文本, 近因文本, 会话文本)
}

/// 格位统计：每格位条目数 + 总条数 + 最稀疏/最密集格位
///
/// 承接 决策契约 260828-格位稀缺原则（36 格位上限 / 进退守恒 / 不弃高换低）：
/// 稀缺护栏的前提是可观测，本工具把每格位条目数可视化，供写入方判断是否失衡。
pub fn 格位统计(记忆库路径: &str) -> Result<Vec<String>, 错误> {
    let 存储 = SQLite存储::文件新建(记忆库路径)?;
    let 全部 = 存储.查_全部();
    let 总数 = 全部.len();
    let mut 行 = Vec::new();
    let mut 清单: Vec<(String, String, usize)> = Vec::new();
    for g in &所有格位 {
        let n = 全部
            .iter()
            .filter(|e| e.总纲 == g.总纲 && e.本质 == g.本质)
            .count();
        清单.push((g.总纲.名称().to_string(), g.本质.名称().to_string(), n));
        行.push(format!("{}·{}={}", g.总纲.名称(), g.本质.名称(), n));
    }
    let 最稀疏 = 清单
        .iter()
        .min_by_key(|(_, _, n)| *n)
        .map(|(c, b, n)| format!("{}·{}（{} 条）", c, b, n))
        .unwrap_or_default();
    let 最密集 = 清单
        .iter()
        .max_by_key(|(_, _, n)| *n)
        .map(|(c, b, n)| format!("{}·{}（{} 条）", c, b, n))
        .unwrap_or_default();
    行.push(format!("总条数：{}", 总数));
    行.push(format!("最稀疏格位：{}", 最稀疏));
    行.push(format!("最密集格位：{}", 最密集));
    Ok(行)
}

/// 治理事件类型 6 类（元三治·治强单一来源）：终裁通过交付/终裁打回/打回重投/打回达上限/玉玺盖印/废止
pub const 治理事件类型: [&str; 6] = [
    "终裁通过交付",
    "终裁打回",
    "打回重投",
    "打回达上限",
    "玉玺盖印",
    "废止",
];

/// 状态报告：36 格位块数 + 账本债务 + 待补提炼快照 + 玉玺块数，一键输出四段状态
///
/// 决策锚：100项任务 任务18 当前状态报告自动生成（持久化真实边界可观测）。
/// falsifiable：空库零 panic（债务=0/待补=[]/玉玺=0）；写入后块数/债务/玉玺数正确变化。
/// 架构：复用 格位统计（36 格位分布）+ 双工流水线（债务/待补）+ 来源过滤（玉玺），正交组合不重复造轮子。
pub fn 状态报告(记忆库路径: &str) -> Result<Vec<String>, 错误> {
    let 格位行 = 格位统计(记忆库路径)?;
    // 玉玺块数：来源=人类（人类直接写入）或 手印非空（界主/终裁盖印）——与读取_格位_玉玺优先 判据一致
    let 存储 = SQLite存储::文件新建(记忆库路径)?;
    let 全部 = 存储.查_全部();
    let 玉玺块数 = 全部
        .iter()
        .filter(|e| {
            e.来源 == 来源::人类
                || e.块元数据
                    .as_ref()
                    .map(|m| !m.手印.is_empty())
                    .unwrap_or(false)
        })
        .count();
    // 债务 + 待补：双工流水线（同一 SQLite 文件独立连接，与格位数据天然共享）
    let 双工 = jiyi_chengzai_fu::双工流水线::新建(SQLite存储::文件新建(记忆库路径)?);
    let 债务 = 双工.债务()?;
    let 待补 = 双工.待补提炼()?;

    let mut 行 = vec!["== 状态报告 ==".to_string()];
    行.extend(格位行);
    行.push(format!("[债务] {}（已交付−已归档）", 债务));
    行.push(format!("[待补提炼] {} 项", 待补.len()));
    for (序, 任务) in 待补.iter().enumerate() {
        行.push(format!("  {}: {}", 序 + 1, 任务));
    }
    行.push(format!("[玉玺块] {} 块（来源=人类或手印非空）", 玉玺块数));
    // 治理事件流：元三治·治强可观测（终裁通过交付/终裁打回/打回重投/打回达上限/玉玺盖印/废止）
    let 治理事件 = 存储
        .事件流_区间(1, i64::MAX)
        .iter()
        .filter(|(_, _, 类型, _)| 治理事件类型.contains(&类型.as_str()))
        .count();
    行.push(format!(
        "[治理事件流] {} 条（终裁通过交付/终裁打回/打回重投/打回达上限/玉玺盖印/废止）",
        治理事件
    ));
    Ok(行)
}

/// 完成度自评：执行收尾时对结果做 1-5 刻度自评并附依据，写入 经历×事件（来源 LLM，权档，无玉玺）
///
/// 吸收 D 盘「完成度刻度」思路的自主设计（不照搬实现）：
/// 1=未达目标 / 2=部分达成 / 3=达成但留尾 / 4=达成无尾巴 / 5=达成+超额。
/// 刻度非法（非 1-5）返回 错误::完成度刻度非法。
pub fn 完成度自评(
    记忆库路径: &str,
    任务标识: &str,
    刻度: u8,
    依据: &str,
) -> Result<记忆ID, 错误> {
    if !(1..=5).contains(&刻度) {
        return Err(错误::完成度刻度非法(刻度));
    }
    let 内容 = format!("完成度 {}/5：{}（来自 [{}]）", 刻度, 依据, 任务标识);
    写入_按格位(
        记忆库路径,
        总纲::经历,
        本质::事件,
        阶段::验收,
        &内容,
        &format!("完成度自评·{}", 任务标识),
    )
}

/// 世界快照：把客观事实（测试数/套件数）登记为 外在×数据 行档（decided_by=扫描）
///
/// 兑现「测试数/套件数/版本号等应落库登记而非人工维护」（防漂移）：
/// 每次全量验收后运行一次，外在×数据 链头即最新数值；重复运行不产生重复链头。
pub fn 世界快照(
    记忆库路径: &str,
    测试数: &str,
    套件数: &str,
) -> Result<(记忆ID, 记忆ID), 错误> {
    let id1 = 登记世界事实(
        记忆库路径,
        阶段::验收,
        &format!("测试数={}", 测试数),
        "世界快照·测试数",
    )?;
    let id2 = 登记世界事实(
        记忆库路径,
        阶段::验收,
        &format!("套件数={}", 套件数),
        "世界快照·套件数",
    )?;
    Ok((id1, id2))
}

/// 任务收尾：完成度自评 + 终点归档 一步到位（无人开发流水线 t4 之后的收尾仪式）
pub fn 任务收尾(
    记忆库路径: &str,
    任务标识: &str,
    执行结果: &str,
    刻度: u8,
) -> Result<Vec<String>, 错误> {
    let mut 日志 = Vec::new();
    let id = 完成度自评(记忆库路径, 任务标识, 刻度, 执行结果)?;
    日志.push(format!("完成度自评：ID {}", id.0));
    let 精炼数 = 终点归档(记忆库路径, 任务标识, 执行结果);
    日志.push(format!("终点归档精炼：{} 条", 精炼数));
    let 事件类型 = "任务收尾";
    let 内容 = format!("完成度 {}/5：{}（{}）", 刻度, 任务标识, 执行结果);
    事件流_记录(记忆库路径, 事件类型, &内容)?;
    日志.push(format!("事件流记录：完成度 {}/5：{}", 刻度, 任务标识));
    Ok(日志)
}

/// 事件流_记录：append-only 时序事实（SQLite 写事务串行互斥）
pub fn 事件流_记录(
    记忆库路径: &str, 事件类型: &str, 内容: &str
) -> Result<i64, 错误> {
    let mut 存储 = SQLite存储::文件新建(记忆库路径)?;
    存储.事件流_追加(事件类型, 内容)
}

/// 事件流_读取：按序号区间读取（含端点），返回格式化行
pub fn 事件流_读取(记忆库路径: &str, 起: i64, 止: i64) -> Result<Vec<String>, 错误> {
    let 存储 = SQLite存储::文件新建(记忆库路径)?;
    let rows = 存储.事件流_区间(起, 止);
    Ok(rows
        .into_iter()
        .map(|(n, t, ty, c)| format!("[{n}] {t} {ty}：{c}"))
        .collect())
}

/// 事件流_全部：读全量事件
pub fn 事件流_全部(记忆库路径: &str) -> Result<Vec<String>, 错误> {
    事件流_读取(记忆库路径, 0, i64::MAX)
}

/// 记会话：工作记忆完整保留（执行轨迹逐行写入 经历×事件，来源 LLM，权档，无玉玺）
pub fn 记会话(记忆库路径: &str, 任务标识: &str, 轨迹: &[String]) -> Vec<记忆ID> {
    let 摘要 = format!("会话记录·{}", 任务标识);
    let mut ids = Vec::new();
    for (i, 行) in 轨迹.iter().enumerate() {
        let 内容 = format!("[{}] {}（任务：{}）", i + 1, 行, 任务标识);
        if let Ok(id) =
            写入_按格位(记忆库路径, 总纲::经历, 本质::事件, 阶段::实施, &内容, &摘要)
        {
            ids.push(id);
        }
    }
    ids
}

/// 查会话：按任务标识读回工作记忆（经历×事件 中含任务标识锚点的轨迹行）
pub fn 查会话(记忆库路径: &str, 任务标识: &str) -> Vec<String> {
    读格位仓库(记忆库路径, 总纲::经历, 本质::事件)
        .into_iter()
        .filter(|s| s.contains(&format!("（任务：{}）", 任务标识)))
        .collect()
}

/// 终点归档：任务结束收尾——读会话轨迹 → 地道精炼（教训/机制/波及）→ 返回精炼条数
pub fn 终点归档(记忆库路径: &str, 任务标识: &str, 执行结果: &str) -> usize {
    let 会话 = 查会话(记忆库路径, 任务标识);
    let mut 结果 = 执行结果.to_string();
    if !会话.is_empty() {
        结果.push_str(&format!("（会话轨迹 {} 行）", 会话.len()));
    }
    地道精炼(记忆库路径, 任务标识, &结果).len()
}

/// 地道精炼：执行结果自动提炼 教训/机制/波及 写入 经历 总纲（来源 LLM，权档，无玉玺）
///
/// 信号词规则（E 盘自主设计，不照搬 D 盘）：
/// - 教训：含 失败/踩坑/修/缺陷/问题 → 经历×教训（阶段归档）
/// - 机制：含 实现/落地/接入/新增 → 经历×事件（阶段实施）
/// - 波及：含 测试/项/数 → 经历×事件（阶段验收）
pub fn 地道精炼(
    记忆库路径: &str, 任务标识: &str, 执行结果: &str
) -> Vec<记忆ID> {
    let 摘要 = format!("地道精炼·{}", 任务标识);
    let 首行 = 执行结果.split(['\n', '，', ',']).next().unwrap_or(执行结果);
    let 提炼 = |前缀: &str| {
        let 截断 = if 首行.chars().count() > 40 {
            let mut s: String = 首行.chars().take(40).collect();
            s.push('…');
            s
        } else {
            首行.to_string()
        };
        format!("{}：{}（来自 [{}]）", 前缀, 截断, 任务标识)
    };
    let mut ids = Vec::new();
    if 执行结果.contains("失败")
        || 执行结果.contains("踩坑")
        || 执行结果.contains("修")
        || 执行结果.contains("缺陷")
        || 执行结果.contains("问题")
    {
        if let Ok(id) = 写入_按格位(
            记忆库路径,
            总纲::经历,
            本质::教训,
            阶段::归档,
            &提炼("教训"),
            &摘要,
        ) {
            ids.push(id);
        }
    }
    if 执行结果.contains("实现")
        || 执行结果.contains("落地")
        || 执行结果.contains("接入")
        || 执行结果.contains("新增")
    {
        if let Ok(id) = 写入_按格位(
            记忆库路径,
            总纲::经历,
            本质::事件,
            阶段::实施,
            &提炼("机制"),
            &摘要,
        ) {
            ids.push(id);
        }
    }
    if 执行结果.contains("测试") || 执行结果.contains("项") || 执行结果.contains("数")
    {
        if let Ok(id) = 写入_按格位(
            记忆库路径,
            总纲::经历,
            本质::事件,
            阶段::验收,
            &提炼("波及"),
            &摘要,
        ) {
            ids.push(id);
        }
    }
    ids
}

/// 播种格位36：36 格位（6 总纲 × 6 本质）模拟内容自主落地（来源 LLM，无玉玺，可执行）
///
/// 无人开发方向（2026-08-28 界主校准）：不依赖人类确认——AI 自主生成内容
/// （玉玺语义：LLM 生成无玉玺、可执行、权威性低；界主日后可 确认格位记忆 盖玉玺）。
/// 空库先触发种子（5 条权威）；已有条目的格位跳过（幂等）；返回本次补齐数。
pub fn 播种格位36(记忆库路径: &str) -> usize {
    // 空库触发种子（权威格位）
    crate::记忆读取_殿::读取方法_阁::读取实现_园::读取任务相关记忆_持久(
        记忆库路径,
        "播种格位36",
    );
    let 存储 = SQLite存储::文件新建(记忆库路径).expect("记忆库打开失败");
    let 既有 = 存储.查_全部();
    let mut 补数 = 0;
    for g in &所有格位 {
        // 36 格位（总纲×本质）全部合法，只跳过已有格位
        let 已有 = 既有.iter().any(|e| e.总纲 == g.总纲 && e.本质 == g.本质);
        if !已有 {
            // 真实资产定位（任务46）：不写模拟占位，写 语义+示例 的真实资产定位
            let 定位 = 查格位资产定位(g.总纲, g.本质);
            let 内容 = format!(
                "{}·{} {}（示例：{}）",
                g.总纲.名称(),
                g.本质.名称(),
                定位.语义,
                定位.示例
            );
            写入_按格位(记忆库路径, g.总纲, g.本质, 阶段::提案, &内容, "播种格位36")
                .expect("播种写入失败");
            补数 += 1;
        }
    }
    补数
}

/// 读记忆工具：任务描述 → 相关记忆文本列表（持久库）
pub fn 读任务记忆(记忆库路径: &str, 任务描述: &str) -> Vec<String> {
    crate::记忆读取_殿::读取方法_阁::读取实现_园::读取任务相关记忆_持久(
        记忆库路径,
        任务描述,
    )
}

/// 查记忆工具：持久库全部记忆文本（[总纲·本质] 内容）
pub fn 查全部记忆(记忆库路径: &str) -> Vec<String> {
    let 存储 = SQLite存储::文件新建(记忆库路径).expect("记忆库打开失败");
    存储
        .查_全部()
        .iter()
        .map(|条目| format!("[{}·{}] {}", 条目.总纲.名称(), 条目.本质.名称(), 条目.内容))
        .collect()
}

/// 永驻摘要工具：持久库 36 行心智模型摘要
pub fn 工具永驻摘要(记忆库路径: &str) -> Vec<String> {
    let 存储 = SQLite存储::文件新建(记忆库路径).expect("记忆库打开失败");
    let 中枢 = 格位中枢::新建(存储);
    中枢.永驻摘要()
}

/// 任务记忆闭环：任务前读相关记忆 → 任务后写 执行×工具 记忆 → 永驻摘要
pub fn 任务记忆闭环(
    记忆库路径: &str,
    任务标识: &str,
    执行结果: &str,
) -> (Vec<String>, Vec<String>) {
    let 读 = 读任务记忆(记忆库路径, 任务标识);
    写入任务记忆(记忆库路径, 执行结果, 任务标识).expect("任务记忆写入失败");
    let 摘 = 工具永驻摘要(记忆库路径);
    (读, 摘)
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
#[allow(non_snake_case)] // 中文函数名 + 标准缩写（API/LLM）刻意保留
mod 测试 {
    use super::*;

    fn 临时库(名: &str) -> String {
        std::env::temp_dir().join(名).to_str().unwrap().to_string()
    }

    #[test]
    fn 写入后读命中() {
        let 库 = 临时库("记忆库_工具读.db");
        let _ = std::fs::remove_file(&库);
        写入任务记忆(&库, "持久化工具记忆", "工具摘要").unwrap();
        let 记忆 = 读任务记忆(&库, "实现 Cargo 测试");
        assert!(记忆.iter().any(|m| m.contains("持久化工具记忆")));
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 查全部包含新条目() {
        let 库 = 临时库("记忆库_工具查.db");
        let _ = std::fs::remove_file(&库);
        写入任务记忆(&库, "可查询条目", "查询摘要").unwrap();
        let 全部 = 查全部记忆(&库);
        assert!(全部.iter().any(|m| m.contains("可查询条目")));
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 工具永驻摘要36行() {
        let 库 = 临时库("记忆库_工具摘.db");
        let _ = std::fs::remove_file(&库);
        写入任务记忆(&库, "摘要条目", "摘要").unwrap();
        let 摘要 = 工具永驻摘要(&库);
        assert_eq!(摘要.len(), 36, "永驻摘要固定 36 行");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 任务记忆闭环写入后摘要含新内容() {
        let 库 = 临时库("记忆库_闭环.db");
        let _ = std::fs::remove_file(&库);
        let (_读, 摘要) = 任务记忆闭环(&库, "实现 Cargo 测试", "闭环执行结果");
        assert_eq!(摘要.len(), 36);
        assert!(摘要.iter().any(|s| s.contains("实现 Cargo 测试")));
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 任务记忆闭环读命中种子记忆() {
        let 库 = 临时库("记忆库_闭环读.db");
        let _ = std::fs::remove_file(&库);
        let (读, _摘要) = 任务记忆闭环(&库, "实现 Cargo 测试", "结果");
        assert!(读.iter().any(|m| m.contains("36 格位闭环 API")));
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 文件库连续写入不覆盖已有记忆() {
        let 库 = 临时库("记忆库_不覆盖.db");
        let _ = std::fs::remove_file(&库);
        let _读 = 读任务记忆(&库, "实现 Cargo 测试");
        let 全部1 = 查全部记忆(&库);
        assert_eq!(全部1.len(), 5, "种子 5 条全部落盘");
        assert!(全部1.iter().any(|m| m.contains("25号 AI 自给自足目标")));
        写入任务记忆(&库, "新条目", "新摘要").unwrap();
        let 全部 = 查全部记忆(&库);
        assert_eq!(全部.len(), 6, "5 种子 + 1 新条目，无覆盖");
        assert!(全部.iter().any(|m| m.contains("25号 AI 自给自足目标")));
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 写入任务记忆_幂等_重复摘要覆盖不新增() {
        let 库 = 临时库("记忆库_幂等.db");
        let _ = std::fs::remove_file(&库);
        写入任务记忆(&库, "第一次内容", "任务X").unwrap();
        写入任务记忆(&库, "第二次内容", "任务X").unwrap();
        let 存储 = SQLite存储::文件新建(&库).unwrap();
        let 全部 = 存储.查_全部();
        let 命中: Vec<_> = 全部
            .iter()
            .filter(|e| {
                e.块元数据
                    .as_ref()
                    .map(|m| m.绑定任务 == "任务X")
                    .unwrap_or(false)
            })
            .collect();
        assert_eq!(命中.len(), 1, "同一任务重复写入应覆盖而非新增幻影块");
        assert_eq!(命中[0].内容, "第二次内容", "覆盖后内容应为最新");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 按格位写入合法格位摘要出现() {
        let 库 = 临时库("记忆库_按格位.db");
        let _ = std::fs::remove_file(&库);
        写入_按格位(
            &库,
            总纲::经历,
            本质::归档,
            阶段::归档,
            "归档内容",
            "归档摘要",
        )
        .unwrap();
        let 摘要 = 工具永驻摘要(&库);
        assert!(摘要
            .iter()
            .any(|s| s.contains("[经历·归档]") && s.contains("归档摘要")));
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 按格位写入非法格位拒绝() {
        let 库 = 临时库("记忆库_按格位非法.db");
        let _ = std::fs::remove_file(&库);
        // 目标×归档 总纲本质不匹配（非法格位）→ 应返回 Err
        let 结果 = 写入_按格位(
            &库,
            总纲::目标,
            本质::归档,
            阶段::归档,
            "非法内容",
            "非法摘要",
        );
        assert!(结果.is_err(), "目标×归档 非法格位应被三层防护拒绝");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 格位仓库分区隔离() {
        let 库 = 临时库("记忆库_仓库分区.db");
        let _ = std::fs::remove_file(&库);
        写入_按格位(
            &库,
            总纲::执行,
            本质::命令,
            阶段::提案,
            "提案内容",
            "提案摘要",
        )
        .unwrap();
        let 工具仓库 = 读格位仓库(&库, 总纲::执行, 本质::工具);
        assert!(工具仓库.is_empty(), "同总纲不同本质分区隔离");
        let 命令仓库 = 读格位仓库(&库, 总纲::执行, 本质::命令);
        assert_eq!(命令仓库.len(), 1);
        assert!(命令仓库[0].contains("提案内容"));
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 确认格位记忆_出玉玺覆盖() {
        let 库 = 临时库("记忆库_玉玺.db");
        let _ = std::fs::remove_file(&库);
        写入_按格位(
            &库,
            总纲::执行,
            本质::工具,
            阶段::实施,
            "LLM 生成内容",
            "LLM 摘要",
        )
        .unwrap();
        确认格位记忆(
            &库,
            总纲::执行,
            本质::工具,
            阶段::实施,
            "人类确认内容",
            "确认摘要",
        )
        .unwrap();
        let 仓库 = 读格位仓库(&库, 总纲::执行, 本质::工具);
        assert!(
            仓库[0].contains("人类确认内容"),
            "链头应切到人类确认版本：{}",
            仓库[0]
        );
        let 存储 = SQLite存储::文件新建(&库).unwrap();
        let 全部 = 存储.查_全部();
        let 首条 = 全部.iter().max_by_key(|e| e.id.0).unwrap();
        assert_eq!(首条.来源, 来源::人类, "确认后来源=人类（玉玺）");
        assert_eq!(首条.decided_by, "界主", "确认后决定者=界主");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 事件流_追加有序与区间读取() {
        let 库 = 临时库("记忆库_事件流.db");
        let _ = std::fs::remove_file(&库);
        let id1 = 事件流_记录(&库, "启动", "阶段3 开始").unwrap();
        let id2 = 事件流_记录(&库, "测试", "331 项").unwrap();
        let id3 = 事件流_记录(&库, "收尾", "提交").unwrap();
        assert!(id1 < id2 && id2 < id3, "序号应递增");
        let 区间 = 事件流_读取(&库, id2, id3).unwrap();
        assert_eq!(区间.len(), 2, "区间读应 2 条");
        let 全部 = 事件流_全部(&库).unwrap();
        assert_eq!(全部.len(), 3, "全部应 3 条");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 事件流_并发写不交错() {
        let 库 = 临时库("记忆库_事件流并发.db");
        let _ = std::fs::remove_file(&库);
        let 库一 = 库.clone();
        let t1 = std::thread::spawn(move || {
            for i in 0..5 {
                事件流_记录(&库一, "线程一", &format!("事件{}", i)).unwrap();
            }
        });
        let 库二 = 库.clone();
        let t2 = std::thread::spawn(move || {
            for i in 0..5 {
                事件流_记录(&库二, "线程二", &format!("事件{}", i)).unwrap();
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();
        let 全部 = 事件流_全部(&库).unwrap();
        assert_eq!(全部.len(), 10, "并发写应 10 条不丢失");
        assert!(
            全部.iter().filter(|s| s.contains("线程一")).count() == 5
                && 全部.iter().filter(|s| s.contains("线程二")).count() == 5,
            "两线程各 5 条"
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 完成度自评_刻度合法与拒绝() {
        let 库 = 临时库("记忆库_完成度.db");
        let _ = std::fs::remove_file(&库);
        let _id = 完成度自评(&库, "播种", 4, "36 格位补齐").unwrap();
        let 验收 = 读格位仓库(&库, 总纲::经历, 本质::事件);
        assert!(
            验收
                .iter()
                .any(|s| s.contains("完成度 4/5") && s.contains("播种")),
            "经历×事件 应含完成度 4/5"
        );
        let 非法6 = 完成度自评(&库, "x", 6, "y");
        assert!(
            matches!(非法6, Err(错误::完成度刻度非法(6))),
            "刻度 6 应拒绝"
        );
        let 非法0 = 完成度自评(&库, "x", 0, "y");
        assert!(非法0.is_err(), "刻度 0 应拒绝");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 状态报告_空库零panic_四段齐全() {
        let 库 = 临时库("状态报告_空.db");
        let _ = std::fs::remove_file(&库);
        let 行 = 状态报告(&库).unwrap();
        let 全文 = 行.join("\n");
        assert!(全文.contains("== 状态报告 =="), "应含报告头");
        assert!(全文.contains("[债务] 0"), "空库债务 0：{}", 全文);
        assert!(全文.contains("[待补提炼] 0 项"), "空库待补 0");
        assert!(全文.contains("[玉玺块] 0 块"), "空库玉玺 0");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 状态报告_写入后债务玉玺正确变化() {
        let 库 = 临时库("状态报告_变.db");
        let _ = std::fs::remove_file(&库);
        // 制造债务：前端登记 + 交付（债务 +1）
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let mut 双工 = jiyi_chengzai_fu::双工流水线::新建(存储);
            双工.前端_登记("状态任务").unwrap();
            双工.前端_交付("状态任务").unwrap();
        }
        // 写入玉玺块：来源=人类（玉玺权威，规则×规范 经档）
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let mut 中枢 = jiyi_chengzai_fu::格位中枢::新建(存储);
            中枢
                .写入_格位_幂等(
                    jiyi_chengzai_fu::格位 {
                        总纲: 总纲::规则,
                        本质: 本质::规范,
                    },
                    jiyi_chengzai_fu::阶段::拍板,
                    "规范内容",
                    "规范摘要",
                    jiyi_chengzai_fu::档位::经档,
                    jiyi_chengzai_fu::来源::人类,
                    "界主",
                    "道·元",
                    "规范任务",
                )
                .unwrap();
        }
        let 行 = 状态报告(&库).unwrap();
        let 全文 = 行.join("\n");
        assert!(全文.contains("[债务] 1"), "债务应为 1：{}", 全文);
        assert!(全文.contains("[玉玺块] 1 块"), "玉玺块应为 1：{}", 全文);
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 格位统计_稀缺可见且数准确() {
        let 库 = 临时库("记忆库_格位统计.db");
        let _ = std::fs::remove_file(&库);
        let 空 = 格位统计(&库).unwrap();
        assert!(空.iter().any(|s| s.ends_with("=0")), "空库格位应全 0");
        assert!(
            空.iter().any(|s| s.starts_with("总条数：0")),
            "空库总条数应 0"
        );
        写入_按格位(&库, 总纲::经历, 本质::事件, 阶段::实施, "甲", "摘要甲").unwrap();
        写入_按格位(&库, 总纲::经历, 本质::事件, 阶段::实施, "乙", "摘要乙").unwrap();
        let 行 = 格位统计(&库).unwrap();
        assert!(行.iter().any(|s| s == "经历·事件=2"), "经历·事件 应=2");
        assert!(行.iter().any(|s| s.starts_with("总条数：2")), "总条数应 2");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 世界快照_链头更新且不重复() {
        let 库 = 临时库("记忆库_世界快照.db");
        let _ = std::fs::remove_file(&库);
        世界快照(&库, "335", "32").unwrap();
        let 验收 = 读格位仓库(&库, 总纲::外在, 本质::数据);
        assert!(
            验收.iter().any(|s| s.contains("测试数=335")),
            "外在×数据 应含测试数=335"
        );
        世界快照(&库, "336", "32").unwrap();
        let 验收2 = 读格位仓库(&库, 总纲::外在, 本质::数据);
        assert_eq!(
            验收2.iter().filter(|s| s.contains("测试数=336")).count(),
            1,
            "336 应只 1 条"
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 任务收尾_完成度与精炼一步到位() {
        let 库 = 临时库("记忆库_任务收尾.db");
        let _ = std::fs::remove_file(&库);
        记会话(&库, "流水线X", &["写入 SQLite".to_string()]);
        let 收 = 任务收尾(&库, "流水线X", "实现完成 测试数增加", 4).unwrap();
        assert_eq!(收.len(), 3, "收尾日志应 3 条");
        let 事件 = 事件流_全部(&库).unwrap();
        assert!(事件
            .iter()
            .any(|s| s.contains("任务收尾") && s.contains("完成度 4/5")));
        let 验收 = 读格位仓库(&库, 总纲::经历, 本质::事件);
        assert!(验收
            .iter()
            .any(|s| s.contains("完成度 4/5") && s.contains("流水线X")));
        let 全部 = 查全部记忆(&库);
        assert!(全部.iter().any(|s| s.contains("测试数增加")));
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 会话记录_轨迹保留与终点归档() {
        let 库 = 临时库("记忆库_会话记录.db");
        let _ = std::fs::remove_file(&库);
        let 轨迹: Vec<String> = ["发起请求", "解析响应", "写入 SQLite"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ids = 记会话(&库, "流水线A", &轨迹);
        assert_eq!(ids.len(), 3, "3 行轨迹应记 3 条");
        let 会话 = 查会话(&库, "流水线A");
        assert_eq!(会话.len(), 3, "查会话应回 3 行");
        let n = 终点归档(&库, "流水线A", "实现完成 测试数增加");
        assert!(n >= 2, "归档应提炼至少 2 条");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 地道精炼_提炼教训机制波及() {
        let 库 = 临时库("记忆库_地道精炼.db");
        let _ = std::fs::remove_file(&库);
        let ids = 地道精炼(
            &库,
            "修三档投影 clippy",
            "修复 3 处 clippy；新增 播种格位36 实现落地；328 测试数",
        );
        assert!(ids.len() >= 2, "应提炼至少 2 条");
        let 教训 = 读格位仓库(&库, 总纲::经历, 本质::教训);
        assert!(
            教训.iter().any(|s| s.contains("教训")),
            "经历×教训 应含教训"
        );
        let 事件 = 读格位仓库(&库, 总纲::经历, 本质::事件);
        assert!(
            事件.iter().any(|s| s.contains("机制")),
            "经历×事件 应含机制"
        );
        assert!(
            事件.iter().any(|s| s.contains("波及")),
            "经历×事件 应含波及"
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 播种格位36_补满36格位且幂等() {
        let 库 = 临时库("记忆库_播种36.db");
        let _ = std::fs::remove_file(&库);
        let 补数 = 播种格位36(&库);
        assert_eq!(补数, 31, "36 格位减种子 5 应补 31：{}", 补数);
        let 摘要 = 工具永驻摘要(&库);
        let 有内容 = 摘要
            .iter()
            .filter(|s| s.contains(']') && !s.ends_with("] "))
            .count();
        assert_eq!(有内容, 36, "永驻摘要有内容行应为 36（全合法）");
        assert_eq!(摘要.len(), 36, "永驻摘要应 36 行");
        assert!(
            !摘要.iter().any(|s| s.contains("本质不可用")),
            "新模型 36 格位全合法，不应有本质不可用标注"
        );
        let 全部 = 查全部记忆(&库);
        assert_eq!(全部.len(), 36, "全库应为 36 条（种子5+播31）");
        let 补2 = 播种格位36(&库);
        assert_eq!(补2, 0, "第二次播种应无新增（幂等）");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 三档投影_种子经档入首因() {
        let 库 = 临时库("记忆库_三档投影1.db");
        let _ = std::fs::remove_file(&库);
        crate::记忆读取_殿::读取方法_阁::读取实现_园::读取任务相关记忆_持久(&库, "种子触发");
        let (首, _近, _会) = 读取_三档投影(&库);
        assert!(
            首.iter().any(|s| s.contains("目标")),
            "种子 目标×未来 经档应入首因：{}",
            首.join(";")
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 三档投影_权档近因与行档会话() {
        let 库 = 临时库("记忆库_三档投影2.db");
        let _ = std::fs::remove_file(&库);
        写入_按格位(
            &库,
            总纲::执行,
            本质::工具,
            阶段::实施,
            "权档内容A",
            "摘要A",
        )
        .unwrap();
        写入_按格位(
            &库,
            总纲::执行,
            本质::工具,
            阶段::实施,
            "权档内容B",
            "摘要B",
        )
        .unwrap();
        确认格位记忆(
            &库,
            总纲::执行,
            本质::工具,
            阶段::实施,
            "权档人类确认",
            "确认",
        )
        .unwrap();
        登记世界事实(&库, 阶段::验收, "行档测试数", "326-13-13").unwrap();
        let (首, 近, 会) = 读取_三档投影(&库);
        assert!(首.is_empty(), "无经档写入时首因为空");
        assert!(
            近.iter().any(|s| s.contains("权档人类确认")),
            "近因=每格位最新权档：{}",
            近.join(";")
        );
        assert!(
            !近.iter().any(|s| s.contains("权档内容B")),
            "近因只取最新权档"
        );
        assert!(
            会.iter().any(|s| s.contains("行档测试数")),
            "会话=行档条目：{}",
            会.join(";")
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 登记世界事实_代码源() {
        let 库 = 临时库("记忆库_世界事实.db");
        let _ = std::fs::remove_file(&库);
        登记世界事实(&库, 阶段::验收, "325 测试全绿", "325-13-13 门禁").unwrap();
        let 仓库 = 读格位仓库(&库, 总纲::外在, 本质::数据);
        assert!(仓库[0].contains("325 测试全绿"), "链头应切到登记事实");
        let 存储 = SQLite存储::文件新建(&库).unwrap();
        let 全部 = 存储.查_全部();
        let 首条 = 全部.iter().max_by_key(|e| e.id.0).unwrap();
        assert_eq!(首条.来源, 来源::代码, "世界事实来源=代码");
        assert_eq!(首条.decided_by, "扫描", "世界事实决定者=扫描");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn LLM生成内容_无玉玺() {
        let 库 = 临时库("记忆库_无玉玺.db");
        let _ = std::fs::remove_file(&库);
        写入_按格位(
            &库,
            总纲::执行,
            本质::工具,
            阶段::实施,
            "LLM 生成内容",
            "LLM 摘要",
        )
        .unwrap();
        let 存储 = SQLite存储::文件新建(&库).unwrap();
        let 全部 = 存储.查_全部();
        let 尾条 = 全部.iter().max_by_key(|e| e.id.0).unwrap();
        assert_eq!(尾条.来源, 来源::LLM, "LLM 生成无玉玺");
        assert_eq!(尾条.decided_by, "ai助手", "LLM 生成决定者=ai助手");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 格位仓库深挖按ID降序() {
        let 库 = 临时库("记忆库_仓库深挖.db");
        let _ = std::fs::remove_file(&库);
        写入_按格位(&库, 总纲::经历, 本质::归档, 阶段::归档, "首条经历", "首条").unwrap();
        写入_按格位(&库, 总纲::经历, 本质::归档, 阶段::归档, "最近经历", "最近").unwrap();
        let 仓库 = 读格位仓库(&库, 总纲::经历, 本质::归档);
        assert_eq!(仓库.len(), 2);
        assert!(仓库[0].contains("最近经历"), "ID 降序 → 最近优先");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 流水线记忆闭环_端到端() {
        let 库 = 临时库("记忆库_流水线闭环.db");
        let _ = std::fs::remove_file(&库);
        let 前 = 读任务记忆(&库, "实现 Cargo 测试");
        assert!(前.iter().any(|m| m.contains("36 格位闭环 API")));
        写入_按格位(
            &库,
            总纲::执行,
            本质::工具,
            阶段::实施,
            "流水线执行结果：集成闭环通过",
            "集成闭环",
        )
        .unwrap();
        let 后 = 读任务记忆(&库, "实现 Cargo 测试");
        assert!(后
            .iter()
            .any(|m| m.contains("流水线执行结果：集成闭环通过")));
        let 摘要 = 工具永驻摘要(&库);
        assert!(摘要.iter().any(|s| s.contains("集成闭环")));
        let 全部 = 查全部记忆(&库);
        assert_eq!(全部.len(), 6, "5 种子 + 1 流水线写入，无覆盖");
        let _ = std::fs::remove_file(&库);
    }
}
