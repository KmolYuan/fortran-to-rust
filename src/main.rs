extern "C" {
    fn __ten_mod_MOD_ten(input: &libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 20;
    let output = unsafe { __ten_mod_MOD_ten(&input) };
    println!("{} * 10 = {}", input, output);
}
