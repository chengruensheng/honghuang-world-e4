//! 查格位绑定 - 临时排查「执行×产出/验证」分区的绑定任务分布
use jiyi_chengzai_fu::{总纲, 本质, 格位, 格位中枢, SQLite存储};
use mingling_caozuo_fu::默认记忆库路径;

fn main() {
    let 存储 = SQLite存储::文件新建(默认记忆库路径).expect("打开失败");
    let 中枢 = 格位中枢::新建(存储);
    for (总, 本) in [(总纲::执行, 本质::产出), (总纲::执行, 本质::验证)] {
        let 分区 = 中枢.读取_格位_分区(格位 {
            总纲: 总, 本质: 本
        });
        println!("=== 执行×{} === {} 块", 本.名称(), 分区.len());
        for e in &分区 {
            let 绑定 = e
                .块元数据
                .as_ref()
                .map(|m| m.绑定任务.clone())
                .unwrap_or_else(|| "无".to_string());
            println!(
                "  [id{}][绑定={}] {}",
                e.id.0,
                绑定,
                e.内容.chars().take(40).collect::<String>()
            );
        }
    }
}
