//! 核心园 - 命令结果 + 命令 trait + 帮助命令 + 分发
//!
//! 殿核心类型与分发函数，桥接 init/status/run/e2e 四阁。

// 跨阁引用：从殿层 re-export 拿各阁符号
use super::super::super::{
    自检命令, 跑流水线_mock_llm, 跑流水线_反序, 跑流水线_循环打回, 跑流水线_跳层, Init命令,
    Run命令, Status命令,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 命令结果 {
    pub 退出码: i32,
    pub 输出: String,
}

impl 命令结果 {
    pub fn 成功(输出: impl Into<String>) -> Self {
        Self {
            退出码: 0,
            输出: 输出.into(),
        }
    }
    pub fn 失败(码: i32, 输出: impl Into<String>) -> Self {
        Self {
            退出码: 码,
            输出: 输出.into(),
        }
    }
}

pub trait 命令: Send + Sync {
    fn 名称(&self) -> &str;
    fn 执行(&self, 参数: &[&str]) -> 命令结果;
}

pub struct 帮助命令;

impl 命令 for 帮助命令 {
    fn 名称(&self) -> &str {
        "帮助"
    }
    fn 执行(&self, _参数: &[&str]) -> 命令结果 {
        命令结果::成功(
            "洪荒 · 世界 v3 · 阶段 8 端到端验证 + CLI\n\n             命令：\n               帮助                显示此帮助\n               init                环境检查 + 输出就绪状态\n               status              健康检查（workspace metadata）\n               run --task=<id>     跑任务（4 分类流水线）\n               e2e                 端到端 mock LLM（v4 阶段 17：真实 HTTP）\n               跳层测试            跳层拒绝路径\n               反序测试            反序拒绝路径\n               循环测试            循环打回升级道祖终裁\n               记忆 <三档|查|播种|精炼|会话|事件|完成度> 记忆命令（无人开发：AI 自主，无玉玺可执行）"
        )
    }
}

/// 记忆命令：命令行读取/播种/自评记忆（无人开发方向——AI 自主，不依赖人类确认）
///
/// 子命令：
/// - 三档 [库路径]：三档投影（首因=经档 / 近因=权档 / 会话=行档）
/// - 查 [库路径]：查全部记忆
/// - 播种 [库路径]：播种格位36（补缺格位，AI 生成无玉玺可执行）
/// - 完成度 <刻度1-5> <依据> [库路径]：完成度自评（写 经历/验收，来源LLM权档无玉玺；刻度非法拒绝）
pub struct 记忆命令;

impl 命令 for 记忆命令 {
    fn 名称(&self) -> &str {
        "记忆"
    }
    fn 执行(&self, 参数: &[&str]) -> 命令结果 {
        use crate::记忆读取_殿::{
            事件流_全部, 事件流_记录, 事件流_读取, 地道精炼, 完成度自评, 播种格位36, 查会话,
            查全部记忆, 格位统计, 终点归档, 读取_三档投影, 默认记忆库路径,
        };
        let 子 = 参数.first().copied().unwrap_or("");
        let 库 = if 参数.len() >= 2 && !参数[1].is_empty() {
            参数[1]
        } else {
            默认记忆库路径
        };
        match 子 {
            "三档" => {
                let (首, 近, 会) = 读取_三档投影(库);
                let mut 输出 = format!("== 记忆三档投影 ==\n库：{}\n", 库);
                输出.push_str("【首因】（经档·经典不变）\n");
                for s in &首 {
                    输出.push_str(&format!("  {}\n", s));
                }
                if 首.is_empty() {
                    输出.push_str("  （空）\n");
                }
                输出.push_str("【近因】（权档·因时制宜）\n");
                for s in &近 {
                    输出.push_str(&format!("  {}\n", s));
                }
                if 近.is_empty() {
                    输出.push_str("  （空）\n");
                }
                输出.push_str("【会话】（行档·当下行动）\n");
                for s in &会 {
                    输出.push_str(&format!("  {}\n", s));
                }
                if 会.is_empty() {
                    输出.push_str("  （空）\n");
                }
                命令结果::成功(输出)
            }
            "查" => {
                let 全部 = 查全部记忆(库);
                let mut 输出 = format!("== 查全部记忆（{} 条）==\n", 全部.len());
                for s in &全部 {
                    输出.push_str(&format!("  {}\n", s));
                }
                命令结果::成功(输出)
            }
            "播种" => {
                let 补数 = 播种格位36(库);
                命令结果::成功(format!(
                    "== 播种格位36 完成 ==\n库：{}\n本次补齐 {} 格位（AI 生成无玉玺可执行；界主确认可盖玉玺）",
                    库, 补数
                ))
            }
            "精炼" => {
                let 内容 = if 参数.len() >= 2 && !参数[1].is_empty() {
                    参数[1]
                } else {
                    return 命令结果::失败(1, "用法：记忆 精炼 <执行结果> [库路径]");
                };
                // 精炼的参数[1]是内容，库路径在参数[2]（区别于 三档/查/播种 的参数[1]）
                let 库 = if 参数.len() >= 3 && !参数[2].is_empty() {
                    参数[2]
                } else {
                    默认记忆库路径
                };
                let 任务标识 = 内容.chars().take(12).collect::<String>();
                let ids = 地道精炼(库, &任务标识, 内容);
                命令结果::成功(format!(
                    "== 地道精炼 完成 ==\n库：{}\n提炼 {} 条（教训/机制/波及 → 经历 格位，无玉玺可执行）",
                    库,
                    ids.len()
                ))
            }
            "会话" => {
                let 子 = 参数.get(1).copied().unwrap_or("");
                let 任务 = 参数.get(2).copied().unwrap_or("");
                let 库 = if 参数.len() >= 4 && !参数[3].is_empty() {
                    参数[3]
                } else {
                    默认记忆库路径
                };
                match 子 {
                    "查" => {
                        if 任务.is_empty() {
                            return 命令结果::失败(
                                1,
                                "用法：记忆 会话 查 <任务标识> [库路径]",
                            );
                        }
                        let 会话 = 查会话(库, 任务);
                        let mut 输出 =
                            format!("== 查会话（{} 行）==\n任务：{}\n", 会话.len(), 任务);
                        for s in &会话 {
                            输出.push_str(&format!("  {}\n", s));
                        }
                        命令结果::成功(输出)
                    }
                    "归档" => {
                        let 结果 = 参数.get(3).copied().unwrap_or("");
                        if 任务.is_empty() || 结果.is_empty() {
                            return 命令结果::失败(
                                1,
                                "用法：记忆 会话 归档 <任务标识> <执行结果> [库路径]",
                            );
                        }
                        let 库 = if 参数.len() >= 5 && !参数[4].is_empty() {
                            参数[4]
                        } else {
                            默认记忆库路径
                        };
                        let n = 终点归档(库, 任务, 结果);
                        命令结果::成功(format!(
                            "== 会话终点归档 完成 ==\n任务：{}\n精炼 {} 条（教训/机制/波及 → 经历 格位，无玉玺）",
                            任务, n
                        ))
                    }
                    _ => 命令结果::失败(
                        1,
                        "用法：记忆 会话 <查|归档> <任务标识> [执行结果] [库路径]",
                    ),
                }
            }
            "统计" => {
                let 库 = if 参数.len() >= 2 && !参数[1].is_empty() {
                    参数[1]
                } else {
                    默认记忆库路径
                };
                match 格位统计(库) {
                    Ok(行) => {
                        let mut 输出 = "== 格位统计 ==\n".to_string();
                        for l in &行 {
                            输出.push_str(&format!("  {}\n", l));
                        }
                        命令结果::成功(输出)
                    }
                    Err(e) => 命令结果::失败(1, format!("格位统计失败：{}", e)),
                }
            }
            "完成度" => {
                let 刻度 = 参数.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                let 依据 = 参数.get(2).copied().unwrap_or("");
                if 依据.is_empty() {
                    return 命令结果::失败(1, "用法：记忆 完成度 <刻度1-5> <依据> [库路径]");
                }
                let 库 = if 参数.len() >= 4 && !参数[3].is_empty() {
                    参数[3]
                } else {
                    默认记忆库路径
                };
                match 完成度自评(库, "命令行", 刻度, 依据) {
                    Ok(_) => 命令结果::成功(format!(
                        "== 完成度自评 完成 ==\n刻度：{}/5\n库：{}",
                        刻度, 库
                    )),
                    Err(e) => 命令结果::失败(1, format!("完成度自评失败：{}", e)),
                }
            }
            "事件" => {
                let 子 = 参数.get(1).copied().unwrap_or("");
                match 子 {
                    "记录" => {
                        let 内容 = 参数.get(2).copied().unwrap_or("");
                        if 内容.is_empty() {
                            return 命令结果::失败(
                                1,
                                "用法：记忆 事件 记录 <内容> [类型] [库路径]",
                            );
                        }
                        let 类型 = 参数.get(3).copied().unwrap_or("事件");
                        let 库 = if 参数.len() >= 5 && !参数[4].is_empty() {
                            参数[4]
                        } else {
                            默认记忆库路径
                        };
                        match 事件流_记录(库, 类型, 内容) {
                            Ok(n) => 命令结果::成功(format!(
                                "== 事件记录 ==\n序号：{}\n库：{}",
                                n, 库
                            )),
                            Err(e) => 命令结果::失败(1, format!("事件记录失败：{}", e)),
                        }
                    }
                    "读取" => {
                        let 起 = 参数.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
                        let 止 = 参数.get(3).and_then(|s| s.parse().ok()).unwrap_or(i64::MAX);
                        let 库 = if 参数.len() >= 5 && !参数[4].is_empty() {
                            参数[4]
                        } else {
                            默认记忆库路径
                        };
                        match 事件流_读取(库, 起, 止) {
                            Ok(行) => {
                                let mut 输出 = format!("== 事件流（{} 条）==\n", 行.len());
                                for l in &行 {
                                    输出.push_str(&format!("  {}\n", l));
                                }
                                命令结果::成功(输出)
                            }
                            Err(e) => 命令结果::失败(1, format!("事件读取失败：{}", e)),
                        }
                    }
                    "全部" => {
                        let 库 = if 参数.len() >= 3 && !参数[2].is_empty() {
                            参数[2]
                        } else {
                            默认记忆库路径
                        };
                        match 事件流_全部(库) {
                            Ok(行) => {
                                let mut 输出 = format!("== 事件流（{} 条）==\n", 行.len());
                                for l in &行 {
                                    输出.push_str(&format!("  {}\n", l));
                                }
                                命令结果::成功(输出)
                            }
                            Err(e) => 命令结果::失败(1, format!("事件读取失败：{}", e)),
                        }
                    }
                    _ => 命令结果::失败(1, "用法：记忆 事件 <记录|读取|全部> [参数...]"),
                }
            }
            _ => 命令结果::失败(
                1,
                "用法：记忆 <三档|查|播种|精炼|会话|事件|完成度|统计> [参数] [库路径]",
            ),
        }
    }
}

pub fn 分发(参数: &[&str]) -> 命令结果 {
    if 参数.is_empty() {
        return 帮助命令.执行(&[]);
    }
    match 参数[0] {
        "帮助" | "--help" | "-h" => 帮助命令.执行(&[]),
        "init" => Init命令.执行(&参数[1..]),
        "status" => Status命令.执行(&参数[1..]),
        "自检" => 自检命令.执行(&参数[1..]),
        "run" => Run命令.执行(&参数[1..]),
        "e2e" => 跑流水线_mock_llm("e2e-默认任务"),
        "跳层测试" => 跑流水线_跳层(),
        "反序测试" => 跑流水线_反序(),
        "循环测试" => 跑流水线_循环打回(),
        "记忆" => 记忆命令.执行(&参数[1..]),
        other => 命令结果::失败(1, format!("未知命令：{}（运行 '帮助'）", other)),
    }
}
