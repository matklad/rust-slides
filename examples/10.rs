#[link(name = "extlib")]
extern {
    fn register_callback(cb: extern fn(i32)) -> i32;
}

extern fn callback(a: i32) {
    println!("I'm called from C with value {0}", a);
}

fn main() {}
