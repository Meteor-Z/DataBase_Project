use crate::domain::{Student, Teacher};
use crate::number;
use crate::number::INPUT_ERR_MESSAGE;
use crate::tools;
use chrono::NaiveTime;
use mysql::prelude::Queryable;
use mysql::prelude::*;
use mysql::*;

use std::collections::HashMap;
use std::fmt::format;
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
    loop {
        println!("选项 1 不同职称的教师数量");
        println!("选项 2 不同职称的教师的平均工资");
        println!("选项 3 按班级的成绩列表");
        println!("选项 4 某位学生的学分");
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
            1 => admin_manage_one_unique_teacher_count(),
            2 => admin_manage_two_unique_average_teacher_salayr(),
            3 => admin_manage_three_list_by_class(),
            4 => admin_manage_four_student_list(),
            _ => {
                return;
            }
        }
    }
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
    println!("请输入当前学生的id");
    let user_id = tools::scan();
    let sql = format!("select * from student  where id = '{}'", user_id);
    let mut have_this_user: bool = false;
    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
    connect.query_iter(sql).unwrap().for_each(|row| {
        let _r: (String, String, i32, i32, i32, String) = from_row(row.unwrap());
        have_this_user = true;
    });
    if have_this_user {
        println!("有这名同学的信息了,拒接访问");
        return;
    }
    println!("请输入学生姓名");
    let user_name = tools::scan();
    println!("请输入学生性别 1 表示 男 0 表示 女");
    let user_sex = tools::scan();
    println!("请输入当前学生的年龄");
    let user_age = tools::scan();
    println!("请输入当学生的班级");
    let user_class = tools::scan();
    println!("请输入学生的初始化密码");
    let user_password = tools::scan();
    let sql: String = format!(
        "insert into database_project.student(id, name, sex, age, class, password) values (
        '{}', '{}', {}, {}, {}, '{}')",
        user_id, user_name, user_sex, user_age, user_class, user_password
    );
    connect.query_iter(sql).unwrap(); // 这个是插入

    println!("插入成功!");
}

/*
    println!("选项 1 不同职称的教师数量");
    println!("选项 2 不同职称的教师的平均工资");
    println!("选项 3 按班级的成绩列表");
    println!("选项 4 某位学生的学分");
    println!("选项 5 退出");
*/
fn admin_manage_one_unique_teacher_count() {
    let sql: String = String::from("select * from teacher");
    let mut teacher_count: HashMap<String, i32> = HashMap::new();
    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
    connect.query_iter(sql).unwrap().for_each(|row| {
        let r: (i32, String, i32, String, f64) = from_row(row.unwrap());
        let profession: String = r.3.clone();
        for class in profession.split_whitespace() {
            let number = teacher_count.entry(class.to_string()).or_insert(0);
            *number += 1;
        }
    });
    println!("以下是不同职称的数量");
    for (key, val) in &teacher_count {
        println!("{}\t{}", key, val);
    }
    println!();
}
fn admin_manage_two_unique_average_teacher_salayr() {
    let sql: String = String::from("select * from teacher");

    let mut teacher_count: HashMap<String, i32> = HashMap::new();
    let mut teacher_slary: HashMap<String, f64> = HashMap::new();
    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
    connect.query_iter(sql).unwrap().for_each(|row| {
        let r: (i32, String, i32, String, f64) = from_row(row.unwrap());
        let profession: String = r.3.clone();
        for class in profession.split_whitespace() {
            let number = teacher_count.entry(class.to_string()).or_insert(0);
            *number += 1;

            let slary_count = teacher_slary.entry(class.to_string()).or_insert(0.0);
            *slary_count += r.4;
        }
    });
    println!("以下是相关信息");
    for (key, value) in &teacher_count {
        println!(
            "{}\t{}",
            key,
            *teacher_slary.get(key).unwrap() / ((*value) as f64)
        );
    }
}
fn admin_manage_three_list_by_class() {
    let class_number = tools::scan().parse::<i32>();
    if class_number.is_err() {
        println!("输入错误，请重新输入");
        return;
    }
    let sql: String = String::from("select * from student");
    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接

    println!("以下是相关信息");

    connect.query_iter(sql).unwrap().for_each(|row| {
        let r: (String, String, i32, i32, i32, String) = from_row(row.unwrap());
        let user_id = &r.0;
        let user_name = &r.1;
        let user_sex = &r.2;
        let user_age = &r.3;
        let user_class = &r.4;
        let user_password = &r.5;
        println!(
            "{} {} {} {} {} {}",
            user_id, user_name, user_sex, user_age, user_class, user_password
        );
    });
}
fn admin_manage_four_student_list() {
    println!("请输入学号");
    let user_id = tools::scan();
    let sql: String = format!("select score from class where student_id = '{}'", user_id);
    let pool = mysql::Pool::new(number::URL).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap(); // 获取连接
    let mut score = 0;
    connect.query_iter(sql).unwrap().for_each(|row| {
        let r: i32 = from_row(row.unwrap());
        score += r;
    });
    println!("学分为:{}\t", score);
}
