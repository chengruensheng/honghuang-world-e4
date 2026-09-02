//! 查询-实现-园 - 从「洪荒记忆库.sq3」读取真实数据，转 JSON
//!
//! 真实数据表结构：
//! - 任务账本（任务标识 / 已交付 / 已归档 / 更新时间）
//! - 事件流（序号 / 时间戳 / 事件类型 / 内容）
//! - 记忆条目（id / 总纲 / 本质 / 阶段 / 档位 / 来源 / 内容 / 摘要 / decided_by / …）

use gongju_fu::{命令执行结果, 执行命令_结构化};
use rusqlite::Connection;

/// 打开记忆库（只读），返回连接或错误
fn 打开库(路径: &str) -> Result<Connection, String> {
    Connection::open(路径).map_err(|错| format!("无法打开记忆库：{}", 错))
}

/// 读取总览：任务/事件/记忆三表计数 + 交付与归档统计
pub fn 读取_总览(路径: &str) -> Result<serde_json::Value, String> {
    let 库 = 打开库(路径)?;
    let 任务总数 = 计数(&库, "任务账本")?;
    let 事件总数 = 计数(&库, "事件流")?;
    let 记忆总数 = 计数(&库, "记忆条目")?;
    let 已交付 = 条件计数(&库, "任务账本", "已交付 = 1")?;
    let 已归档 = 条件计数(&库, "任务账本", "已归档 = 1")?;
    let 打回数 = 条件计数(
        &库,
        "事件流",
        "事件类型 IN ('打回重投','打回达上限','终裁打回')",
    )?;
    let 终裁通过 = 条件计数(&库, "事件流", "事件类型 = '终裁通过交付'")?;
    Ok(serde_json::json!({
        "任务总数": 任务总数,
        "事件总数": 事件总数,
        "记忆总数": 记忆总数,
        "已交付": 已交付,
        "已归档": 已归档,
        "打回数": 打回数,
        "终裁通过": 终裁通过,
    }))
}

/// 读取任务账本：全量任务，倒序（最近在前）
pub fn 读取_任务账本(路径: &str) -> Result<serde_json::Value, String> {
    let 库 = 打开库(路径)?;
    let mut 查询 = 库
        .prepare("SELECT 任务标识, 已交付, 已归档, 更新时间 FROM 任务账本 ORDER BY 更新时间 DESC")
        .map_err(|错| 错.to_string())?;
    let 行 = 查询
        .query_map([], |行| {
            Ok(serde_json::json!({
                "任务标识": 行.get::<_, String>(0)?,
                "已交付": 行.get::<_, i64>(1)?,
                "已归档": 行.get::<_, i64>(2)?,
                "更新时间": 行.get::<_, String>(3)?,
            }))
        })
        .map_err(|错| 错.to_string())?;
    let mut 列表 = Vec::new();
    for 条目 in 行 {
        列表.push(条目.map_err(|错| 错.to_string())?);
    }
    Ok(serde_json::json!({ "任务": 列表 }))
}

/// 读取事件流：全量事件，按序号升序（时间正序）
pub fn 读取_事件流(路径: &str) -> Result<serde_json::Value, String> {
    let 库 = 打开库(路径)?;
    let mut 查询 = 库
        .prepare("SELECT 序号, 时间戳, 事件类型, 内容 FROM 事件流 ORDER BY 序号 ASC")
        .map_err(|错| 错.to_string())?;
    let 行 = 查询
        .query_map([], |行| {
            Ok(serde_json::json!({
                "序号": 行.get::<_, i64>(0)?,
                "时间戳": 行.get::<_, String>(1)?,
                "事件类型": 行.get::<_, String>(2)?,
                "内容": 行.get::<_, String>(3)?,
            }))
        })
        .map_err(|错| 错.to_string())?;
    let mut 列表 = Vec::new();
    for 条目 in 行 {
        列表.push(条目.map_err(|错| 错.to_string())?);
    }
    Ok(serde_json::json!({ "事件": 列表 }))
}

/// 读取记忆条目：全量记忆，按 id 升序
pub fn 读取_记忆条目(路径: &str) -> Result<serde_json::Value, String> {
    let 库 = 打开库(路径)?;
    let mut 查询 = 库
        .prepare(
            "SELECT id, 总纲, 本质, 阶段, 档位, 来源, 内容, 摘要, decided_by, 生效状态 \
             FROM 记忆条目 ORDER BY id ASC",
        )
        .map_err(|错| 错.to_string())?;
    let 行 = 查询
        .query_map([], |行| {
            Ok(serde_json::json!({
                "id": 行.get::<_, i64>(0)?,
                "总纲": 行.get::<_, String>(1)?,
                "本质": 行.get::<_, String>(2)?,
                "阶段": 行.get::<_, String>(3)?,
                "档位": 行.get::<_, String>(4)?,
                "来源": 行.get::<_, String>(5)?,
                "内容": 行.get::<_, String>(6)?,
                "摘要": 行.get::<_, String>(7)?,
                "decided_by": 行.get::<_, Option<String>>(8)?,
                "生效状态": 行.get::<_, Option<String>>(9)?,
            }))
        })
        .map_err(|错| 错.to_string())?;
    let mut 列表 = Vec::new();
    for 条目 in 行 {
        列表.push(条目.map_err(|错| 错.to_string())?);
    }
    Ok(serde_json::json!({ "记忆": 列表 }))
}

/// 表计数
fn 计数(库: &Connection, 表: &str) -> Result<u64, String> {
    库.query_row(&format!("SELECT COUNT(*) FROM \"{}\"", 表), [], |行| {
        行.get::<_, i64>(0)
    })
    .map(|量| 量 as u64)
    .map_err(|错| 错.to_string())
}

/// 条件计数
fn 条件计数(库: &Connection, 表: &str, 条件: &str) -> Result<u64, String> {
    库.query_row(
        &format!("SELECT COUNT(*) FROM \"{}\" WHERE {}", 表, 条件),
        [],
        |行| 行.get::<_, i64>(0),
    )
    .map(|量| 量 as u64)
    .map_err(|错| 错.to_string())
}

/// 执行白名单命令并返回结构化终端输出。
///
/// 门户终端只允许执行项目治理白名单内的命令（cargo 构建/测试/格式/静态检查/一键全验），
/// 返回退出码、标准输出、标准错误，供前端原样展示。
pub fn 执行_终端命令(命令: &str) -> Result<命令执行结果, String> {
    执行命令_结构化(命令)
}

#[cfg(test)]
mod 测试 {
    use super::*;

    /// 建一个含真实表结构的临时库，写入 3 任务 / 5 事件 / 2 记忆
    fn 建测试库(名: &str) -> String {
        let 路径 = std::env::temp_dir().join(format!("门户测试_{}.db", 名));
        let _ = std::fs::remove_file(&路径);
        let 库 = Connection::open(&路径).expect("建库失败");
        库.execute_batch(
            "CREATE TABLE 任务账本(任务标识 TEXT, 已交付 INTEGER, 已归档 INTEGER, 更新时间 TEXT);
             CREATE TABLE 事件流(序号 INTEGER, 时间戳 TEXT, 事件类型 TEXT, 内容 TEXT);
             CREATE TABLE 记忆条目(id INTEGER, 总纲 TEXT, 本质 TEXT, 阶段 TEXT, 档位 TEXT, 来源 TEXT, 内容 TEXT, 摘要 TEXT, decided_by TEXT, 生效状态 TEXT);
             INSERT INTO 任务账本 VALUES('甲',1,1,'1'),('乙',1,0,'2'),('丙',0,0,'3');
             INSERT INTO 事件流 VALUES(1,'1','终裁通过交付','甲'),(2,'1','打回重投','乙'),(3,'1','打回达上限','乙'),(4,'1','终裁打回','丙'),(5,'1','终裁通过交付','乙');
             INSERT INTO 记忆条目 VALUES(1,'目标','未来','拍板','经档','人类','a','a',NULL,NULL),(2,'规则','门禁','实施','权档','人类','b','b',NULL,NULL);",
        )
        .expect("建表失败");
        drop(库);
        路径.to_string_lossy().to_string()
    }

    #[test]
    fn 测试_读取总览() {
        let 路径 = 建测试库("总览");
        let 值 = 读取_总览(&路径).unwrap();
        assert_eq!(值["任务总数"].as_i64().unwrap(), 3);
        assert_eq!(值["事件总数"].as_i64().unwrap(), 5);
        assert_eq!(值["记忆总数"].as_i64().unwrap(), 2);
        assert_eq!(值["已交付"].as_i64().unwrap(), 2);
        assert_eq!(值["打回数"].as_i64().unwrap(), 3);
        assert_eq!(值["终裁通过"].as_i64().unwrap(), 2);
        let _ = std::fs::remove_file(路径);
    }

    #[test]
    fn 测试_读取任务与事件与记忆() {
        let 路径 = 建测试库("明细");
        let 任务 = 读取_任务账本(&路径).unwrap();
        assert_eq!(任务["任务"].as_array().unwrap().len(), 3);
        let 事件 = 读取_事件流(&路径).unwrap();
        assert_eq!(事件["事件"].as_array().unwrap().len(), 5);
        let 记忆 = 读取_记忆条目(&路径).unwrap();
        assert_eq!(记忆["记忆"].as_array().unwrap().len(), 2);
        let _ = std::fs::remove_file(路径);
    }

    #[test]
    fn 测试_打不开的库返回错误() {
        // 父目录不存在 → SQLite 无法创建文件 → 打开必失败，且不留任何残留文件
        let 路径 = std::env::temp_dir().join("门户不存在目录_abc/不存在的路径.sq3");
        assert!(读取_总览(&路径.to_string_lossy()).is_err());
    }
}
