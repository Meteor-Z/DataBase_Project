pub struct Teacher {
    pub id: String,         // 工号
    pub name: String,       // 姓名
    pub gender: i32,        // 性别
    pub profession: String, // 职称
    pub salary: f64,
}
pub struct Student {
    pub id: String,       // 学号
    pub name: String,     // 姓名
    pub sex: i32,         // 性别
    pub age: i32,         // 年龄
    pub class: i32,       // 所属班级
    pub password: String, // 密码
}
impl Student {
    pub fn new() -> Student {
        Student {
            id: String::new(),
            name: String::new(),
            sex: -1,
            age: -1,
            class: -1,
            password: String::new(),
        }
    }
}
impl Teacher {
    pub fn new() -> Teacher {
        Teacher {
            id: String::new(),
            name: String::new(),
            gender: 0,
            profession: String::new(),
            salary: 0.0,
        }
    }
}
