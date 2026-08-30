//! 任务单校验-阁 - 缺字段校验 + 验收命令白名单
//!
//! 治理铁律：验收命令必须是 cargo 构建/测试类白名单命令（复用 gongju_fu 命令白名单），
//! 拒绝任意 shell 注入；路径必须受 gongju_fu 写路径白名单约束。

use crate::自举任务单_殿::自举任务单;

/// 校验任务单：逐字段非空 + 验收命令白名单 + 目标文件路径白名单
pub fn 校验任务单(单: &自举任务单) -> Result<(), String> {
    if 单.标识.trim().is_empty() {
        return Err("标识为空".to_string());
    }
    if 单.目标文件.trim().is_empty() {
        return Err("目标文件为空".to_string());
    }
    if 单.需求描述.trim().is_empty() {
        return Err("需求描述为空".to_string());
    }
    if 单.decided_by.trim().is_empty() {
        return Err("decided_by为空".to_string());
    }
    // 目标文件路径受写路径白名单约束（治理铁律：不写治理资产）
    gongju_fu::校验_写入路径(单.目标文件.as_str())?;
    // 验收命令受命令白名单约束（治理铁律：只跑 cargo 构建/测试类）
    gongju_fu::校验_命令(单.验收命令.as_str())?;
    Ok(())
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use std::collections::HashMap;

    fn 样例参数() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("标识".to_string(), "自举-001".to_string());
        m.insert("目标文件".to_string(), "世界/入口.rs".to_string());
        m.insert("需求描述".to_string(), "加一个版本命令".to_string());
        m.insert("验收命令".to_string(), "cargo test".to_string());
        m.insert(
            "可证伪命题".to_string(),
            "退出码0且版本字符串非空".to_string(),
        );
        m.insert("decided_by".to_string(), "界主".to_string());
        m
    }

    #[test]
    fn 合法任务单通过() {
        let 单 = 自举任务单::从参数解析(&样例参数()).unwrap();
        assert!(校验任务单(&单).is_ok());
    }

    #[test]
    fn 危险验收命令拒绝() {
        let mut m = 样例参数();
        m.insert("验收命令".to_string(), "rm -rf .".to_string());
        let 单 = 自举任务单::从参数解析(&m).unwrap();
        let r = 校验任务单(&单);
        assert!(r.is_err(), "危险命令应拒绝");
    }

    #[test]
    fn 治理资产目标文件拒绝() {
        let mut m = 样例参数();
        m.insert("目标文件".to_string(), ".env".to_string());
        let 单 = 自举任务单::从参数解析(&m).unwrap();
        assert!(校验任务单(&单).is_err(), ".env 应拒绝");
    }
}
