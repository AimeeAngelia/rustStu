mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();
//     //绝对路径
//     front_of_house::hosting::add_to_waitlist();
//     //相对路径
// }

use crate::front_of_house::hosting;
mod customer {
    // use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist();
        super::hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result {}

fn function2() -> io::Result<()> {}

use std::io::{self, Write};
