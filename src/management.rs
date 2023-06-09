use crate::domain::{Student, Teacher};
use crate::number;
use crate::number::INPUT_ERR_MESSAGE;
use crate::tools;
use mysql::prelude::Queryable;
use mysql::prelude::*;
use mysql::*;

use std::collections::HashMap;
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
    println!("选项 4\t 退出");
}
pub fn super_admin_manage() {
    let (user_name, password) = tools::scanf_username_and_password();
    if !(user_name == number::SUPER_ADMIN_USERNAME && password == number::SUPER_ADMIN_PASSWORD) {
        println!("你的输入有误，拒绝访问!");
        return;
    }
    println!("欢迎使用超级管理员面板");
    loop {
        println!("选项 1 查看学生信息");
        println!("选项 2 查看教师基本信息");
        println!("选项 3 查看教师所受课程");
        println!("选项 4 添加学生");
        println!("选项 5 退出");
        let a = tools::scan().parse::<i32>();

        // 判断错误
        if a.is_err() {
            println!("{}", number::INPUT_ERR_MESSAGE);
            continue;
        }
        // 判断受否是其他数字
        let number = a.unwrap();
        if number < 1 || number > 5 {
            println!("{}", number::INPUT_ERR_MESSAGE);
            continue;
        }
        match number {
            1 => super_admin_manage_one_look_student_info(),
            2 => super_admin_manage_two_look_teacher_info(),
            3 => super_admin_manage_three_look_teacher_class(),
            4 => super_admin_manage_four_add_student(),
            5 => {
                return;
            }
            _ => {}
        }
    }
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

    let mut user = Student::new();
    let mut have_this_user: bool = false;

    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接

    connect.query_iter(sql).unwrap().for_each(|row| {
        let r: (String, String, i32, i32, i32, String) = from_row(row.unwrap());
        user = Student {
            id: r.0.clone(),
            name: r.1.clone(),
            sex: r.2.clone(),
            age: r.3.clone(),
            class: r.4,
            password: r.5.clone(),
        };
        have_this_user = true;
    });
    match have_this_user {
        true => println!("欢迎来到学生管理系统"),
        false => {
            println!("{}", INPUT_ERR_MESSAGE);
            return;
        }
    }
}

/*
       println!("选项 1 查看学生信息");
       println!("选项 2 查看教师基本信息");
       println!("选项 3 查看教师所受课程");
       println!("选项 4 添加学生");
       println!("选项 5 退出");
*/
fn super_admin_manage_one_look_student_info() {
    println!("请输入当前学生的 id");
    let user_id = tools::scan();
    let sql = format!("select * from student  where '{}'", user_id);

    let mut have_this_user: bool = false;
    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
    let mut user: Student = Student::new();

    connect.query_iter(sql).unwrap().for_each(|row| {
        let r: (String, String, i32, i32, i32, String) = from_row(row.unwrap());
        user = Student {
            id: r.0.clone(),
            name: r.1.clone(),
            sex: r.2.clone(),
            age: r.3.clone(),
            class: r.4,
            password: r.5.clone(),
        };
        have_this_user = true;
    });
    match have_this_user {
        true => {
            println!("以下是个人信息\t");
            println!("学号:\t{}", user.id);
            println!("姓名:\t{}", user.name);
            println!("性别:\t{}", user.sex);
            println!("年龄:\t{}", user.age);
            println!("班级:\t{}", user.class);
            println!("密码:\t{}\n", user.password);
        }
        false => {
            println!("没有这个学号\t");
        }
    }

    println!();
}
// 查看教师信息
fn super_admin_manage_two_look_teacher_info() {
    println!("请输入当前教师的工号");
    println!("请输入当前学生的 id");
    let user_id = tools::scan();
    let sql = format!("select * from teacher  where '{}'", user_id);

    let mut have_this_user: bool = false;
    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
    let mut user: Teacher = Teacher::new();

    connect.query_iter(sql).unwrap().for_each(|row| {
        let r: (String, String, i32, String, f64) = from_row(row.unwrap());
        user = Teacher {
            id: r.0.clone(),
            name: r.1.clone(),
            gender: r.2.clone(),
            profession: r.3.clone(),
            salary: r.4.clone(),
        };
        have_this_user = true;
    });
    match have_this_user {
        true => {
            println!("以下是这个教师的信息");
            println!("工号:\t{}", user.id);
            println!("姓名:\t{}", user.name);
            println!("性别:\t{}", user.gender);
            println!("所教科目:\t{}", user.profession);
            println!("薪水:\t{}\t", user.salary);
        }
        false => {
            println!("没有这个老师的信息，请重新输入");
        }
    }
}
fn super_admin_manage_three_look_teacher_class() {
    println!("请输入当前教师的工号");
    let user_id = tools::scan();
    let sql = format!("select * from teacher  where '{}'", user_id);

    let mut have_this_user: bool = false;
    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
    let mut user: Teacher = Teacher::new();

    connect.query_iter(sql).unwrap().for_each(|row| {
        let r: (String, String, i32, String, f64) = from_row(row.unwrap());
        user = Teacher {
            id: r.0.clone(),
            name: r.1.clone(),
            gender: r.2.clone(),
            profession: r.3.clone(),
            salary: r.4.clone(),
        };
        have_this_user = true;
    });
    match have_this_user {
        true => {
            println!("以下是改老师的所教的课程");
            println!("所教科目:\t{}", user.profession);
        }
        false => {
            println!("没有这个老师的信息，请重新输入");
        }
    }
}
fn super_admin_manage_four_add_student() {
    println!("请输入当前函数")
}
