mod management;
mod number;
mod tools;

fn main() {
    management::first_management();
    let a = tools::scan();
    println!("{}", a);
}

// 测试一下数据库能否正常使用
#[test]
fn test() {
    let url = "mysql://root:200298@localhost:3306/database_project"; // 数据库地址
    let pool = mysql::Pool::new(url).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
}
