//! SQLite 阁 - SQLite 经档持久化后端（实现 trait 记忆存储）
//!
//! v3 阶段 11：SQLite 经档持久化
//! 决策锚：260826-2240 传承殿启动 § 记忆模型
//! 关联文档：02-概念/记忆/03-记忆.md + 04-设计/数据模型/01-记忆.md

// 跨殿引用：类型定义在类型定义殿，trait 在存储操作殿（六层返工后改用 crate:: 路径）
use crate::记忆存储_殿::记忆存储;
use crate::记忆类型_殿::{来源, 档位, 范畴, 记忆ID, 记忆条目, 错误, 阶段};

// ============================================================================
// SQLite 存储后端
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
        let mut stmt = match self.db.prepare(
            "SELECT id, 范畴, 阶段, 档位, 来源, 内容, 摘要, decided_by, implements FROM 记忆条目",
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

// ============================================================================
// 单元测试
// ============================================================================

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
