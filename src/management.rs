use mysql::from_row;

use crate::number;
use crate::number::INPUT_ERR_MESSAGE;
use crate::tools;
pub fn first_management() {
    unsafe {
        if number::IS_MANAGE_FIST_TIME {
            number::IS_MANAGE_FIST_TIME = false;
            println!("欢迎来到学生信息管理系统");
            println!();
        }
    }
    println!("请选择一个您的身份");
    println!("选项 1\t 系统管理员");
    println!("选项 2\t 普通管理员");
    println!("选项 3\t 学生");
}
pub fn super_admin_manage() {
    let (user_name, password) = tools::scanf_username_and_password();
    if !(user_name == number::SUPER_ADMIN_USERNAME && password == number::SUPER_ADMIN_PASSWORD) {
        println!("你的输入有误，拒绝访问!");
        return;
    }
    println!("欢迎使用超级管理员面板");
}

pub fn admin_manage() {
    let (user_name, password) = tools::scanf_username_and_password();
    if !(user_name == number::ADMIN_USERNAME && password == number::ADMIN_PASSWORD) {
        println!("你的输入有误， 拒绝访问!");
        return;
    }
    println!("欢迎使用管理员系统");
}
pub fn student_manage() {
    let (user_id, password) = tools::scanf_username_and_password();
    let sql: String = format!(
        "select * from student where id = '{}' and password = '{}'",
        user_id, password
    );

    // 进行查询，查看是否有这个学生
    use mysql::prelude::*;

    let mut have: bool = false;
    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
    connect.query_iter(sql).unwrap().for_each(|row| {
        let r: (String, String, i32, i32, String, String) = from_row(row.unwrap());
        println!("{} {} {} {} {} {}", r.0, r.1, r.2, r.3, r.4, r.5);
        have = true;
    });

    match have {
        true => println!("欢迎来到学生管理系统"),
        false => {
            println!("{}", INPUT_ERR_MESSAGE);
            return;
        }
    }
}
