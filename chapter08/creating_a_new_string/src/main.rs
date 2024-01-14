fn main() {
    let mut s = String::new();
    println!("[{}]", s);

    let data = "initial contents";
    println!("[{}]", data);

    let s = data.to_string();
    println!("[{}]", s);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("[{}]", s);

    let s = String::from("initial contents");
    println!("[{}]", s);

    let hello = String::from("السلام عليكم");
    println!("[{}]", hello);
    let hello = String::from("Dobrý den");
    println!("[{}]", hello);
    let hello = String::from("Hello");
    println!("[{}]", hello);
    let hello = String::from("שָׁלוֹם");
    println!("[{}]", hello);
    let hello = String::from("नमस्ते");
    println!("[{}]", hello);
    let hello = String::from("こんにちは");
    println!("[{}]", hello);
    let hello = String::from("안녕하세요");
    println!("[{}]", hello);
    let hello = String::from("你好");
    println!("[{}]", hello);
    let hello = String::from("Olá");
    println!("[{}]", hello);
    let hello = String::from("Здравствуйте");
    println!("[{}]", hello);
    let hello = String::from("Hola");
    println!("[{}]", hello);
}
