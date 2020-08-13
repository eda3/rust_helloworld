fn main() {
    /// 値の所有権をもらう代わりに引数としてオブジェクトへの参照をとる
    let s1 = String::from("hello");

    // s1の値を参照する参照を作成
    let len = calculate_length(&s1);

    println!("{}の長さは{}です", s1, len);
    // => helloの長さは5です
    println!();

    /// 借用しようとした値の変更を試みる
    let mut s1 = String::from("hello");

    // これだとエラーになる
    // change(&s1);
    // &mut s1することで、エラーにならない
    change(&mut s1);
    println!("{}", s1);
    println!();

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize { // sはStringへの参照
    s.len()
} // ここで、sはスコープ外になる。参照しているもの(s1)の所有権は持っていないので何も起きない

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String{ // dangleはStringを返す
fn dangle() -> String{ // dangleはStringを返す
    let s = String::from("hello"); // sは新しいString

    // &s // String sへの参照を返す。このままだと危険。ドロップされるときそのメモリは消される
    s
}