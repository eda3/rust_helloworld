fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
    println!();

    /* エラーがでる。 「value borrowed here after move」
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
    */

    // clone()を使わないとエラーになる
    let s1 = String::from("hello");
    let _s2 = s1.clone();
    println!("{}, world!", s1);
    println!();

    // これはエラーにならない。
    // 整数などのプリミティブ型はスタックにすっぽり入る
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

}

/*
struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} は食事し始めた", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Player1"),
        Philosopher::new("Player2"),
        Philosopher::new("Player3"),
        Philosopher::new("Player4"),
        Philosopher::new("Player5"),
    ];
    for p in &philosophers {
        p.eat();
    }
}
*/
/*数字当てゲーム
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数字当てゲーム！");

    // 答えの数値を設定
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret_number:{}", secret_number);

    loop {
        println!("1～100の間の数字を入力してください！");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("read line エラー発生");

        // guessとsecret_numberを比較させるため、型変換
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます"),
            Ordering::Greater => println!("大きすぎます"),
            Ordering::Equal => {
                println!("あなたの勝ちです");
                break;
            },
        }
    }
}
*/
