
fn main() {
    println!("Hello World!");
    welcome();
}

fn scan() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}