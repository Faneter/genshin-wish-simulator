use console_utils::input::{input, select};
use simulator::CharacterUpSimulator;
use strategy::{Strategy, Strategy1};

mod simulator;
mod strategy;

/**
 * 抽卡模拟器基本思路：
 * 先建立一套完善的抽卡模拟器
 * 再允许创建独立的抽卡策略
 * 默认 无策略:继续交给用户自行判断抽卡选项
 *
 * 用户点击抽卡(单/10)->得到抽卡结果(5星[歪/不歪/捕获明光]、四星、三星)->交付选中的抽卡策略判断下一步
 *
 * 大统计工具基本思路
 * 输入抽取规模
 * 选取抽卡策略
 * 默认 无策略1：持续10连 / 无策略2：持续单抽
 *
 * 得到各个策略最终结果(5星[歪数/不歪数/捕获明光数]、四星数、三星数)
 */

fn main() {
    'outer: loop {
        let money: String = input("请输入预算抽数:");
        println!("{}", money);
        let money: i32 = match money.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let options = ["上个小保底歪了", "上个小保底没歪"];
        let selected_index = select("请选择：", &options);
        let option = String::from(options[selected_index]);
        let is_wai = match &option as &str {
            "上个小保底歪了" => true,
            "上个小保底没歪" => false,
            _ => false,
        };
        loop {
            let mut strategy = Strategy1::new(money);
            strategy.run(&mut CharacterUpSimulator::new(2, is_wai));
            let options = ["重来", "修改数据", "退出"];
            let selected_index = select("请选择：", &options);
            let option = String::from(options[selected_index]);
            match &option as &str {
                "重来" => continue, 
                "修改数据" => continue 'outer,
                _ => break 'outer,
            }
        }
    }
}
