mod cat_command;
use crate::cat_command::{print_cat, print_msg_box};

fn main() {
    print_msg_box("You think <insert person's name here> is very hot and you like them very much.");
    print_cat();
}
