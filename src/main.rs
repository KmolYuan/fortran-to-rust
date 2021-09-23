extern "C" {
    fn test_(a: *mut f64, n: *const i32);
}

fn main() {
    let mut a = [0.5, 20.25];
    unsafe { test_(a.as_mut_ptr(), &2) };
    println!("Rust: {}, {}", a[0], a[1]);
}
