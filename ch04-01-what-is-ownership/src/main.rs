fn main() {
    let mut s = String::from("hello"); // _sはここから有効になる
    s.push_str(", world");

    let s1 = String::from("hello");
    // let s2 = s1; これだとエラーになる
    // println!("{}", s1);

    let s2 = s1.clone(); // clone()を使うとオッケー
    println!("s1 = {}, s2 = {}", s1, s2);
    // => s1 = hello, s2 = hello
    println!();

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜けて、dropが呼ばれる。

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
