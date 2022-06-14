use std::io::Read;

fn get_one_char() {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);
}

fn main() {
    let username = whoami::username();
    println!("Hello, {username} - you're hacked...!");
    println!("¯\\(ツ)/¯");
    println!("Press Enter to exit shamefully...!");

    get_one_char();
}
