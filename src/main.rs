mod domain;
mod management;
mod number;
mod tools;

fn main() {
    loop {
        management::first_management();
        let a = tools::scan().parse::<i32>();

        if a.is_err() {
            println!("{}", number::INPUT_ERR_MESSAGE);
            continue;
        }

        let number = a.unwrap();
        if number < 1 || number > 4 {
            println!("{}", number::INPUT_ERR_MESSAGE);
            continue;
        }
        match number {
            1 => {
                management::super_admin_manage();
            }
            2 => {
                management::admin_manage();
            }
            3 => {
                management::student_manage();
            }
            4 => {
                return;
            }
            _ => {}
        }
    }
}

// 测试一下数据库能否正常使用
#[test]
fn test() {
    use mysql::prelude::*;
    use mysql::*;
    let url = number::URL; // 数据库地址
    let pool = mysql::Pool::new(url).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
    connect
        .query_iter("select * from test")
        .unwrap()
        .for_each(|row| {
            let r: (String, i32) = from_row(row.unwrap());
            println!("{} {}", r.0, r.1);
        })
}
