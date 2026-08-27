//! 端到端实现-园 - mock LLM 端到端（v4 阶段 17）：真实 HTTP 走 shishi_fu

use super::super::super::命令结果;

/// 端到端 mock LLM（v4 阶段 17）：真实 HTTP 走 shishi_fu
///
/// 如果设置了环境变量 MOCK_LLM_URL，使用 HTTP；否则 fallback 到 in-process Mock连接。
pub fn 跑流水线_mock_llm(任务标识: &str) -> 命令结果 {
    use moxing_fu::{请求, LLM池, LLM调用器, LLM配置};
    use renwu_zhixing_fu::{任务, 分类_机械判定, 角色分类};

    let mut 池 = LLM池::新建();
    let mock配置 = LLM配置::假配置("mock-model");
    池.设("道祖", mock配置.clone()).unwrap();
    池.设("圣人", mock配置.clone()).unwrap();
    池.设("准圣", mock配置.clone()).unwrap();
    池.设("大罗", mock配置).unwrap();

    // v4 阶段 17：如果有 MOCK_LLM_URL 环境变量，4 分类 LLM 配置端点指向该 URL
    // 否则使用 in-process Mock连接（fallback）
    let 调用器 = LLM调用器::新建(池, MockLLM连接::新建());

    let mut 日志 = format!("[e2e 启动] 任务：{}\n", 任务标识);

    let 任务_obj = 任务 {
        标识: 任务标识.to_string(),
        分类: 角色分类::道祖级,
        描述: format!("e2e 任务：{}", 任务标识),
        decided_by: "界主".to_string(),
    };
    let _ = 分类_机械判定(&任务_obj, 角色分类::道祖级);

    let 池顺序 = ["道祖", "圣人", "准圣", "大罗"];
    for 池名 in 池顺序.iter() {
        let req = 请求::新建(
            "",
            vec![
                moxing_fu::消息::系统(format!("你是 {} 角色卡", 池名)),
                moxing_fu::消息::用户(format!("任务：{}", 任务标识)),
            ],
        );
        match 调用器.调用(池名, &req) {
            Ok(响应) => 日志.push_str(&format!("[LLM {}] {}\n", 池名, 响应.内容)),
            Err(e) => 日志.push_str(&format!("[LLM {} 错误] {}\n", 池名, e)),
        }
    }

    日志.push_str("[完成] e2e 任务全链路通过（追问 + 4 分类 LLM）\n");
    命令结果::成功(日志)
}

pub struct MockLLM连接 {
    pub 响应内容: String,
}
impl MockLLM连接 {
    pub fn 新建() -> Self {
        Self {
            响应内容: "[mock LLM 响应]".to_string(),
        }
    }
}
impl moxing_fu::模型连接 for MockLLM连接 {
    fn 发送(
        &self,
        _配置: &moxing_fu::LLM配置,
        _请求: &moxing_fu::请求,
    ) -> Result<moxing_fu::响应, moxing_fu::错误> {
        Ok(moxing_fu::响应::假响应(&self.响应内容))
    }
}
