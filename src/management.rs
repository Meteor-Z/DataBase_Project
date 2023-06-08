use crate::number;
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
}

pub fn admin_manage() {
    let (user_name, password) = tools::scanf_username_and_password();
}
