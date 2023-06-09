pub struct Teacher {
    id: String,         // 工号
    name: String,       // 姓名
    gender: i32,        // 性别
    profession: String, // 职称
    salary: f64,        // 工资
}
pub struct Class {
    id: String,   // 课程号
    name: String, // 课程名
    credit: f64,  // 学分
}
pub struct Student {
    id: String,       // 学号
    name: String,     // 姓名
    sex: i32,         // 性别
    age: i32,         // 年龄
    class: Class,     // 所属班级
    password: String, // 密码
    score: f64,       // 每一门科目的总学分
}
