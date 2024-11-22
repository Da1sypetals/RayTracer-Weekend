use std::io::{self, Write};

pub fn wait_for_input() {
    let mut input = String::new();
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap(); // 确保提示信息在等待输入之前显示
    io::stdin().read_line(&mut input).unwrap(); // 等待用户输入
}
