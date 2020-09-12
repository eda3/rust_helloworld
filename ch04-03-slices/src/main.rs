fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word{:?}", word);
}

/// 文字列を受け取って、その文字列中の最初の単語を返す関数
fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
             return i;
        }
    }

    s.len()
}

fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}