use ndarray::{Array2, ShapeBuilder};

extern "C" {
    fn test_(a: *mut f64, n: &i32);
    fn test_2d_(a: *const f64, row: &i32, col: &i32);
}

fn main() {
    // 1d array (slice)
    let mut a = [0.5, 20.25];
    unsafe { test_(a.as_mut_ptr(), &(a.len() as i32)) };
    println!("Rust: {}, {}", a[0], a[1]);
    // 2d array with Fortran memory layout
    let mut a = Array2::zeros([2, 2].f());
    a[[0, 0]] = 11.;
    a[[0, 1]] = 12.;
    a[[1, 0]] = 21.;
    a[[1, 1]] = 22.;
    unsafe { test_2d_(a.as_ptr(), &(a.nrows() as i32), &(a.ncols() as i32)) };
}
