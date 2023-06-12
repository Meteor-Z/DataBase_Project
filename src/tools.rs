use mysql::PooledConn;

// 简单的读取
pub fn scan() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

pub fn scanf_username_and_password() -> (String, String) {
    println!("请输入你的学号或者工号");
    let user_name = scan();
    println!("请输入你的密码");
    let password = scan();
    
    (user_name, password)
}
pub fn get_connect() -> PooledConn {
    
    let pool = mysql::Pool::new(crate::number::URL).unwrap(); // 获取连接池
    let connect = pool.get_conn().unwrap(); // 获取连接
    connect

}