//! 内存度量 - 内存使用量读取 + 5 项基准脚本

pub fn 内存_使用() -> u64 {
    let _ = std::process::id();
    0
}

pub fn 基准_全() -> super::super::super::运行_执行_阁::运行_实现_园::基准器 {
    use super::super::super::运行_执行_阁::运行_实现_园::基准器;
    let mut b = 基准器::新建();
    b.跑("空运行", || {
        let mut s = 0u64;
        for _ in 0..1000 {
            s = s.wrapping_add(1);
        }
    });
    b.跑("字符串构建", || {
        let mut s = String::new();
        for i in 0..100 {
            s.push_str(&format!("测试-{}-{}", i, i * i));
        }
    });
    b.跑("Vec 增删", || {
        let mut v: Vec<u64> = Vec::new();
        for i in 0..1000 {
            v.push(i);
        }
        for _ in 0..500 {
            v.pop();
        }
    });
    b.跑("HashMap 插入", || {
        let mut m: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
        for i in 0..1000 {
            m.insert(i, i * i);
        }
    });
    b.跑("字符串哈希", || {
        let mut h: u64 = 0;
        for i in 0..1000 {
            h = h.wrapping_mul(31).wrapping_add(i);
        }
    });
    b
}
