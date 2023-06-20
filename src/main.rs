mod cat_command;
use crate::cat_command::{print_cat, print_msg_box};
use std::env;

fn main() {
    // print_msg_box("You think <insert person's name here> is very hot and you like them very much.");
    // print_cat();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("NO! BAD USER!");
    } else {
        print_msg_box(&args[1]);
        print_cat();
    }
}
