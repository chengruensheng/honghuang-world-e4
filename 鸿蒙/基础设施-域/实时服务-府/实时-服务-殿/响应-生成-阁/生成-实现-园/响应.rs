//! 生成-实现-园 - 4 分类 LLM 响应（mock）+ 解析分类 + 构造 mock JSON

/// 4 分类 LLM 响应（mock）
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 分类响应 {
    道祖,
    圣人,
    准圣,
    大罗,
}

impl 分类响应 {
    pub fn 名称(&self) -> &'static str {
        match self {
            Self::道祖 => "道祖",
            Self::圣人 => "圣人",
            Self::准圣 => "准圣",
            Self::大罗 => "大罗",
        }
    }
    pub fn 内容(&self) -> &'static str {
        match self {
            Self::道祖 => "[mock 道祖] 决策已下，目标对齐",
            Self::圣人 => "[mock 圣人] 设计完成，方案已固化",
            Self::准圣 => "[mock 准圣] 验收通过，质量达标",
            Self::大罗 => "[mock 大罗] 实现完成，可验收",
        }
    }
}

pub fn 解析_分类(请求: &str) -> 分类响应 {
    if 请求.contains("道祖") || 请求.contains("daozu") {
        return 分类响应::道祖;
    }
    if 请求.contains("圣人") || 请求.contains("shengren") {
        return 分类响应::圣人;
    }
    if 请求.contains("准圣") || 请求.contains("zhunsheng") {
        return 分类响应::准圣;
    }
    if 请求.contains("大罗") || 请求.contains("dalun") {
        return 分类响应::大罗;
    }
    分类响应::大罗
}

pub fn 构造_mock_json(分类: &分类响应) -> String {
    // JSON 字符串中转义双引号
    let mut s = String::new();
    s.push_str("{\"id\":\"chatcmpl-mock\",\"object\":\"chat.completion\",\"model\":\"");
    s.push_str(分类.名称());
    s.push_str("\",\"choices\":[{\"index\":0,\"message\":{\"role\":\"assistant\",\"content\":\"");
    s.push_str(分类.内容());
    s.push_str("\"}}]}");
    s
}
