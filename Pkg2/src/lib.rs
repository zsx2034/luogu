// 模块默认是私有的
mod front_of_house {
    // 子模块可以使用所有祖先的模块
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    // 结构体公有的, 字段需要单独设置 pub
    pub struct Order {
        // uuid 公有的
        pub uuid: String,
        // name 私有的
        name: String,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

///////////////////////////////////////////////////////////
//  super
///////////////////////////////////////////////////////////

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        // 子模块调用父模块
        super::serve_order();
    }
}

///////////////////////////////////////////////////////////
//  use
///////////////////////////////////////////////////////////

// 指定相对路径
// use front_of_house::hosting;
// 指定绝对路径
// 可以使用 as 进行本地重命名
use crate::front_of_house::hosting::add_to_waitlist as h_add;
fn use_test() {
    // 使用use之后就不需要前面的那些了，相当于hosting和当前是同级目录
    h_add();
}

// 嵌套 use 简化
use std::{
    io::Cursor,
    collections::*, // 使用通配符
};

