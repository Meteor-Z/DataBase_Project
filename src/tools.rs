// 简单的读取
pub fn scan() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

pub fn scanf_username_and_password() -> (String, String) {
    println!("请输入你的账号");
    let user_name = scan();
    println!("请输入你的密码");
    let password = scan();
    
    (user_name, password)
}
