mod management;
mod number;
mod tools;

fn main() {

    management::first_management();
    let a = tools::scan();
}

// 测试一下数据库能否正常使用
#[test]
fn test() {
    
}