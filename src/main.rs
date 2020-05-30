extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("数字当てゲーム！");

    // 答えの数値を設定
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret_number:{}", secret_number);

    println!("1～100の間の数字を入力してください！");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("read line エラー発生");

    println!("あなたの予想：{}", guess);
}

