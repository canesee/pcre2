#![allow(dead_code, non_camel_case_types)]

mod bindings;
mod ffi;
mod regex;

use regex::Regex;

fn main() {
    let pattern = r"\d\d\d\d[^0-9\s]{3,11}[\S]";
    let subject = b"a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopiqa=)*(^!@#$%^&*())9999999";
    let regex = Regex::new(pattern).expect("regex new error");
    for result in regex.find_iter(subject) {
        let m = result.unwrap();
        println!(
            "{:?}",
            std::str::from_utf8(&subject[m.start() + 4..m.end() - 1])
        );
    }
}
