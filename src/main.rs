use strategy::{Strategy1, Strategy};
use std::process::Command;

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
    let money = 300;
    let mut strategy =  Strategy1::new(money);
    strategy.run();

    //let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
