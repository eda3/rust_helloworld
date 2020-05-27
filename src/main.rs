use std::io;

fn main() {
    println!("数字当てゲーム！");

    println!("1～100の間の数字を入力してください！");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("read line エラー発生");

    println!("あなたの予想：{}", guess);

