use util_library::rc;
use util_library::utils::utils;

fn main() {
    println!("Hello, world!");
    println!("output: {}", utils::get_name());
    println!("output1: {}", rc::rc_function());
}
