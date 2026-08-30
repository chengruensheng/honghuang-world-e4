//! 端到端实现-园 - 后端选择端到端（v4 阶段 17 + Round 9）
//!
//! 决策锚：260827-moxing_fu调用方集成（Round 9）
//! 关联文档：22-moxing_fu调用方集成-实施方案.md § 七、回归保证
//! falsifiable：
//!   - 跑流水线_mock_llm 默认走 MockLLM连接（向后兼容）
//!   - 跑流水线_真实_llm 走 moxing_fu::HTTP连接（无 key 时 fail loud 退出码 4，严禁降级 mock）

use super::super::super::真实_后端_选择_殿::{
    后端模式, 解析后端模式, 读端点配置, 读终裁温度, 读终裁采样次数,
};
use super::super::super::{命令结果, 退出码};
use super::召回::*;
use super::终裁判定::{
    多数裁决, 提取修订要求, 终裁结论, 解析终裁结论, 质量门禁校验
};
use super::还债::*;

/// 端到端后端选择（v4 阶段 17 + Round 9）
///
/// 根据 LLM_BACKEND 环境变量选择后端：
/// - "real" + 有 LLM_API_KEY → moxing_fu::HTTP连接
/// - 其他 → MockLLM连接（默认 + 降级）
pub fn 跑流水线_mock_llm(任务标识: &str) -> 命令结果 {
    跑流水线_按后端(任务标识, 解析后端模式())
}

/// 端到端后端选择（指定 backend）
pub fn 跑流水线_真实_llm(任务标识: &str) -> 命令结果 {
    跑流水线_按后端(任务标识, 后端模式::真实)
}

/// 抽象连接枚举（绕开 LLM调用器<C> 单态化限制）
enum 连接抽象 {
    Mock(MockLLM连接),
    HTTP(moxing_fu::HTTP连接),
    /// 故障注入：测试超时/非2xx 契约时直接返回指定错误（确定性，不碰真实网络）
    #[cfg(test)]
    故障(moxing_fu::错误),
}

impl moxing_fu::模型连接 for 连接抽象 {
    fn 发送(
        &self,
        配置: &moxing_fu::LLM配置,
        请求: &moxing_fu::请求,
    ) -> Result<moxing_fu::响应, moxing_fu::错误> {
        match self {
            连接抽象::Mock(c) => c.发送(配置, 请求),
            连接抽象::HTTP(c) => c.发送(配置, 请求),
            #[cfg(test)]
            连接抽象::故障(e) => Err(e.clone()),
        }
    }
}

/// 产出最大令牌：执行层产出代码需更大输出窗口，默认 2048 会物理截断（终裁判「半成品」打回）
pub(crate) const 产出最大令牌: u32 = 8000;

fn 跑流水线_按后端(任务标识: &str, 模式: 后端模式) -> 命令结果 {
    use moxing_fu::LLM调用器;

    let mut 日志 = format!("[e2e 启动] 任务：{} 后端={:?}\n", 任务标识, 模式);

    // 交付回填库：真实/默认模式交付后回填到默认持久库；mock 不回填（避免污染测试库）
    let 回填库: Option<&str> = match &模式 {
        后端模式::真实 | 后端模式::默认 => Some(crate::默认记忆库路径),
        后端模式::Mock => None,
    };

    // 打回重试上限：真实/默认从端点配置读（env LLM_打回上限，默认 3 对齐契约「可循环打回≤3 轮」；mock 固定 3 测完整机制）；
    // mock 固定 3（测试验证完整打回重投机制）
    let 打回上限 = match &模式 {
        后端模式::真实 | 后端模式::默认 => 读端点配置().打回上限,
        后端模式::Mock => 3,
    };

    // 默认与真实均走真实 API（严禁 mock）；mock 仅显式 LLM_BACKEND=mock
    let 调用器: LLM调用器<连接抽象> = match 模式 {
        后端模式::真实 | 后端模式::默认 => {
            // 真实模式：尝试 moxing_fu::从环境变量构造()；失败 → fail loud（严禁降级 mock）
            match moxing_fu::从环境变量构造() {
                Some(池) => {
                    let 配置 = 读端点配置();
                    日志.push_str(&format!(
                        "[真实模式] 端点={} 超时={}ms 模型={}\n",
                        配置.端点, 配置.超时毫秒, 配置.模型
                    ));
                    LLM调用器::新建(池, 连接抽象::HTTP(moxing_fu::HTTP连接::新建()))
                }
                None => {
                    // 严禁 mock：无 key 直接失败（frozen outcome），带上启动日志便于定位任务
                    日志.push_str(
                        "真实模式无可用 API key（LLM_API_KEY 未设置）——严禁降级 mock，请配置 .env\n",
                    );
                    return 命令结果::失败(退出码::模型故障, 日志);
                }
            }
        }
        后端模式::Mock => {
            let 池 = 构建模拟池();
            LLM调用器::新建(池, 连接抽象::Mock(MockLLM连接::新建()))
        }
    };

    let mut 结果 = 跑流水线_按连接(任务标识, &调用器, 日志, 回填库, 打回上限);
    // 后端还债循环（契约 §六 编排层）：前端 fire-and-forget 交付后，后台线程独立驱动后端整理，前端永不阻塞
    // 真并发：前端 worker 交付即返回，后端 worker 独立线程异步还债；
    // main 侧 join 仅确保进程退出前还债完成（合并后端日志），不改前端不 await 后端语义。
    if let Some(库) = 回填库 {
        let 库克隆 = 库.to_string();
        let 后端调用器 = std::sync::Arc::new(调用器);
        let 后端线程 = std::thread::spawn(move || {
            let mut 后端日志 = String::new();
            后端_还债_至清零(&库克隆, 后端调用器.as_ref(), &mut 后端日志);
            后端日志
        });
        let 后端日志 = 后端线程.join().unwrap_or_default();
        结果.输出.push_str(&后端日志);
    }
    结果
}

/// 跑流水线_自举：自举任务单驱动，大罗产出代码 → 确定性落盘 → cargo 验证 → 准圣验收 → 终裁
///
/// 阶段 2 流水线接工具循环核心：LLM 只产代码意图，确定性程序走 gongju_fu 工具落盘并验证，
/// 验证结果作为准圣/终裁的真实验收证据（治理铁律 1 + 铁律 6 落地）。
/// 决策锚：260830 第一版自举规划（阶段 2）。
pub fn 跑流水线_自举(单: &renwu_zhixing_fu::自举任务单) -> 命令结果 {
    use moxing_fu::LLM调用器;

    let 模式 = 解析后端模式();
    let mut 日志 = format!(
        "[自举启动] 任务：{} 目标文件={} 验收命令={} 后端={:?}\n",
        单.标识, 单.目标文件, 单.验收命令, 模式
    );

    // 交付回填库：真实/默认模式交付后回填默认库；mock 不回填（避免污染测试库）
    let 回填库: Option<&str> = match &模式 {
        后端模式::真实 | 后端模式::默认 => Some(crate::默认记忆库路径),
        后端模式::Mock => None,
    };
    let 打回上限 = match &模式 {
        后端模式::真实 | 后端模式::默认 => 读端点配置().打回上限,
        后端模式::Mock => 3,
    };

    // 后端选择（复用现有：真实/默认走真实 API 严禁 mock，无 key fail loud）
    let 调用器: LLM调用器<连接抽象> = match 模式 {
        后端模式::真实 | 后端模式::默认 => match moxing_fu::从环境变量构造() {
            Some(池) => {
                let 配置 = 读端点配置();
                日志.push_str(&format!(
                    "[真实模式] 端点={} 模型={}\n",
                    配置.端点, 配置.模型
                ));
                LLM调用器::新建(池, 连接抽象::HTTP(moxing_fu::HTTP连接::新建()))
            }
            None => {
                日志.push_str(
                    "真实模式无可用 API key（LLM_API_KEY 未设置）——严禁降级 mock，请配置 .env\n",
                );
                return 命令结果::失败(退出码::模型故障, 日志);
            }
        },
        后端模式::Mock => {
            let 池 = 构建模拟池();
            LLM调用器::新建(池, 连接抽象::Mock(MockLLM连接::新建()))
        }
    };

    let mut 结果 = 跑流水线_自举核心(单, &调用器, 日志, 回填库, 打回上限);
    // 后端还债循环（契约 §六 编排层）：前端 fire-and-forget 交付后，后台线程独立驱动后端整理，前端永不阻塞
    if let Some(库) = 回填库 {
        let 库克隆 = 库.to_string();
        let 后端调用器 = std::sync::Arc::new(调用器);
        let 后端线程 = std::thread::spawn(move || {
            let mut 后端日志 = String::new();
            后端_还债_至清零(&库克隆, 后端调用器.as_ref(), &mut 后端日志);
            后端日志
        });
        let 后端日志 = 后端线程.join().unwrap_or_default();
        结果.输出.push_str(&后端日志);
    }
    结果
}

/// 自举核心循环：道祖→圣人→大罗（产出代码）→[确定性落盘+验证]→准圣（验收）→道祖终裁
fn 跑流水线_自举核心(
    单: &renwu_zhixing_fu::自举任务单,
    调用器: &moxing_fu::LLM调用器<连接抽象>,
    mut 日志: String,
    回填库: Option<&str>,
    打回上限: usize,
) -> 命令结果 {
    use moxing_fu::请求;

    // 模型可见⟺已记录：读持久库召回上下文（与玩具任务流水线同源）
    let 读库 = 回填库.unwrap_or(crate::默认记忆库路径);
    let 召回 = match 组装召回上下文_按路径(读库, 单.标识.as_str()) {
        Ok(记) => 记,
        Err(错) => return 命令结果::失败(退出码::模型故障, 错),
    };
    let 记忆文本 = 召回.任务开始.clone();

    let 池顺序 = ["道祖", "圣人", "大罗", "准圣"];
    let mut llm失败数 = 0;
    let 最大打回轮 = 打回上限;
    let mut 打回轮 = 0usize;
    let mut 打回上下文 = String::new();
    let mut 最终结论: 终裁结论 = 终裁结论::打回;
    // 自举执行证据：大罗代码落盘 + cargo 验证结果，注入准圣与终裁
    let mut 验证证据 = String::new();
    let mut 验证通过 = false;

    loop {
        let mut 上文 = String::new();
        for 池名 in 池顺序.iter() {
            let mut 消息列表 = 组装消息列表(池名, 单.标识.as_str(), &记忆文本, "");
            // 自举任务上下文：目标文件 + 需求 + 验收命令（大罗据此产出代码）
            消息列表.push(moxing_fu::消息::用户(format!(
                "自举任务单：目标文件={} 需求={} 验收命令={} 可证伪={}\n大罗须产出含「目标文件：」标记与代码围栏的可编译实现，标注 decided_by/falsifiable/implements/复现命令。",
                单.目标文件, 单.需求描述, 单.验收命令, 单.可证伪命题
            )));
            if !打回上下文.is_empty() {
                消息列表.push(moxing_fu::消息::用户(format!(
                    "上一轮打回：\n{}",
                    打回上下文
                )));
            }
            if !上文.is_empty() {
                消息列表.push(moxing_fu::消息::用户(
                    format!("上一角色产出：\n{}", 上文),
                ));
            }
            // 准圣注入验证证据（真实验收：看到落盘 + cargo 结果才验收）
            if *池名 == "准圣" && !验证证据.is_empty() {
                消息列表.push(moxing_fu::消息::系统(format!(
                    "自举执行证据（确定性程序落盘 + cargo 验证）：\n{}\n验收通过={}",
                    验证证据, 验证通过
                )));
            }
            let req = 请求::新建("", 消息列表).设最大token(产出最大令牌);
            match 调用_带重试(调用器, 池名, &req) {
                Ok(响应) => {
                    上文 = 响应.内容.clone();
                    日志.push_str(&format!("[LLM {}] {}\n", 池名, 响应.内容));
                    // 大罗产出后：确定性执行器落盘 + cargo 验证（治理铁律：LLM 产意图，确定程序执行）
                    if *池名 == "大罗" && !响应.内容.is_empty() {
                        let 代码 = crate::提取代码块(&响应.内容);
                        // 治理铁律：目标路径是任务单确定性锚，LLM 不得改写（防大罗写错路径落盘失败）
                        let 目标 = 单.目标文件.clone();
                        let 执行结果 = crate::自举执行(&代码, &目标, &单.验收命令);
                        验证通过 = 执行结果.验证通过;
                        验证证据 = format!(
                            "目标文件={}\n落盘：{}\n验证：{}",
                            目标, 执行结果.落盘结果, 执行结果.验证结果
                        );
                        日志.push_str(&format!("[自举执行] {}\n", 验证证据));
                    }
                }
                Err(e) => {
                    llm失败数 += 1;
                    日志.push_str(&format!(
                        "[LLM {} 错误·{}] {}\n",
                        池名,
                        错误归因(&e),
                        脱敏(e.to_string())
                    ));
                }
            }
        }

        // 道祖终裁：注入验证证据 + 4 角色接力产出
        let mut 终裁消息 =
            组装消息列表("道祖", 单.标识.as_str(), &记忆文本, 召回.玉玺裁决.as_str());
        if !验证证据.is_empty() {
            终裁消息.push(moxing_fu::消息::系统(format!(
                "自举执行证据：\n{}\n验收通过={}",
                验证证据, 验证通过
            )));
        }
        if !上文.is_empty() {
            终裁消息.push(moxing_fu::消息::用户(format!(
                "以上为 4 角色接力产出，请道祖终裁：通过或打回，并给出理由。\n{}",
                上文
            )));
        }
        let 采样次数 = 读终裁采样次数();
        let mut 裁决列表: Vec<终裁结论> = Vec::new();
        let mut 终裁内容 = String::new();
        for 采样序 in 0..采样次数 {
            match 调用_带重试(
                调用器,
                "道祖",
                &请求::新建("", 终裁消息.clone())
                    .设最大token(产出最大令牌)
                    .设温度(读终裁温度()),
            ) {
                Ok(响应) => {
                    终裁内容 = 响应.内容.clone();
                    日志.push_str(&format!(
                        "[道祖终裁·采样{}/{}] {}\n",
                        采样序 + 1,
                        采样次数,
                        响应.内容
                    ));
                    裁决列表.push(解析终裁结论(&响应.内容));
                }
                Err(e) => {
                    llm失败数 += 1;
                    日志.push_str(&format!(
                        "[道祖终裁错误·采样{}/{}·{}] {}\n",
                        采样序 + 1,
                        采样次数,
                        错误归因(&e),
                        脱敏(e.to_string())
                    ));
                }
            }
        }
        if 裁决列表.is_empty() {
            break;
        }
        最终结论 = 多数裁决(&裁决列表);
        if 最终结论 == 终裁结论::通过 {
            // 自举硬门槛：验证必须通过（cargo 全绿），否则改判打回
            if !验证通过 {
                日志.push_str("[自举门禁] 终裁通过但 cargo 验证未通过，改判打回\n");
                最终结论 = 终裁结论::打回;
            } else {
                // 质量门禁三必填 + 产出评分
                let 缺项 = 质量门禁校验(&上文);
                if 缺项.is_empty() {
                    日志.push_str(&format!("[产出评分] {}/100\n", 产出评分(&上文)));
                    break;
                }
                日志.push_str(&format!(
                    "[质量门禁] 终裁通过但产出缺必填项 {:?}，改判打回\n",
                    缺项
                ));
                最终结论 = 终裁结论::打回;
            }
        }
        打回轮 += 1;
        if 打回轮 >= 最大打回轮 {
            日志.push_str(&format!("[打回] 已打回 {} 轮达到上限，停止重投\n", 打回轮));
            let _ = crate::事件流_记录(读库, "打回达上限", 单.标识.as_str());
            break;
        }
        let 下探 = 召回_遇阻下探(读库).join("\n");
        let 修订要求 = 提取修订要求(&终裁内容);
        // 上一轮执行证据快照注入打回上下文：大罗重投须知道 cargo 失败详情（断言失败/编译错误），
        // 终裁报告未必复述失败细节（尤其硬门槛改判打回时终裁文本只说通过）——证据是唯一事实源
        let 证据快照 = if 验证证据.is_empty() {
            "（无）".to_string()
        } else {
            format!("{}\n验收通过={}", 验证证据, 验证通过)
        };
        打回上下文 = format!(
            "打回理由（结构化修订要求）：\n{}\n\n上一轮自举执行证据（确定性落盘 + cargo 验证）：\n{}\n\n下探召回：\n{}",
            修订要求, 证据快照, 下探
        );
        let _ = crate::事件流_记录(读库, "打回重投", 单.标识.as_str());
        // 打回重投前清空自举执行证据：防新一轮大罗调用失败时残留上一轮旧值误导准圣/终裁
        验证证据 = String::new();
        验证通过 = false;
        日志.push_str(&format!(
            "[打回重投] 第 {} 轮，注入结构化修订要求\n",
            打回轮
        ));
    }

    // 交付才回填（契约 §四 + §六）：终裁通过 → 前端登记交付（债务+1）
    if llm失败数 == 0 {
        if let Some(库) = 回填库 {
            if 最终结论 == 终裁结论::通过 {
                match 前端_登记并交付(库, 单.标识.as_str()) {
                    Ok(()) => {
                        日志.push_str("[交付] 登记并交付（债务+1）\n");
                        let _ = crate::事件流_记录(库, "终裁通过交付", 单.标识.as_str());
                    }
                    Err(e) => 日志.push_str(&format!("[交付失败] {}\n", 脱敏(e))),
                }
            } else {
                日志.push_str("[交付] 终裁判定打回，不交付不回填\n");
                let _ = crate::事件流_记录(库, "终裁打回", 单.标识.as_str());
            }
        }
    }

    日志.push_str(&format!("[完成] 自举任务全链路 LLM 失败数={}\n", llm失败数));
    if llm失败数 > 0 {
        命令结果::失败(退出码::模型故障, 日志)
    } else if 最终结论 == 终裁结论::打回 {
        // 打回达上限 = 终裁未通过，不得返回「成功」（批量汇总须能识别任务失败）
        命令结果::失败(退出码::状态机违规, 日志)
    } else {
        命令结果::成功(日志)
    }
}

/// 脱敏：错误信息里不得出现 API 密钥（Bearer token）
/// 策略：把 "Bearer " 之后的 token 替换为 "***"（直至空白或引号）
pub(crate) fn 脱敏(原文: String) -> String {
    match 原文.find("Bearer ") {
        Some(头) => {
            let 起点 = 头 + "Bearer ".len();
            let 尾 = 原文[起点..]
                .find(|c| ['\'', '"', ' ', '\n', '\r'].contains(&c))
                .map(|i| 起点 + i)
                .unwrap_or(原文.len());
            format!("{}Bearer ***{}", &原文[..头], &原文[尾..])
        }
        None => 原文,
    }
}

/// 是否可重试：仅网络抖动类错误（超时 / 5xx / 网络传输 0 / 解析）幂等重试；业务错误（额度耗尽/配置/鉴权/4xx）立即失败 fail loud
fn 是否可重试(错误: &moxing_fu::错误) -> bool {
    match 错误 {
        moxing_fu::错误::超时 => true,
        moxing_fu::错误::HTTP错误 { 状态码, .. } => *状态码 >= 500 || *状态码 == 0,
        moxing_fu::错误::解析错误(_) => true,
        _ => false,
    }
}

/// 错误归因标签：把 moxing_fu::错误 归为可定位类别（超时/5xx/4xx/额度/鉴权/配置/解析/网络），供失败日志分级定位
fn 错误归因(错误: &moxing_fu::错误) -> &'static str {
    match 错误 {
        moxing_fu::错误::超时 => "超时",
        moxing_fu::错误::HTTP错误 { 状态码, .. } if *状态码 >= 500 => "5xx",
        moxing_fu::错误::HTTP错误 { 状态码, .. } if *状态码 >= 400 => "4xx",
        moxing_fu::错误::HTTP错误 { 状态码, .. } if *状态码 == 0 => "网络",
        moxing_fu::错误::HTTP错误 { .. } => "HTTP",
        moxing_fu::错误::额度耗尽 => "额度",
        moxing_fu::错误::鉴权失败 => "鉴权",
        moxing_fu::错误::配置错误(_) => "配置",
        moxing_fu::错误::解析错误(_) => "解析",
    }
}

/// LLM 调用容错（契约生产级）：网络抖动类错误幂等重试一次，其余 fail loud
/// 幂等性：同请求重发不改变语义（POST 补全）；业务错误（额度/鉴权/4xx）重试无益，立即返回
pub(crate) fn 调用_带重试<C: moxing_fu::模型连接>(
    调用器: &moxing_fu::LLM调用器<C>,
    池名: &str,
    请求: &moxing_fu::请求,
) -> Result<moxing_fu::响应, moxing_fu::错误> {
    match 调用器.调用(池名, 请求) {
        Ok(响应) => Ok(响应),
        Err(首次错误) => {
            if 是否可重试(&首次错误) {
                调用器.调用(池名, 请求)
            } else {
                Err(首次错误)
            }
        }
    }
}

/// 核心执行：任务判定 + 记忆注入 + 4 分类循环 + 累积 llm失败数 + fail loud（可注入连接，供故障契约测试）
fn 跑流水线_按连接(
    任务标识: &str,
    调用器: &moxing_fu::LLM调用器<连接抽象>,
    mut 日志: String,
    回填库: Option<&str>,
    打回上限: usize,
) -> 命令结果 {
    use moxing_fu::请求;
    use renwu_zhixing_fu::{任务, 分类_机械判定, 角色分类};

    let 任务_obj = 任务 {
        标识: 任务标识.to_string(),
        分类: 角色分类::道祖级,
        描述: format!("e2e 任务：{}", 任务标识),
        decided_by: "界主".to_string(),
    };
    let _ = 分类_机械判定(&任务_obj, 角色分类::道祖级);

    // 模型可见⟺已记录：四时机召回上下文（契约 §五），流水线组装 LLM 请求前读持久库 + 反向断言（可审计）
    // 读库 = 回填库（Some）或默认库——读写同一库，测试用临时回填库隔离，不竞争默认库
    let 读库 = 回填库.unwrap_or(crate::默认记忆库路径);
    let 召回 = match 组装召回上下文_按路径(读库, 任务标识) {
        Ok(记) => 记,
        Err(错) => return 命令结果::失败(退出码::模型故障, 错),
    };
    let 记忆文本 = 召回.任务开始.clone();
    // 时机① 任务开始关键词召回记忆列表（正向断言用）
    let 记忆列表: Vec<String> = 召回
        .任务开始
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let 池顺序 = ["道祖", "圣人", "大罗", "准圣"];
    let mut llm失败数 = 0;
    // 打回重试循环（契约流水线闭环：终裁打回 → 时机②遇阻下探 → 重投 ≤3 轮）
    let 最大打回轮 = 打回上限;
    let mut 打回轮 = 0usize;
    let mut 打回上下文 = String::new();
    let mut 最终结论: 终裁结论 = 终裁结论::打回;
    loop {
        // 角色接力：上一角色产出作为下一角色上下文（道祖→圣人→大罗→准圣，不可跳层不可反序）
        let mut 上文 = String::new();
        for 池名 in 池顺序.iter() {
            // 时机③：准圣额外注入验收对比召回（历史验收块）
            let 额外: &str = if *池名 == "准圣" {
                召回.验收对比.as_str()
            } else {
                ""
            };
            let mut 消息列表 = 组装消息列表(池名, 任务标识, &记忆文本, 额外);
            if !打回上下文.is_empty() {
                消息列表.push(moxing_fu::消息::用户(format!(
                    "上一轮打回：\n{}",
                    打回上下文
                )));
            }
            if !上文.is_empty() {
                消息列表.push(moxing_fu::消息::用户(
                    format!("上一角色产出：\n{}", 上文),
                ));
            }
            // 正向断言（拦截点：调用器.调用 之前）：时机①读到的每条记忆必须出现在消息列表
            if let Err(错) = 断言注入到位(&记忆列表, &消息列表) {
                return 命令结果::失败(退出码::模型故障, 错);
            }
            let req = 请求::新建("", 消息列表).设最大token(产出最大令牌);
            match 调用_带重试(调用器, 池名, &req) {
                Ok(响应) => {
                    上文 = 响应.内容.clone();
                    日志.push_str(&format!("[LLM {}] {}\n", 池名, 响应.内容));
                }
                Err(e) => {
                    llm失败数 += 1;
                    日志.push_str(&format!(
                        "[LLM {} 错误·{}] {}\n",
                        池名,
                        错误归因(&e),
                        脱敏(e.to_string())
                    ));
                }
            }
        }

        // 道祖终裁：4 角色接力产出后追加道祖二次调用，产出通过/打回结论（流水线闭环）
        // 时机④：终裁额外注入玉玺裁决召回（玉玺块）
        let mut 终裁消息 = 组装消息列表("道祖", 任务标识, &记忆文本, 召回.玉玺裁决.as_str());
        if !上文.is_empty() {
            终裁消息.push(moxing_fu::消息::用户(format!(
                "以上为 4 角色接力产出，请道祖终裁：通过或打回，并给出理由。\n{}",
                上文
            )));
        }
        // 终裁多次采样取多数（任务 4）：对冲 LLM 采样非确定；默认 1 次=单次判定，实验设 3 次=多数裁决
        let 采样次数 = 读终裁采样次数();
        let mut 裁决列表: Vec<终裁结论> = Vec::new();
        let mut 终裁内容 = String::new();
        for 采样序 in 0..采样次数 {
            match 调用_带重试(
                调用器,
                "道祖",
                &请求::新建("", 终裁消息.clone())
                    .设最大token(产出最大令牌)
                    .设温度(读终裁温度()),
            ) {
                Ok(响应) => {
                    终裁内容 = 响应.内容.clone();
                    日志.push_str(&format!(
                        "[道祖终裁·采样{}/{}] {}\n",
                        采样序 + 1,
                        采样次数,
                        响应.内容
                    ));
                    裁决列表.push(解析终裁结论(&响应.内容));
                }
                Err(e) => {
                    llm失败数 += 1;
                    日志.push_str(&format!(
                        "[道祖终裁错误·采样{}/{}·{}] {}\n",
                        采样序 + 1,
                        采样次数,
                        错误归因(&e),
                        脱敏(e.to_string())
                    ));
                }
            }
        }

        // 全采样失败 = LLM 故障 → fail loud 不重投（区别于「终裁打回」：打回是成功调用后的裁决，故障是调用失败）
        if 裁决列表.is_empty() {
            break;
        }

        // 多数裁决（采样次数=1 时即单次判定，行为与旧版等价）
        最终结论 = 多数裁决(&裁决列表);
        if 采样次数 > 1 {
            日志.push_str(&format!(
                "[多数裁决] {}/{} 采样通过 → {:?}\n",
                裁决列表.iter().filter(|c| **c == 终裁结论::通过).count(),
                采样次数,
                最终结论
            ));
        }
        if 最终结论 == 终裁结论::通过 {
            // 质量门禁三必填：终裁通过但产出缺 decided_by/falsifiable → 改判打回（空壳产出不得通过）
            let 缺项 = 质量门禁校验(&上文);
            if 缺项.is_empty() {
                // 产出抽样评分：通过后记录量化质量（阶段 D 出口标准）
                日志.push_str(&format!("[产出评分] {}/100\n", 产出评分(&上文)));
                break;
            }
            日志.push_str(&format!(
                "[质量门禁] 终裁通过但产出缺必填项 {:?}，改判打回\n",
                缺项
            ));
            最终结论 = 终裁结论::打回;
        }
        // 打回：时机②遇阻下探召回 + 打回理由 → 重投
        打回轮 += 1;
        if 打回轮 >= 最大打回轮 {
            日志.push_str(&format!("[打回] 已打回 {} 轮达到上限，停止重投\n", 打回轮));
            // 治理事件流审计：打回达上限 append-only 可追溯
            let _ = crate::事件流_记录(读库, "打回达上限", 任务标识);
            break;
        }
        let 下探 = 召回_遇阻下探(读库).join("\n");
        // 打回理由结构化：提取可执行修订要求（致命修复/验收判据/复现命令），而非全文注入
        let 修订要求 = 提取修订要求(&终裁内容);
        打回上下文 = format!(
            "打回理由（结构化修订要求）：\n{}\n\n下探召回（历史教训/步骤）：\n{}",
            修订要求, 下探
        );
        // 治理事件流审计：打回重投 append-only 可追溯
        let _ = crate::事件流_记录(读库, "打回重投", 任务标识);
        日志.push_str(&format!(
            "[打回重投] 第 {} 轮，注入结构化修订要求\n",
            打回轮
        ));
    }

    // 交付才回填（契约 §四 + §六 双工）：终裁通过 → 前端登记/交付（fire-and-forget，债务+1）
    // 后端还债由编排层（跑流水线_按后端）独立驱动，前端永不阻塞
    if llm失败数 == 0 {
        if let Some(库) = 回填库 {
            if 最终结论 == 终裁结论::通过 {
                // 前端交付：只登记/交付产生债务，提炼回填交后端整理
                match 前端_登记并交付(库, 任务标识) {
                    Ok(()) => {
                        日志.push_str("[交付] 登记并交付（债务+1）\n");
                        // 治理事件流审计（元三治·治强）：终裁通过交付 append-only 时序事实，可追溯
                        let _ = crate::事件流_记录(库, "终裁通过交付", 任务标识);
                    }
                    Err(e) => 日志.push_str(&format!("[交付失败] {}\n", 脱敏(e))),
                }
            } else {
                日志.push_str("[交付] 终裁判定打回，不交付不回填\n");
                // 治理事件流审计（元三治·治强）：终裁打回 append-only 时序事实，可追溯
                let _ = crate::事件流_记录(库, "终裁打回", 任务标识);
            }
        }
    }

    日志.push_str(&format!(
        "[完成] e2e 任务全链路（追问 + 4 分类 LLM + 道祖终裁）LLM 失败数={}\n",
        llm失败数
    ));
    if llm失败数 > 0 {
        // 真实 LLM 故障 fail loud：任一角色调用失败，流水线不得假装成功（frozen outcome）
        命令结果::失败(退出码::模型故障, 日志)
    } else if 最终结论 == 终裁结论::打回 {
        // 打回达上限 = 终裁未通过，不得返回「成功」（与自举版对齐，批量汇总须能识别任务失败）
        命令结果::失败(退出码::状态机违规, 日志)
    } else {
        命令结果::成功(日志)
    }
}

/// 组装单个池的消息列表：记忆上下文（系统，头部）→ 角色卡（系统，4 分类差异化）→ 任务（用户）→ 召回块（尾部）
fn 组装消息列表(
    池名: &str,
    任务标识: &str,
    记忆文本: &str,
    召回额外: &str,
) -> Vec<moxing_fu::消息> {
    let mut 列表 = vec![moxing_fu::消息::系统(角色卡文案(池名))];
    if !记忆文本.is_empty() {
        列表.insert(
            0,
            moxing_fu::消息::系统(format!(
                "相关记忆：
{}",
                记忆文本
            )),
        );
    }
    列表.push(moxing_fu::消息::用户(format!("任务：{}", 任务标识)));
    if !召回额外.is_empty() {
        列表.push(moxing_fu::消息::系统(format!(
            "召回上下文：
{}",
            召回额外
        )));
    }
    列表
}

/// 4 分类角色卡文案：单一来源为 liushuixian_qudong_fu::分类::角色卡 的文案字段（职责差异化 + 命名合规铁律）
fn 角色卡文案(池名: &str) -> String {
    use liushuixian_qudong_fu::分类;
    let 分类 = match 池名 {
        "道祖" => 分类::道祖级,
        "圣人" => 分类::圣人级,
        "大罗" => 分类::大罗金仙级,
        "准圣" => 分类::准圣级,
        other => return format!("你是 {} 角色卡", other),
    };
    分类.角色卡().文案.to_string()
}

/// 产出抽样评分：按决策契约三要素（decided_by/falsifiable/implements）+ 可复现 + 非空壳 打分（0-100）
/// 阶段 D 出口标准：空壳产出 0 分，三要素齐全 + 可复现 = 100 分（命名合规由静态门禁脚本保证）
fn 产出评分(产出: &str) -> u32 {
    let 有决策 = 产出.contains("decided_by") || 产出.contains("决策者");
    let 有可证伪 = 产出.contains("falsifiable") || 产出.contains("可证伪");
    let 有锚 = 产出.contains("implements");
    let 有复现 = 产出.contains("复现") || 产出.contains("验证") || 产出.contains("cargo test");
    let 非空壳 = 有决策 || 有可证伪 || 有锚 || 有复现;
    if !非空壳 {
        return 0;
    }
    20 + (u32::from(有决策) + u32::from(有可证伪) + u32::from(有锚) + u32::from(有复现)) * 20
}

/// 正向可审计断言：读到的每条记忆必须出现在消息列表（注入确实发生，可见⟺已记录）
fn 断言注入到位(记忆: &[String], 消息列表: &[moxing_fu::消息]) -> Result<(), String> {
    for 条 in 记忆 {
        let 内容 = 条.split_once("] ").map(|(_, 内)| 内).unwrap_or(条.as_str());
        if !消息列表.iter().any(|m| m.内容.contains(内容)) {
            return Err(format!("正向断言失败：读到的记忆未注入消息列表：{}", 条));
        }
    }
    Ok(())
}

/// 构造 Mock 4 分类 LLM 池
fn 构建模拟池() -> moxing_fu::LLM池 {
    use moxing_fu::{LLM池, LLM配置};
    let mut 池 = LLM池::新建();
    let mock配置 = LLM配置::假配置("mock-model");
    池.设("道祖", mock配置.clone()).unwrap();
    池.设("圣人", mock配置.clone()).unwrap();
    池.设("准圣", mock配置.clone()).unwrap();
    池.设("大罗", mock配置).unwrap();
    池
}

pub struct MockLLM连接 {
    pub 响应内容: String,
    /// 最近一次收到的请求（审计捕获：模型可见⟺已记录 正向断言的可测试支撑）
    pub 最近请求: std::sync::Mutex<Option<moxing_fu::请求>>,
    /// 首次失败注入：第一次发送返回该错误并清空，后续正常（测「重试后成功」）
    pub 首次失败: std::sync::Mutex<Option<moxing_fu::错误>>,
    /// 终裁响应：道祖终裁请求（含「道祖终裁」提示）专用响应，默认显式通过（终裁质量：沉默=打回，mock 默认明确裁决）
    pub 终裁响应: String,
}
impl MockLLM连接 {
    pub fn 新建() -> Self {
        Self {
            响应内容: "decided_by=ai助手 falsifiable=测试全绿 implements=术 复现=cargo test [mock LLM 响应]".to_string(),
            最近请求: std::sync::Mutex::new(None),
            首次失败: std::sync::Mutex::new(None),
            终裁响应: "道祖终裁：通过".to_string(),
        }
    }
}
impl moxing_fu::模型连接 for MockLLM连接 {
    fn 发送(
        &self,
        _配置: &moxing_fu::LLM配置,
        请求: &moxing_fu::请求,
    ) -> Result<moxing_fu::响应, moxing_fu::错误> {
        // 首次失败注入：只失败一次后清空，后续正常（供「重试后成功」测试）
        if let Ok(mut 首次) = self.首次失败.lock() {
            if let Some(错) = 首次.take() {
                return Err(错);
            }
        }
        // 审计捕获：请求存入字段（最近请求）+ thread_local 槽（供测试断言消息列表含注入记忆）
        if let Ok(mut 槽) = self.最近请求.lock() {
            *槽 = Some(请求.clone());
        }
        记录最近请求(请求);
        // 终裁请求（含「道祖终裁」提示）→ 终裁响应；否则 4 角色产出响应
        let 是终裁 = 请求.消息列表.iter().any(|m| m.内容.contains("道祖终裁"));
        let 内容 = if 是终裁 {
            &self.终裁响应
        } else {
            &self.响应内容
        };
        Ok(moxing_fu::响应::假响应(内容))
    }
}

// 审计捕获槽：MockLLM连接::发送 把最近请求写入 thread_local，
// 供单元测试断言「模型可见⟺已记录」（注入记忆必出现在发往 LLM 的真实请求消息列表）。
thread_local! {
    static 最近请求槽: std::cell::RefCell<Vec<moxing_fu::请求>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn 记录最近请求(请求: &moxing_fu::请求) {
    最近请求槽.with(|槽| 槽.borrow_mut().push(请求.clone()));
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn 环境锁() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    /// 清理所有 LLM 相关 env（避免测试间污染）
    fn 清空_env() {
        std::env::remove_var("LLM_BACKEND");
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_MODEL");
        std::env::remove_var("LLM_TIMEOUT_MS");
        std::env::remove_var("LLM_MODEL_DAOZU");
        std::env::remove_var("LLM_MODEL_SHENGREN");
        std::env::remove_var("LLM_MODEL_ZHUNSHENG");
        std::env::remove_var("LLM_MODEL_DALUO");
    }

    #[test]
    fn 测试_角色卡_4分类_职责差异化() {
        let 道祖 = 角色卡文案("道祖");
        let 圣人 = 角色卡文案("圣人");
        let 大罗 = 角色卡文案("大罗");
        let 准圣 = 角色卡文案("准圣");
        assert!(道祖.contains("决策"), "道祖卡应含决策职责");
        assert!(圣人.contains("设计"), "圣人卡应含设计职责");
        assert!(大罗.contains("执行"), "大罗卡应含执行职责");
        assert!(准圣.contains("验收"), "准圣卡应含验收职责");
        assert!(
            道祖.contains("规则格位"),
            "道祖卡规则应指向格位（职责与规则分离）"
        );
        assert!(
            大罗.contains("规则格位"),
            "大罗卡规则应指向格位（职责与规则分离）"
        );
        assert!(大罗.contains("逐项产出"), "大罗卡应含逐项交付要求");
        assert!(大罗.contains("不得遗漏"), "大罗卡应含不得遗漏交付项");
        assert!(
            准圣.contains("规则格位"),
            "准圣卡规则应指向格位（职责与规则分离）"
        );
        assert!(准圣.contains("差异对比"), "准圣卡应含验收对比差异标注要求");
        assert!(准圣.contains("新增问题"), "准圣卡应含四类差异标注");
        assert_ne!(道祖, 圣人, "角色卡应互不相同");
        assert_ne!(圣人, 大罗, "角色卡应互不相同");
        assert_ne!(大罗, 准圣, "角色卡应互不相同");
    }

    #[test]
    fn 测试_角色卡_未知池名_回退旧格式() {
        let 文案 = 角色卡文案("未知");
        assert_eq!(文案, "你是 未知 角色卡");
    }

    #[test]
    fn 断言可重建_正向通过() {
        let 注入 = vec!["[执行] 36 格位闭环 API".to_string()];
        let 全部 = vec!["[执行·工具] 36 格位闭环 API".to_string()];
        assert!(断言可重建(&注入, &全部).is_ok());
    }

    #[test]
    fn 断言可重建_反向失败() {
        let 注入 = vec!["[执行] 库中不存在的记忆".to_string()];
        let 全部 = vec!["[执行·工具] 36 格位闭环 API".to_string()];
        match 断言可重建(&注入, &全部) {
            Err(错) => assert!(错.contains("不可审计"), "错误信息应含「不可审计」：{}", 错),
            Ok(_) => panic!("库中不存在的内容必须判定不可审计"),
        }
    }

    #[test]
    fn 组装记忆上下文_已知条目可重建() {
        let 路径 = std::env::temp_dir().join(format!("洪荒记忆测试_{}.sq3", std::process::id()));
        let 路径_str = 路径.to_str().unwrap();
        // 空库自动种子落盘（含 执行/工具 "36 格位闭环 API"）
        let _ = crate::读取任务相关记忆_持久(路径_str, "实现 Cargo 测试");
        let 记忆 =
            组装记忆上下文_按路径(路径_str, "实现 Cargo 测试").expect("注入记忆必须可被持久库重建");
        assert!(
            记忆.iter().any(|m| m.contains("执行")),
            "应命中执行总纲记忆：{:?}",
            记忆
        );
        let _ = std::fs::remove_file(&路径);
    }

    #[test]
    fn 组装消息列表_记忆注入头部() {
        let 记忆文本 = "[程序] 36 格位闭环 API\n[规则] 命名门禁规则";
        let 列表 = 组装消息列表("道祖", "实现 Cargo 测试", 记忆文本, "");
        // 首条为记忆系统消息
        assert!(matches!(列表[0].角色, moxing_fu::角色::系统));
        assert!(
            列表[0].内容.contains("36 格位闭环 API"),
            "首条应含记忆：{}",
            列表[0].内容
        );
        assert!(列表[0].内容.contains("命名门禁规则"));
        // 末条为用户任务
        let 用户 = 列表
            .iter()
            .find(|m| matches!(m.角色, moxing_fu::角色::用户))
            .expect("应有用户消息");
        assert!(用户.内容.contains("实现 Cargo 测试"));
    }

    #[test]
    fn 断言注入到位_正向通过() {
        let 记忆 = vec!["[执行] 36 格位闭环 API".to_string()];
        let 列表 = 组装消息列表("道祖", "任务", "[执行] 36 格位闭环 API", "");
        assert!(断言注入到位(&记忆, &列表).is_ok());
    }

    #[test]
    fn 断言注入到位_缺失记忆失败() {
        let 记忆 = vec!["[执行] 36 格位闭环 API".to_string()];
        // 消息列表不含该记忆（仅角色卡）→ 正向断言应失败
        let 列表 = vec![moxing_fu::消息::系统("你是 道祖 角色卡".to_string())];
        match 断言注入到位(&记忆, &列表) {
            Err(错) => assert!(错.contains("未注入"), "错误信息应含「未注入」：{}", 错),
            Ok(_) => panic!("缺失记忆必须判定正向断言失败"),
        }
    }

    #[test]
    fn 测试_流水线_请求消息列表_含注入记忆() {
        let _g = 环境锁();
        清空_env();
        std::env::set_var("LLM_BACKEND", "mock");
        最近请求槽.with(|槽| 槽.borrow_mut().clear());
        let r = 跑流水线_mock_llm("记忆注入审计测试");
        assert_eq!(r.退出码, 0, "流水线应成功：{}", r.输出);
        let 请求们 = 最近请求槽.with(|槽| 槽.borrow().clone());
        assert!(!请求们.is_empty(), "应捕获到发往 LLM 的请求");
        for 请求 in &请求们 {
            let 全文本 = 请求
                .消息列表
                .iter()
                .map(|m| m.内容.clone())
                .collect::<Vec<_>>()
                .join("\n");
            assert!(
                全文本.contains("36 格位闭环 API"),
                "请求消息列表应含注入记忆：{}",
                全文本
            );
        }
    }

    #[test]
    fn 测试_角色接力_上一角色产出注入下一角色() {
        let _g = 环境锁();
        清空_env();
        std::env::set_var("LLM_BACKEND", "mock");
        最近请求槽.with(|槽| 槽.borrow_mut().clear());
        let r = 跑流水线_mock_llm("接力传递审计");
        assert_eq!(r.退出码, 0, "流水线应成功：{}", r.输出);
        let 请求们 = 最近请求槽.with(|槽| 槽.borrow().clone());
        assert_eq!(
            请求们.len(),
            5,
            "应捕获 4 角色 + 道祖终裁请求：{}",
            请求们.len()
        );
        // 链首（道祖）不应有「上一角色产出」
        let 道祖消息 = 请求们[0]
            .消息列表
            .iter()
            .map(|m| m.内容.clone())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !道祖消息.contains("上一角色产出"),
            "道祖是链首不应有上文：{}",
            道祖消息
        );
        // 第 2 角色（圣人）应收到道祖产出作为「上一角色产出」
        let 圣人消息 = 请求们[1]
            .消息列表
            .iter()
            .map(|m| m.内容.clone())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            圣人消息.contains("上一角色产出"),
            "圣人应收到道祖产出：{}",
            圣人消息
        );
        assert!(
            圣人消息.contains("[mock LLM 响应]"),
            "上文应含道祖 mock 响应：{}",
            圣人消息
        );
        // 道祖终裁（第 5 个请求）应收到「终裁」提示 + 4 角色接力产出
        let 终裁消息 = 请求们[4]
            .消息列表
            .iter()
            .map(|m| m.内容.clone())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            终裁消息.contains("道祖终裁"),
            "终裁请求应含终裁提示：{}",
            终裁消息
        );
        assert!(
            终裁消息.contains("[mock LLM 响应]"),
            "终裁应收到 4 角色接力产出：{}",
            终裁消息
        );
    }

    #[test]
    fn 测试_契约_超时_退出码4() {
        let _g = 环境锁();
        清空_env();
        let 调用器 =
            moxing_fu::LLM调用器::新建(构建模拟池(), 连接抽象::故障(moxing_fu::错误::超时));
        let r = 跑流水线_按连接("超时契约", &调用器, String::new(), None, 3);
        assert_eq!(r.退出码, 4, "超时应 fail loud：{}", r.输出);
        assert!(r.输出.contains("[LLM 道祖 错误·"), "应记录错误：{}", r.输出);
        assert!(r.输出.contains("请求超时"), "应含超时错误文本：{}", r.输出);
        assert!(
            r.输出.contains("LLM 失败数=5"),
            "4 角色 + 终裁全失败：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[道祖终裁错误·"),
            "终裁失败也应记录：{}",
            r.输出
        );
    }

    #[test]
    fn 测试_契约_非2xx_401_退出码4() {
        let _g = 环境锁();
        清空_env();
        let 调用器 = moxing_fu::LLM调用器::新建(
            构建模拟池(),
            连接抽象::故障(moxing_fu::错误::HTTP错误 {
                状态码: 401,
                原因: "Unauthorized".to_string(),
            }),
        );
        let r = 跑流水线_按连接("401契约", &调用器, String::new(), None, 3);
        assert_eq!(r.退出码, 4, "401 应 fail loud：{}", r.输出);
        assert!(r.输出.contains("[LLM 道祖 错误·"));
        assert!(r.输出.contains("HTTP 错误 401"), "应含 401：{}", r.输出);
    }

    #[test]
    fn 测试_契约_非2xx_500_退出码4() {
        let _g = 环境锁();
        清空_env();
        let 调用器 = moxing_fu::LLM调用器::新建(
            构建模拟池(),
            连接抽象::故障(moxing_fu::错误::HTTP错误 {
                状态码: 500,
                原因: "Internal Server Error".to_string(),
            }),
        );
        let r = 跑流水线_按连接("500契约", &调用器, String::new(), None, 3);
        assert_eq!(r.退出码, 4, "500 应 fail loud：{}", r.输出);
        assert!(r.输出.contains("[LLM 道祖 错误·"));
        assert!(r.输出.contains("HTTP 错误 500"), "应含 500：{}", r.输出);
    }

    #[test]
    fn 测试_默认走真实_无key报错() {
        let _g = 环境锁();
        清空_env();
        let r = 跑流水线_mock_llm("default-real-no-key");
        // 界主硬规则：默认走真实 API，无 key 必须 fail loud，严禁降级 mock
        assert_eq!(
            r.退出码, 4,
            "默认应走真实且无 key 报错（模型故障）：{}",
            r.输出
        );
        assert!(r.输出.contains("后端=默认") || r.输出.contains("后端=真实"));
        assert!(r.输出.contains("严禁降级 mock"));
    }
    #[test]
    fn 测试_LLM_BACKEND_mock_显式走_mock() {
        let _g = 环境锁();
        清空_env();
        std::env::set_var("LLM_BACKEND", "mock");
        let r = 跑流水线_mock_llm("env-mock");
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("后端=Mock"));
        std::env::remove_var("LLM_BACKEND");
    }

    #[test]
    fn 测试_LLM_BACKEND_real_无_key_报错() {
        let _g = 环境锁();
        清空_env();
        std::env::set_var("LLM_BACKEND", "real");
        // LLM_API_KEY 未设置 → fail loud（严禁降级 mock）
        let r = 跑流水线_mock_llm("env-real-no-key");
        assert_eq!(r.退出码, 4, "真实模式无 key 应报错（模型故障）：{}", r.输出);
        assert!(r.输出.contains("后端=真实"));
        assert!(r.输出.contains("严禁降级 mock"));
        std::env::remove_var("LLM_BACKEND");
    }
    #[test]
    fn 测试_LLM_BACKEND_real_有_key_走真实() {
        let _g = 环境锁();
        清空_env();
        // 先设置 key，再设置 backend，避免 race（key 在 backend 之前可见）
        std::env::set_var("LLM_API_KEY", "sk-test-fake-key");
        std::env::set_var(
            "LLM_BASE_URL",
            "https://api.test.invalid/v1/chat/completions",
        );
        std::env::set_var("LLM_BACKEND", "real");
        // 有 key → 真实模式尝试 HTTP；但 base URL 是 .invalid 会失败
        // 此测试只验证：走到「真实模式」分支（[真实模式] 行），不验证 HTTP 成功
        let r = 跑流水线_mock_llm("env-real-with-key");
        // 故障合约：真实 LLM 网络失败必须 fail loud（退出码 4），不得假装成功
        assert_eq!(r.退出码, 4, "真实 LLM 失败应 fail loud：{}", r.输出);
        // 后端=真实 总是出现
        assert!(r.输出.contains("后端=真实"));
        assert!(
            r.输出.contains("[真实模式]"),
            "有 key 应走真实模式：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[LLM 道祖 错误·"),
            "HTTP 失败应记录错误：{}",
            r.输出
        );
        清空_env();
    }

    #[test]
    fn 测试_跑流水线_真实_llm_显式_无_key_报错() {
        let _g = 环境锁();
        清空_env();
        // 显式走真实模式 + 无 key → 必须 fail loud（严禁降级 mock）
        let r = 跑流水线_真实_llm("explicit-real-no-key");
        assert_eq!(r.退出码, 4, "显式真实无 key 应报错（模型故障）：{}", r.输出);
        assert!(r.输出.contains("后端=真实"));
        assert!(r.输出.contains("严禁降级 mock"));
    }
    #[test]
    fn 测试_跑流水线_真实_llm_显式_有_key() {
        let _g = 环境锁();
        清空_env();
        std::env::set_var("LLM_API_KEY", "sk-test");
        std::env::set_var(
            "LLM_BASE_URL",
            "https://api.test.invalid/v1/chat/completions",
        );
        let r = 跑流水线_真实_llm("explicit-real-with-key");
        // 故障合约：有 key 但 HTTP 失败 → fail loud（退出码 4）
        assert_eq!(r.退出码, 4, "真实 LLM 失败应 fail loud：{}", r.输出);
        assert!(r.输出.contains("[真实模式]"));
        assert!(r.输出.contains("[LLM 道祖 错误·"));
        清空_env();
    }

    #[test]
    fn 测试_任务标识传递_真实模式() {
        let _g = 环境锁();
        清空_env();
        let r = 跑流水线_真实_llm("任务标识真实模式");
        assert!(r.输出.contains("任务标识真实模式"));
    }

    #[test]
    fn 测试_任务标识传递_mock模式() {
        let _g = 环境锁();
        清空_env();
        let r = 跑流水线_mock_llm("任务标识mock模式");
        assert!(r.输出.contains("任务标识mock模式"));
    }

    #[test]
    fn 测试_交付回填_mock_终裁通过写执行产出块() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("回填测试_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        let 池 = 构建模拟池();
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::Mock(MockLLM连接::新建()));
        // mock 终裁响应「道祖终裁：通过」+ 产出含三必填 → 门禁通过 → 前端交付（fire-and-forget）
        let mut r = 跑流水线_按连接("回填任务", &调用器, String::new(), Some(&库), 3);
        // 编排层后端还债：前端交付后独立驱动整理（契约 §六 前后端分离）
        后端_还债_至清零(&库, &调用器, &mut r.输出);
        assert_eq!(r.退出码, 0, "回填不应影响前端成功：{}", r.输出);
        assert!(
            r.输出.contains("[产出评分]"),
            "通过应记录产出评分：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[后端整理] 正常归档"),
            "后端整理应归档：{}",
            r.输出
        );
        let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
        let 中枢 = jiyi_chengzai_fu::格位中枢::新建(存储);
        let 分区 = 中枢.读取_格位_分区(jiyi_chengzai_fu::格位 {
            总纲: jiyi_chengzai_fu::总纲::执行,
            本质: jiyi_chengzai_fu::本质::产出,
        });
        assert_eq!(分区.len(), 1, "执行×产出 应有 1 块");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_交付回填_终裁打回不回填() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("回填打回_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        let 池 = 构建模拟池();
        let mut 连接 = MockLLM连接::新建();
        连接.终裁响应 = "道祖终裁：打回，缺边界条件".to_string();
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::Mock(连接));
        let r = 跑流水线_按连接("回填打回任务", &调用器, String::new(), Some(&库), 3);
        assert_eq!(
            r.退出码, 3,
            "打回达上限不得假成功（状态机违规）：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[交付] 终裁判定打回，不交付不回填"),
            "打回不回填：{}",
            r.输出
        );
        let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
        let 中枢 = jiyi_chengzai_fu::格位中枢::新建(存储);
        let 分区 = 中枢.读取_格位_分区(jiyi_chengzai_fu::格位 {
            总纲: jiyi_chengzai_fu::总纲::执行,
            本质: jiyi_chengzai_fu::本质::产出,
        });
        assert!(分区.is_empty(), "打回不应产生块");
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_后端整理_端到端_交付回填债务清零() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("后端整理_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        let 池 = 构建模拟池();
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::Mock(MockLLM连接::新建()));

        // 前端登记 + 交付（fire-and-forget）
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let mut 双工 = jiyi_chengzai_fu::双工流水线::新建(存储);
            双工.前端_登记("后端任务").unwrap();
            双工.前端_交付("后端任务").unwrap();
            assert_eq!(双工.债务().unwrap(), 1, "交付后债务=1");
        }

        // 后端整理（提炼回填 + 归档）
        let 结果 = 后端整理_一个(&库, &调用器).unwrap().unwrap();
        match 结果 {
            jiyi_chengzai_fu::整理结果::正常归档(任务) => assert_eq!(任务, "后端任务"),
            jiyi_chengzai_fu::整理结果::降级归档(任务) => {
                panic!("应正常归档，实际降级：{}", 任务)
            }
        }

        // 债务清零 + 执行×产出有块（账本与格位同一 SQLite 文件共享）
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let 双工 = jiyi_chengzai_fu::双工流水线::新建(存储);
            assert_eq!(双工.债务().unwrap(), 0, "整理后债务=0");
        }
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let 中枢 = jiyi_chengzai_fu::格位中枢::新建(存储);
            let 分区 = 中枢.读取_格位_分区(jiyi_chengzai_fu::格位 {
                总纲: jiyi_chengzai_fu::总纲::执行,
                本质: jiyi_chengzai_fu::本质::产出,
            });
            assert_eq!(分区.len(), 1, "执行×产出应有 1 块");
        }
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_前端交付后_后台线程异步还债() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("真并发_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        let 池 = 构建模拟池();
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::Mock(MockLLM连接::新建()));

        // 前端登记 + 交付（fire-and-forget）→ 债务=1，前端立即返回（不等待后端）
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let mut 双工 = jiyi_chengzai_fu::双工流水线::新建(存储);
            双工.前端_登记("并发任务").unwrap();
            双工.前端_交付("并发任务").unwrap();
            assert_eq!(双工.债务().unwrap(), 1, "前端交付后债务=1（后端未还债）");
        }

        // 真并发：后台线程独立驱动后端还债，前端不 await（Arc 共享调用器跨线程）
        let 库克隆 = 库.clone();
        let 调用器 = std::sync::Arc::new(调用器);
        let 后端线程 = std::thread::spawn(move || {
            let mut 后端日志 = String::new();
            后端_还债_至清零(&库克隆, 调用器.as_ref(), &mut 后端日志);
            后端日志
        });
        let 后端日志 = 后端线程.join().unwrap_or_default();
        assert!(
            后端日志.contains("[后端整理]"),
            "后端线程应执行整理：{}",
            后端日志
        );

        // 后端还债完成 → 债务清零
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let 双工 = jiyi_chengzai_fu::双工流水线::新建(存储);
            assert_eq!(双工.债务().unwrap(), 0, "后端还债后债务=0");
        }
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_前端交付失败_不产生债务() {
        let _g = 环境锁();
        清空_env();
        // 无效存储路径（父目录不存在）→ 前端_登记并交付 失败 → 未交付 → 债务不增（无残留）
        let r = 前端_登记并交付("不存在的目录_绝对不存在/库.sq3", "失败任务");
        assert!(r.is_err(), "无效路径应交付失败：{:?}", r);
    }

    #[test]
    fn 测试_后端整理降级_提炼失败仍归档债务清零() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("降级整理_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        // 前端登记 + 交付 → 债务=1
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let mut 双工 = jiyi_chengzai_fu::双工流水线::新建(存储);
            双工.前端_登记("降级任务").unwrap();
            双工.前端_交付("降级任务").unwrap();
            assert_eq!(双工.债务().unwrap(), 1, "交付后债务=1");
        }
        // 提炼失败（额度耗尽不可重试 → 整理回调 false）→ 降级归档 → 债务清零 + 待补提炼
        let 池 = 构建模拟池();
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::故障(moxing_fu::错误::额度耗尽));
        let 结果 = 后端整理_一个(&库, &调用器).unwrap().unwrap();
        assert!(
            matches!(结果, jiyi_chengzai_fu::整理结果::降级归档(_)),
            "提炼失败应降级归档"
        );
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let 双工 = jiyi_chengzai_fu::双工流水线::新建(存储);
            assert_eq!(双工.债务().unwrap(), 0, "降级归档后债务=0");
        }
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_四时机召回_验收对比与玉玺裁决() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("四时机召回_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        // 写 执行×验证 验收块（时机③召回源）+ 目标×当前 玉玺块（时机④召回源，带手印）
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let mut 中枢 = jiyi_chengzai_fu::格位中枢::新建(存储);
            中枢
                .写入_格位_幂等(
                    jiyi_chengzai_fu::格位 {
                        总纲: jiyi_chengzai_fu::总纲::执行,
                        本质: jiyi_chengzai_fu::本质::验证,
                    },
                    jiyi_chengzai_fu::阶段::验收,
                    "历史验收块内容",
                    "验收摘要",
                    jiyi_chengzai_fu::档位::权档,
                    jiyi_chengzai_fu::来源::LLM,
                    "准圣级",
                    "鉴·验收",
                    "任务",
                )
                .unwrap();
            中枢
                .写入_格位_幂等(
                    jiyi_chengzai_fu::格位 {
                        总纲: jiyi_chengzai_fu::总纲::目标,
                        本质: jiyi_chengzai_fu::本质::当前,
                    },
                    jiyi_chengzai_fu::阶段::拍板,
                    "玉玺目标当前内容",
                    "玉玺摘要",
                    jiyi_chengzai_fu::档位::权档,
                    jiyi_chengzai_fu::来源::人类,
                    "界主",
                    "元",
                    "玉玺任务",
                )
                .unwrap();
            中枢
                .盖玉玺_格位(
                    jiyi_chengzai_fu::格位 {
                        总纲: jiyi_chengzai_fu::总纲::目标,
                        本质: jiyi_chengzai_fu::本质::当前,
                    },
                    "玉玺任务",
                    "界主手印",
                )
                .unwrap();
        }
        // 组装四时机召回上下文
        let 召回 = 组装召回上下文_按路径(&库, "任务").unwrap();
        assert!(
            召回.验收对比.contains("历史验收块内容"),
            "时机③ 验收对比应含历史验收块：{}",
            召回.验收对比
        );
        assert!(
            召回.验收对比.contains("最新基线"),
            "时机③ 最近验收块应标注最新基线：{}",
            召回.验收对比
        );
        assert!(
            召回.玉玺裁决.contains("玉玺目标当前内容"),
            "时机④ 玉玺裁决应含玉玺块：{}",
            召回.玉玺裁决
        );
        assert!(
            !召回.玉玺裁决.contains("历史验收块内容"),
            "非玉玺块不应进玉玺裁决：{}",
            召回.玉玺裁决
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_验收对比_跨任务隔离() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("跨任务隔离_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        {
            let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
            let mut 中枢 = jiyi_chengzai_fu::格位中枢::新建(存储);
            // 任务甲 与 任务乙 各写一个验收块（执行×验证）
            for (任务, 内容) in [("任务甲", "甲验收块"), ("任务乙", "乙验收块")] {
                中枢
                    .写入_格位_幂等(
                        jiyi_chengzai_fu::格位 {
                            总纲: jiyi_chengzai_fu::总纲::执行,
                            本质: jiyi_chengzai_fu::本质::验证,
                        },
                        jiyi_chengzai_fu::阶段::验收,
                        内容,
                        "验收摘要",
                        jiyi_chengzai_fu::档位::权档,
                        jiyi_chengzai_fu::来源::LLM,
                        "准圣级",
                        "鉴·验收",
                        任务,
                    )
                    .unwrap();
            }
        }
        let 召回甲 = 召回_验收对比(&库, "任务甲");
        assert!(
            召回甲.iter().any(|s| s.contains("甲验收块")),
            "任务甲应召回甲验收块：{:?}",
            召回甲
        );
        assert!(
            !召回甲.iter().any(|s| s.contains("乙验收块")),
            "任务甲不应召回乙验收块（跨任务隔离）：{:?}",
            召回甲
        );
        let 召回乙 = 召回_验收对比(&库, "任务乙");
        assert!(
            召回乙.iter().any(|s| s.contains("乙验收块")),
            "任务乙应召回乙验收块：{:?}",
            召回乙
        );
        assert!(
            !召回乙.iter().any(|s| s.contains("甲验收块")),
            "任务乙不应召回甲验收块：{:?}",
            召回乙
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_读终裁采样次数_默认1() {
        let _g = 环境锁();
        清空_env();
        assert_eq!(读终裁采样次数(), 1);
    }

    #[test]
    fn 测试_读终裁采样次数_自定义3_非法回退1() {
        let _g = 环境锁();
        清空_env();
        std::env::set_var("LLM_终裁采样次数", "3");
        assert_eq!(读终裁采样次数(), 3);
        std::env::set_var("LLM_终裁采样次数", "abc");
        assert_eq!(读终裁采样次数(), 1, "非法值回退 1");
        std::env::set_var("LLM_终裁采样次数", "0");
        assert_eq!(读终裁采样次数(), 1, "零回退 1");
    }

    #[test]
    fn 测试_质量门禁_四要素() {
        // 四要素齐全 → 无缺项
        assert!(质量门禁校验(
            "decided_by=ai助手 falsifiable=测试全绿 implements=术 复现=cargo test全绿"
        )
        .is_empty());
        // 缺 decided_by
        assert_eq!(
            质量门禁校验("falsifiable=测试全绿 implements=术 复现=cargo test全绿"),
            vec!["decided_by"]
        );
        // 缺 falsifiable
        assert_eq!(
            质量门禁校验("decided_by=ai助手 implements=术 复现=cargo test全绿"),
            vec!["falsifiable"]
        );
        // 缺 implements
        assert_eq!(
            质量门禁校验("decided_by=ai助手 falsifiable=测试全绿 复现=cargo test全绿"),
            vec!["implements"]
        );
        // 缺 复现
        assert_eq!(
            质量门禁校验("decided_by=ai助手 falsifiable=测试全绿 implements=术"),
            vec!["复现"]
        );
        // 全缺
        assert_eq!(
            质量门禁校验("空壳产出"),
            vec!["decided_by", "falsifiable", "implements", "复现"]
        );
    }

    #[test]
    fn 测试_提取修订要求_结构化提取() {
        // 真实打回报告结构：致命修复表 + 验收判据
        let 报告 = "裁示：打回。\n### 3.1 致命修复（3 项 · 缺一即打回）\n| F1 | 闭合编译错误 | cargo build 返回 0 |\n| F2 | 回归全绿 | cargo test 全绿 |\n以上为修订要求。";
        let 要求 = 提取修订要求(报告);
        assert!(要求.contains("致命修复"), "应含致命修复：{}", 要求);
        assert!(要求.contains("F1"), "应含 F1：{}", 要求);
        assert!(
            要求.contains("验收判据") || 要求.contains("cargo build"),
            "应含验收判据/复现命令：{}",
            要求
        );
        assert!(!要求.contains("裁示：打回"), "应剔除纯裁决行：{}", 要求);
    }

    #[test]
    fn 测试_提取修订要求_无结构回退() {
        // 无结构化修复项 → 回退到原文前 20 行
        let 报告 = "打回，缺边界条件。\n请补齐材料。";
        let 要求 = 提取修订要求(报告);
        assert_eq!(要求, 报告, "无结构应回退原文：{}", 要求);
    }

    #[test]
    fn 测试_产出评分_空壳零分_齐全满分() {
        // 空壳产出 → 0 分
        assert_eq!(产出评分("空壳"), 0, "空壳应 0 分");
        // 三要素齐全 + 可复现 → 100 分
        let 满分产出 =
            "decided_by=道祖级 falsifiable=测试全绿 implements=术 复现命令 cargo test --nocapture";
        assert_eq!(产出评分(满分产出), 100, "齐全应 100 分");
        // 缺 decided_by → 80 分
        let 缺决策 = "falsifiable=测试全绿 implements=术 复现 cargo test";
        assert_eq!(产出评分(缺决策), 80, "缺 decided_by 应 80 分");
        // 缺可复现 → 80 分（文本避开「复现/验证/cargo test」以免误命中）
        let 缺复现 = "decided_by=道祖级 falsifiable=测试全绿 implements=术 内容很丰富但缺命令";
        assert_eq!(产出评分(缺复现), 80, "缺可复现应 80 分");
    }

    #[test]
    fn 测试_质量门禁_终裁通过但缺必填改判打回() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("门禁否决_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        let 池 = 构建模拟池();
        let mut 连接 = MockLLM连接::新建();
        连接.响应内容 = "空壳产出".to_string(); // 4 角色产出缺三必填
        连接.终裁响应 = "道祖终裁：通过".to_string(); // 终裁明确通过
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::Mock(连接));
        let r = 跑流水线_按连接("门禁任务", &调用器, String::new(), Some(&库), 3);
        assert_eq!(r.退出码, 3, "门禁否决打回达上限不得假成功：{}", r.输出);
        assert!(
            r.输出.contains("[质量门禁] 终裁通过但产出缺必填项"),
            "应记录门禁否决：{}",
            r.输出
        );
        // 不应交付：执行×产出 0 块
        let 存储 = jiyi_chengzai_fu::SQLite存储::文件新建(&库).unwrap();
        let 中枢 = jiyi_chengzai_fu::格位中枢::新建(存储);
        let 分区 = 中枢.读取_格位_分区(jiyi_chengzai_fu::格位 {
            总纲: jiyi_chengzai_fu::总纲::执行,
            本质: jiyi_chengzai_fu::本质::产出,
        });
        assert_eq!(分区.len(), 0, "门禁否决应不交付：{} 块", 分区.len());
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_终裁结构化_不通过也判打回() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("结构化打回_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        let 池 = 构建模拟池();
        let mut 连接 = MockLLM连接::新建();
        // 不含「打回」字，但含「不通过」——结构化判定必须捕获真实终裁变体
        连接.终裁响应 = "本次任务暂不通过，需补齐材料".to_string();
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::Mock(连接));
        let r = 跑流水线_按连接("结构化打回任务", &调用器, String::new(), Some(&库), 3);
        assert_eq!(
            r.退出码, 3,
            "打回达上限不得假成功（状态机违规）：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[交付] 终裁判定打回，不交付不回填"),
            "「不通过」应判打回：{}",
            r.输出
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_打回重投_终裁打回触发下探重投() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("打回重投_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        let 池 = 构建模拟池();
        let mut 连接 = MockLLM连接::新建();
        连接.终裁响应 = "道祖终裁：打回，缺边界条件".to_string();
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::Mock(连接));
        let r = 跑流水线_按连接("打回重投任务", &调用器, String::new(), Some(&库), 3);
        assert_eq!(
            r.退出码, 3,
            "打回达上限不得假成功（状态机违规）：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[打回重投] 第 1 轮"),
            "应打回重投：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[打回] 已打回 3 轮达到上限"),
            "3 轮后停止重投：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[交付] 终裁判定打回，不交付不回填"),
            "最终打回不回填：{}",
            r.输出
        );
        // 治理事件流审计：打回关键节点 append-only 可追溯（元三治·治强）
        let 事件 = crate::事件流_读取(&库, 1, 999).unwrap();
        let 事件文本 = 事件.join("\n");
        assert!(
            事件文本.contains("打回重投"),
            "打回重投入事件流：{}",
            事件文本
        );
        assert!(
            事件文本.contains("打回达上限"),
            "打回达上限入事件流：{}",
            事件文本
        );
        assert!(
            事件文本.contains("终裁打回"),
            "终裁打回入事件流：{}",
            事件文本
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_打回上限可配置_上限1只重投1轮() {
        let _g = 环境锁();
        清空_env();
        let 库 = std::env::temp_dir()
            .join(format!("打回上限1_{}.sq3", std::process::id()))
            .to_string_lossy()
            .into_owned();
        let _ = std::fs::remove_file(&库);
        let 池 = 构建模拟池();
        let mut 连接 = MockLLM连接::新建();
        连接.终裁响应 = "道祖终裁：打回，缺边界条件".to_string();
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::Mock(连接));
        let r = 跑流水线_按连接("打回上限1任务", &调用器, String::new(), Some(&库), 1);
        assert_eq!(
            r.退出码, 3,
            "打回达上限不得假成功（状态机违规）：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[打回] 已打回 1 轮达到上限"),
            "上限 1 应 1 轮停止：{}",
            r.输出
        );
        assert!(
            !r.输出.contains("[打回重投] 第 2 轮"),
            "不应重投第 2 轮：{}",
            r.输出
        );
        let _ = std::fs::remove_file(&库);
    }

    #[test]
    fn 测试_是否可重试_变体覆盖() {
        // 可重试：网络抖动类
        assert!(是否可重试(&moxing_fu::错误::超时));
        assert!(是否可重试(&moxing_fu::错误::HTTP错误 {
            状态码: 500,
            原因: "internal".to_string(),
        }));
        assert!(是否可重试(&moxing_fu::错误::HTTP错误 {
            状态码: 503,
            原因: "overload".to_string(),
        }));
        assert!(是否可重试(
            &moxing_fu::错误::解析错误("截断".to_string())
        ));
        // 网络传输错误（状态码 0，连接失败/超时）也重试
        assert!(是否可重试(&moxing_fu::错误::HTTP错误 {
            状态码: 0,
            原因: "network".to_string(),
        }));
        // 不可重试：业务错误（重试无益，fail loud）
        assert!(!是否可重试(&moxing_fu::错误::HTTP错误 {
            状态码: 400,
            原因: "bad".to_string(),
        }));
        assert!(!是否可重试(&moxing_fu::错误::HTTP错误 {
            状态码: 401,
            原因: "unauth".to_string(),
        }));
        assert!(!是否可重试(&moxing_fu::错误::额度耗尽));
        assert!(!是否可重试(&moxing_fu::错误::鉴权失败));
        assert!(!是否可重试(&moxing_fu::错误::配置错误(
            "缺 key".to_string()
        )));
    }

    #[test]
    fn 测试_错误归因_标签覆盖() {
        assert_eq!(错误归因(&moxing_fu::错误::超时), "超时");
        assert_eq!(
            错误归因(&moxing_fu::错误::HTTP错误 {
                状态码: 500,
                原因: "x".to_string()
            }),
            "5xx"
        );
        assert_eq!(
            错误归因(&moxing_fu::错误::HTTP错误 {
                状态码: 400,
                原因: "x".to_string()
            }),
            "4xx"
        );
        assert_eq!(
            错误归因(&moxing_fu::错误::HTTP错误 {
                状态码: 0,
                原因: "x".to_string()
            }),
            "网络"
        );
        assert_eq!(错误归因(&moxing_fu::错误::额度耗尽), "额度");
        assert_eq!(错误归因(&moxing_fu::错误::鉴权失败), "鉴权");
        assert_eq!(
            错误归因(&moxing_fu::错误::配置错误("x".to_string())),
            "配置"
        );
        assert_eq!(
            错误归因(&moxing_fu::错误::解析错误("x".to_string())),
            "解析"
        );
    }

    #[test]
    fn 测试_调用带重试_超时后重试成功() {
        let 池 = 构建模拟池();
        let 连接 = MockLLM连接 {
            响应内容: "重试成功".to_string(),
            最近请求: std::sync::Mutex::new(None),
            首次失败: std::sync::Mutex::new(Some(moxing_fu::错误::超时)),
            终裁响应: "道祖终裁：通过".to_string(),
        };
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::Mock(连接));
        let r = 调用_带重试(&调用器, "道祖", &moxing_fu::请求::新建("", vec![]));
        assert!(r.is_ok(), "超时后应重试成功：{:?}", r.err());
        assert_eq!(r.unwrap().内容, "重试成功");
    }

    #[test]
    fn 测试_调用带重试_业务错误不重试() {
        let 池 = 构建模拟池();
        let 调用器 = moxing_fu::LLM调用器::新建(池, 连接抽象::故障(moxing_fu::错误::鉴权失败));
        let r = 调用_带重试(&调用器, "道祖", &moxing_fu::请求::新建("", vec![]));
        assert!(r.is_err(), "鉴权失败不应重试成功");
        match r.err().unwrap() {
            moxing_fu::错误::鉴权失败 => {}
            other => panic!("应返回鉴权失败，实得 {}", other),
        }
    }

    #[test]
    fn 测试_自举流水线_大罗产出确定性落盘() {
        let _g = 环境锁();
        清空_env();
        std::env::set_var("LLM_BACKEND", "mock");
        // 构造自举任务单（目标文件为临时路径，验收命令走白名单）
        let mut 参数 = std::collections::HashMap::new();
        参数.insert("标识".to_string(), "自举测试-001".to_string());
        参数.insert("目标文件".to_string(), "自举测试_临时.rs".to_string());
        参数.insert("需求描述".to_string(), "加一个返回42的函数".to_string());
        参数.insert("验收命令".to_string(), "cargo fmt --check".to_string());
        参数.insert("可证伪命题".to_string(), "退出码0".to_string());
        参数.insert("decided_by".to_string(), "界主".to_string());
        let 单 = renwu_zhixing_fu::自举任务单::从参数解析(&参数).expect("任务单解析失败");
        let r = 跑流水线_自举(&单);
        // 空代码保护（mock 大罗响应无代码围栏 → 提取为空 → 拒绝写空文件，防破坏既有资产）
        assert!(
            !std::path::Path::new("自举测试_临时.rs").exists(),
            "大罗无代码围栏时不得落空文件：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[自举执行]") && r.输出.contains("拒绝写空文件"),
            "日志应含自举执行证据+拒绝写空文件：{}",
            r.输出
        );
        std::fs::remove_file("自举测试_临时.rs").ok();
    }
}
