#[link(name="mylib.dll")]
extern "C" {
    fn lib_fn();
}

fn main() {
    println!("Hello, world!");
    unsafe { lib_fn() };
}

