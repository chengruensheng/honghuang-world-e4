//! SQLite 阁 - SQLite 经档持久化后端（实现 trait 记忆存储）
//!
//! 决策锚：260826-2240 传承殿启动 § 记忆模型
//! 关联文档：02-概念/记忆/03-记忆.md + 04-设计/数据模型/01-记忆.md

// 跨殿引用：类型定义在类型定义殿，trait 在存储操作殿（六层返工后改用 crate:: 路径）
use crate::记忆_存储_殿::记忆存储;
use crate::记忆_类型_殿::{
    块元数据, 总纲, 所有本质, 本质, 来源, 档位, 生效状态, 记忆ID, 记忆条目, 错误, 阶段,
};

// ============================================================================
// SQLite 存储后端
// ============================================================================

/// SQLite 存储后端
pub struct SQLite存储 {
    db: rusqlite::Connection,
}

impl SQLite存储 {
    pub fn 内存新建() -> Result<Self, 错误> {
        let conn = rusqlite::Connection::open_in_memory().map_err(|e| 错误::持久化故障 {
            分类: 持久化故障类_从rusqlite(&e),
            细节: format!("SQLite 内存打开失败：{}", e),
        })?;
        Self::初始化(conn)
    }

    pub fn 文件新建(路径: &str) -> Result<Self, 错误> {
        let conn = rusqlite::Connection::open(路径).map_err(|e| 错误::持久化故障 {
            分类: 持久化故障类_从rusqlite(&e),
            细节: format!("SQLite 文件打开失败：{}", e),
        })?;
        Self::初始化(conn)
    }

    fn 初始化(conn: rusqlite::Connection) -> Result<Self, 错误> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 记忆条目 (
                id INTEGER PRIMARY KEY,
                总纲 TEXT NOT NULL,
                本质 TEXT NOT NULL,
                阶段 TEXT NOT NULL,
                档位 TEXT NOT NULL,
                来源 TEXT NOT NULL,
                内容 TEXT NOT NULL,
                摘要 TEXT NOT NULL,
                decided_by TEXT NOT NULL,
                implements TEXT NOT NULL,
                hash INTEGER NOT NULL,
                软放弃 INTEGER NOT NULL DEFAULT 0,
                绑定任务 TEXT,
                手印 TEXT,
                注脚 TEXT,
                生效状态 TEXT,
                生效窗口 TEXT,
                可证伪 TEXT
            )",
            [],
        )
        .map_err(|e| 错误::持久化故障 {
            分类: 持久化故障类_从rusqlite(&e),
            细节: format!("SQLite 表创建失败：{}", e),
        })?;
        // 幂等迁移：旧表补契约 6 列（CREATE TABLE IF NOT EXISTS 不会为已存在的旧表加列）
        Self::迁移补列(&conn)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 事件流 (
                序号 INTEGER PRIMARY KEY AUTOINCREMENT,
                时间戳 TEXT NOT NULL,
                事件类型 TEXT NOT NULL,
                内容 TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| 错误::持久化故障 {
            分类: 持久化故障类_从rusqlite(&e),
            细节: format!("SQLite 事件流表创建失败：{}", e),
        })?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 任务账本 (
                任务标识 TEXT PRIMARY KEY,
                已交付 INTEGER NOT NULL DEFAULT 0,
                已归档 INTEGER NOT NULL DEFAULT 0,
                更新时间 TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| 错误::持久化故障 {
            分类: 持久化故障类_从rusqlite(&e),
            细节: format!("SQLite 任务账本表创建失败：{}", e),
        })?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 降级快照 (
                任务标识 TEXT PRIMARY KEY
            )",
            [],
        )
        .map_err(|e| 错误::持久化故障 {
            分类: 持久化故障类_从rusqlite(&e),
            细节: format!("SQLite 降级快照表创建失败：{}", e),
        })?;
        // 并发写互斥保障：写锁等待最多 5 秒（事件流 falsifiable：并发写不交错）
        conn.busy_timeout(std::time::Duration::from_secs(5))
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite busy_timeout 设置失败：{}", e),
            })?;
        Ok(Self { db: conn })
    }

    /// 幂等列迁移：探测缺失列则 ALTER TABLE ADD COLUMN（架构演进不丢旧数据）
    fn 迁移补列(conn: &rusqlite::Connection) -> Result<(), 错误> {
        for (列, 类型) in [
            ("绑定任务", "TEXT"),
            ("手印", "TEXT"),
            ("注脚", "TEXT"),
            ("生效状态", "TEXT"),
            ("生效窗口", "TEXT"),
            ("可证伪", "TEXT"),
        ] {
            let 探测 = conn.prepare(&format!("SELECT {列} FROM 记忆条目 LIMIT 1"));
            if 探测.is_err() {
                conn.execute(&format!("ALTER TABLE 记忆条目 ADD COLUMN {列} {类型}"), [])
                    .map_err(|e| 错误::持久化故障 {
                        分类: 持久化故障类_从rusqlite(&e),
                        细节: format!("SQLite 迁移补列 {列} 失败：{}", e),
                    })?;
            }
        }
        Ok(())
    }
}

fn 总纲_到串(c: 总纲) -> &'static str {
    c.名称()
}
fn 总纲_从串(s: &str) -> Option<总纲> {
    match s {
        "内部" => Some(总纲::内部),
        "外在" => Some(总纲::外在),
        "规则" => Some(总纲::规则),
        "执行" => Some(总纲::执行),
        "目标" => Some(总纲::目标),
        "经历" => Some(总纲::经历),
        _ => None,
    }
}
fn 本质_到串(c: 本质) -> &'static str {
    c.名称()
}
fn 本质_从串(s: &str) -> Option<本质> {
    所有本质.iter().copied().find(|b| b.名称() == s)
}
fn 阶段_到串(p: 阶段) -> &'static str {
    p.名称()
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
    d.名称()
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
    y.名称()
}
fn 来源_从串(s: &str) -> Option<来源> {
    match s {
        "代码" => Some(来源::代码),
        "LLM" => Some(来源::LLM),
        "人类" => Some(来源::人类),
        _ => None,
    }
}

/// 从 6 个可空列解析块元数据：绑定任务为空 → None（契约块缺一即拒，无元数据不构造）
fn 块元数据_从串(
    绑定任务: Option<String>,
    手印: Option<String>,
    注脚: Option<String>,
    生效状态列: Option<String>,
    生效窗口: Option<String>,
    可证伪: Option<String>,
) -> Option<块元数据> {
    let 绑定任务 = 绑定任务?;
    let 状态 = match 生效状态列.as_deref() {
        Some("待确认") => 生效状态::待确认,
        Some("废止") => 生效状态::废止,
        _ => 生效状态::生效,
    };
    Some(块元数据 {
        绑定任务,
        手印: 手印.unwrap_or_default(),
        注脚: 注脚.unwrap_or_default(),
        生效状态: 状态,
        生效窗口: 生效窗口.unwrap_or_default(),
        可证伪: 可证伪.unwrap_or_default(),
    })
}

/// 当前 UNIX 秒时间戳（账本更新时间）
fn 当前时间戳() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

/// 持久化故障分类：rusqlite 错误码 → 可读分类（磁盘满/只读/损坏/锁超时）
///
/// 决策锚：100项任务 任务17——写路径错误不吞不降级，显式分类传播。
/// SQLite 扩展错误码：13 磁盘满 / 8 只读 / 11 损坏 / 26 非数据库 / 5 忙 / 6 锁。
fn 持久化故障类_从rusqlite(
    e: &rusqlite::Error,
) -> crate::记忆_类型_殿::持久化故障类 {
    use crate::记忆_类型_殿::持久化故障类;
    match e {
        rusqlite::Error::SqliteFailure(err, _) => match err.extended_code {
            13 => 持久化故障类::磁盘满,
            8 => 持久化故障类::只读,
            11 | 26 => 持久化故障类::损坏,
            5 | 6 => 持久化故障类::锁超时,
            _ => 持久化故障类::其他,
        },
        _ => 持久化故障类::其他,
    }
}

impl 记忆存储 for SQLite存储 {
    fn 读(&self, id: 记忆ID) -> Option<记忆条目> {
        let mut stmt = self.db.prepare(
            "SELECT 总纲, 本质, 阶段, 档位, 来源, 内容, 摘要, decided_by, implements, hash, 软放弃, 绑定任务, 手印, 注脚, 生效状态, 生效窗口, 可证伪 FROM 记忆条目 WHERE id = ?1"
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
                    row.get::<_, String>(8)?,
                    row.get::<_, i64>(9)?,
                    row.get::<_, i64>(10)?,
                    row.get::<_, Option<String>>(11)?,
                    row.get::<_, Option<String>>(12)?,
                    row.get::<_, Option<String>>(13)?,
                    row.get::<_, Option<String>>(14)?,
                    row.get::<_, Option<String>>(15)?,
                    row.get::<_, Option<String>>(16)?,
                ))
            })
            .ok()?;
        let 条目 = 记忆条目::从持久化(
            id.0,
            总纲_从串(&row.0)?,
            本质_从串(&row.1)?,
            阶段_从串(&row.2)?,
            档位_从串(&row.3)?,
            来源_从串(&row.4)?,
            row.5,
            row.6,
            row.7,
            row.8,
            row.9 as u64,
            row.10 != 0,
            块元数据_从串(row.11, row.12, row.13, row.14, row.15, row.16),
        );
        // 防篡改：hash 与字段不一致说明数据被篡改或损坏，拒绝返回
        if !条目.校验哈希() {
            return None;
        }
        Some(条目)
    }

    fn 写(&mut self, 条目: 记忆条目) -> Result<(), 错误> {
        let (绑定任务, 手印, 注脚, 生效状态列, 生效窗口, 可证伪) = match &条目.块元数据 {
            Some(m) => (
                Some(m.绑定任务.clone()),
                Some(m.手印.clone()),
                Some(m.注脚.clone()),
                Some(m.生效状态.名称().to_string()),
                Some(m.生效窗口.clone()),
                Some(m.可证伪.clone()),
            ),
            None => (None, None, None, None, None, None),
        };
        self.db.execute(
            "INSERT OR REPLACE INTO 记忆条目 (id, 总纲, 本质, 阶段, 档位, 来源, 内容, 摘要, decided_by, implements, hash, 软放弃, 绑定任务, 手印, 注脚, 生效状态, 生效窗口, 可证伪) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            rusqlite::params![
                条目.id.0 as i64,
                总纲_到串(条目.总纲),
                本质_到串(条目.本质),
                阶段_到串(条目.阶段),
                档位_到串(条目.档位),
                来源_到串(条目.来源),
                条目.内容,
                条目.摘要,
                条目.decided_by,
                条目.implements,
                条目.hash as i64,
                条目.软放弃 as i64,
                绑定任务,
                手印,
                注脚,
                生效状态列,
                生效窗口,
                可证伪,
            ],
        ).map_err(|e| 错误::持久化故障 { 分类: 持久化故障类_从rusqlite(&e), 细节: format!("SQLite 写入失败：{}", e) })?;
        Ok(())
    }

    fn 删(&mut self, id: 记忆ID) -> Result<(), 错误> {
        self.db
            .execute(
                "DELETE FROM 记忆条目 WHERE id = ?1",
                rusqlite::params![id.0 as i64],
            )
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 删除失败：{}", e),
            })?;
        Ok(())
    }

    fn 查_全部(&self) -> Vec<记忆条目> {
        let mut stmt = match self.db.prepare(
            "SELECT id, 总纲, 本质, 阶段, 档位, 来源, 内容, 摘要, decided_by, implements, hash, 软放弃, 绑定任务, 手印, 注脚, 生效状态, 生效窗口, 可证伪 FROM 记忆条目",
        ) {
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
                row.get::<_, String>(9)?,
                row.get::<_, i64>(10)?,
                row.get::<_, i64>(11)?,
                row.get::<_, Option<String>>(12)?,
                row.get::<_, Option<String>>(13)?,
                row.get::<_, Option<String>>(14)?,
                row.get::<_, Option<String>>(15)?,
                row.get::<_, Option<String>>(16)?,
                row.get::<_, Option<String>>(17)?,
            ))
        }) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        rows.filter_map(|r| {
            r.ok().and_then(
                |(
                    id,
                    gang,
                    ben,
                    ph,
                    lvl,
                    src,
                    content,
                    summary,
                    dec,
                    imp,
                    hash,
                    软,
                    绑,
                    印,
                    脚,
                    态,
                    窗,
                    伪,
                )| {
                    let 条目 = 记忆条目::从持久化(
                        id as u64,
                        总纲_从串(&gang)?,
                        本质_从串(&ben)?,
                        阶段_从串(&ph)?,
                        档位_从串(&lvl)?,
                        来源_从串(&src)?,
                        content,
                        summary,
                        dec,
                        imp,
                        hash as u64,
                        软 != 0,
                        块元数据_从串(绑, 印, 脚, 态, 窗, 伪),
                    );
                    // 防篡改：hash 不一致的条目跳过
                    if !条目.校验哈希() {
                        return None;
                    }
                    Some(条目)
                },
            )
        })
        .collect()
    }

    fn 事件流_追加(&mut self, 事件类型: &str, 内容: &str) -> Result<i64, 错误> {
        let 时间戳 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_else(|_| "0".to_string());
        let 结果 = self.db.execute(
            "INSERT INTO 事件流 (时间戳, 事件类型, 内容) VALUES (?1, ?2, ?3)",
            rusqlite::params![时间戳, 事件类型, 内容],
        );
        match 结果 {
            Ok(_) => Ok(self.db.last_insert_rowid()),
            Err(e) => Err(错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("事件流追加失败：{}", e),
            }),
        }
    }

    fn 事件流_区间(&self, 起: i64, 止: i64) -> Vec<(i64, String, String, String)> {
        let mut stmt = match self.db.prepare(
            "SELECT 序号, 时间戳, 事件类型, 内容 FROM 事件流 WHERE 序号 BETWEEN ?1 AND ?2 ORDER BY 序号",
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let rows = match stmt.query_map(rusqlite::params![起, 止], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        }) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        rows.filter_map(|r| r.ok()).collect()
    }

    fn 账本_登记(&mut self, 任务标识: &str) -> Result<(), 错误> {
        let 现在 = 当前时间戳();
        self.db
            .execute(
                "INSERT OR REPLACE INTO 任务账本 (任务标识, 已交付, 已归档, 更新时间) VALUES (?1, 0, 0, ?2)",
                rusqlite::params![任务标识, 现在],
            )
            .map_err(|e| 错误::持久化故障 { 分类: 持久化故障类_从rusqlite(&e), 细节: format!("SQLite 账本登记失败：{}", e) })?;
        Ok(())
    }

    fn 账本_标记交付(&mut self, 任务标识: &str) -> Result<(), 错误> {
        let 现在 = 当前时间戳();
        let n = self
            .db
            .execute(
                "UPDATE 任务账本 SET 已交付 = 1, 更新时间 = ?2 WHERE 任务标识 = ?1",
                rusqlite::params![任务标识, 现在],
            )
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 账本标记交付失败：{}", e),
            })?;
        if n == 0 {
            return Err(错误::账本任务不存在(任务标识.to_string()));
        }
        Ok(())
    }

    fn 账本_标记归档(&mut self, 任务标识: &str) -> Result<(), 错误> {
        let 现在 = 当前时间戳();
        let n = self
            .db
            .execute(
                "UPDATE 任务账本 SET 已归档 = 1, 更新时间 = ?2 WHERE 任务标识 = ?1",
                rusqlite::params![任务标识, 现在],
            )
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 账本标记归档失败：{}", e),
            })?;
        if n == 0 {
            return Err(错误::账本任务不存在(任务标识.to_string()));
        }
        Ok(())
    }

    fn 账本_债务(&self) -> Result<i64, 错误> {
        let 交付: i64 = self
            .db
            .query_row(
                "SELECT COUNT(*) FROM 任务账本 WHERE 已交付 = 1",
                [],
                |row| row.get(0),
            )
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 债务查询失败：{}", e),
            })?;
        let 归档: i64 = self
            .db
            .query_row(
                "SELECT COUNT(*) FROM 任务账本 WHERE 已归档 = 1",
                [],
                |row| row.get(0),
            )
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 债务查询失败：{}", e),
            })?;
        Ok(交付 - 归档)
    }

    fn 账本_债务队列(&self) -> Result<Vec<String>, 错误> {
        let mut 语句 = self
            .db
            .prepare("SELECT 任务标识 FROM 任务账本 WHERE 已交付 = 1 AND 已归档 = 0 ORDER BY rowid")
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 债务队列查询失败：{}", e),
            })?;
        let 行们 = 语句
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 债务队列映射失败：{}", e),
            })?;
        let mut 队列 = Vec::new();
        for 行 in 行们 {
            队列.push(行.map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 债务队列行失败：{}", e),
            })?);
        }
        Ok(队列)
    }

    fn 快照_登记(&mut self, 任务标识: &str) -> Result<(), 错误> {
        self.db
            .execute(
                "INSERT OR IGNORE INTO 降级快照 (任务标识) VALUES (?1)",
                [任务标识],
            )
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 快照登记失败：{}", e),
            })?;
        Ok(())
    }

    fn 快照_全部(&self) -> Result<Vec<String>, 错误> {
        let mut 语句 = self
            .db
            .prepare("SELECT 任务标识 FROM 降级快照 ORDER BY rowid")
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 快照查询失败：{}", e),
            })?;
        let 行们 = 语句
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 快照映射失败：{}", e),
            })?;
        let mut 清单 = Vec::new();
        for 行 in 行们 {
            清单.push(行.map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 快照行失败：{}", e),
            })?);
        }
        Ok(清单)
    }

    fn 快照_清除(&mut self, 任务标识: &str) -> Result<(), 错误> {
        self.db
            .execute("DELETE FROM 降级快照 WHERE 任务标识 = ?1", [任务标识])
            .map_err(|e| 错误::持久化故障 {
                分类: 持久化故障类_从rusqlite(&e),
                细节: format!("SQLite 快照清除失败：{}", e),
            })?;
        Ok(())
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod 测试_sqlite {
    use super::*;

    #[test]
    fn 持久化故障类_从rusqlite_五类映射() {
        use crate::记忆_类型_殿::持久化故障类;
        let 造 = |码: i32| rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(码), None);
        assert_eq!(持久化故障类_从rusqlite(&造(13)), 持久化故障类::磁盘满);
        assert_eq!(持久化故障类_从rusqlite(&造(8)), 持久化故障类::只读);
        assert_eq!(持久化故障类_从rusqlite(&造(11)), 持久化故障类::损坏);
        assert_eq!(持久化故障类_从rusqlite(&造(26)), 持久化故障类::损坏);
        assert_eq!(持久化故障类_从rusqlite(&造(5)), 持久化故障类::锁超时);
        assert_eq!(持久化故障类_从rusqlite(&造(6)), 持久化故障类::锁超时);
        assert_eq!(持久化故障类_从rusqlite(&造(1)), 持久化故障类::其他);
        // 非 SqliteFailure（如 ToSql 转换错误）→ 其他
        assert_eq!(
            持久化故障类_从rusqlite(&rusqlite::Error::ToSqlConversionFailure(Box::new(
                std::io::Error::other("x")
            ))),
            持久化故障类::其他
        );
    }

    fn 测试条目(id: u64) -> 记忆条目 {
        记忆条目::新建(
            id,
            总纲::目标,
            本质::未来,
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
        assert_eq!(读.总纲, 总纲::目标);
        assert_eq!(读.本质, 本质::未来);
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
            let 条目 = 记忆条目::新建(
                3,
                总纲::目标,
                本质::未来,
                阶段::实施,
                档位::行档,
                来源::代码,
                "持久化内容",
                "test 摘要",
                "界主",
                "工程-DSH",
            );
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
    fn sqlite_四维正交_保留() {
        let mut s = SQLite存储::内存新建().unwrap();
        let 条目 = 记忆条目::新建(
            4,
            总纲::经历,
            本质::归档,
            阶段::归档,
            档位::经档,
            来源::人类,
            "四维正交测试",
            "经历/归档/归档/经档/人类",
            "界主",
            "工程-DSH",
        );
        s.写(条目).unwrap();
        let 读 = s.读(记忆ID(4)).unwrap();
        assert_eq!(读.总纲, 总纲::经历);
        assert_eq!(读.本质, 本质::归档);
        assert_eq!(读.阶段, 阶段::归档);
        assert_eq!(读.档位, 档位::经档);
        assert_eq!(读.来源, 来源::人类);
    }

    #[test]
    fn sqlite_软放弃_持久化() {
        // 软放弃标志必须跨重启保留（frozen outcome 铁律）
        let 临时路径 = std::env::temp_dir().join("sqlite_softabandon_test.db");
        let _ = std::fs::remove_file(&临时路径);
        {
            let mut s = SQLite存储::文件新建(临时路径.to_str().unwrap()).unwrap();
            let mut 条目 = 测试条目(5);
            条目.软放弃();
            s.写(条目).unwrap();
        }
        {
            let s = SQLite存储::文件新建(临时路径.to_str().unwrap()).unwrap();
            let 读 = s.读(记忆ID(5)).unwrap();
            assert!(读.软放弃, "软放弃标志应在重启后保留");
            assert!(读.校验哈希(), "读回条目 hash 应有效");
        }
        let _ = std::fs::remove_file(&临时路径);
    }

    #[test]
    fn sqlite_hash_防篡改() {
        let mut s = SQLite存储::内存新建().unwrap();
        s.写(测试条目(6)).unwrap();
        s.db.execute("UPDATE 记忆条目 SET 内容 = '被篡改' WHERE id = 6", [])
            .unwrap();
        assert!(s.读(记忆ID(6)).is_none(), "篡改后读回应被拒绝");
    }

    #[test]
    fn sqlite_从持久化_保留hash与软放弃() {
        let mut 条目 = 测试条目(7);
        条目.软放弃();
        let 原hash = 条目.hash;
        let 持久化 = 记忆条目::从持久化(
            条目.id.0,
            条目.总纲,
            条目.本质,
            条目.阶段,
            条目.档位,
            条目.来源,
            条目.内容.clone(),
            条目.摘要.clone(),
            条目.decided_by.clone(),
            条目.implements.clone(),
            条目.hash,
            条目.软放弃,
            None,
        );
        assert_eq!(持久化.hash, 原hash);
        assert!(持久化.软放弃);
        assert!(持久化.校验哈希());
    }

    #[test]
    fn sqlite_块元数据_六字段持久化() {
        let mut 库 = SQLite存储::内存新建().unwrap();
        let 元 = 块元数据 {
            绑定任务: "任务-42".to_string(),
            手印: "界主".to_string(),
            注脚: "元决策".to_string(),
            生效状态: 生效状态::待确认,
            生效窗口: "2026-08-29 起".to_string(),
            可证伪: "绑定任务非空".to_string(),
        };
        let 条 = 记忆条目::新建_契约块(
            42,
            总纲::目标,
            本质::当前,
            阶段::实施,
            档位::行档,
            来源::代码,
            "块内容",
            "块摘要",
            "界主",
            "法·可修正",
            元,
        )
        .unwrap();
        库.写(条).unwrap();
        let 读回 = 库.读(记忆ID(42)).unwrap();
        let m = 读回.块元数据.expect("块元数据应持久化");
        assert_eq!(m.绑定任务, "任务-42");
        assert_eq!(m.手印, "界主");
        assert_eq!(m.注脚, "元决策");
        assert_eq!(m.生效状态, 生效状态::待确认);
        assert_eq!(m.生效窗口, "2026-08-29 起");
        assert_eq!(m.可证伪, "绑定任务非空");
    }

    #[test]
    fn sqlite_账本_债务差集() {
        let mut 库 = SQLite存储::内存新建().unwrap();
        库.账本_登记("任务A").unwrap();
        库.账本_登记("任务B").unwrap();
        库.账本_登记("任务C").unwrap();
        assert_eq!(库.账本_债务().unwrap(), 0);
        库.账本_标记交付("任务A").unwrap();
        库.账本_标记交付("任务B").unwrap();
        assert_eq!(库.账本_债务().unwrap(), 2);
        库.账本_标记归档("任务A").unwrap();
        assert_eq!(库.账本_债务().unwrap(), 1);
    }

    #[test]
    fn sqlite_账本_债务队列先进先出() {
        let mut 库 = SQLite存储::内存新建().unwrap();
        库.账本_登记("任务甲").unwrap();
        库.账本_登记("任务乙").unwrap();
        库.账本_登记("任务丙").unwrap();
        库.账本_标记交付("任务甲").unwrap();
        库.账本_标记交付("任务乙").unwrap();
        // 任务丙未交付，不在债务队列
        assert_eq!(
            库.账本_债务队列().unwrap(),
            vec!["任务甲".to_string(), "任务乙".to_string()]
        );
        库.账本_标记归档("任务甲").unwrap();
        assert_eq!(库.账本_债务队列().unwrap(), vec!["任务乙".to_string()]);
    }

    #[test]
    fn sqlite_账本_未登记标记交付报错() {
        let mut 库 = SQLite存储::内存新建().unwrap();
        let r = 库.账本_标记交付("不存在");
        assert!(matches!(r, Err(错误::账本任务不存在(_))));
    }
}
