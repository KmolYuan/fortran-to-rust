fn main() {
    cc::Build::new().file("src/ten.f90").compile("libten.a");
    // println!("cargo:rustc-link-lib=dylib=gcc");
    // println!("cargo:rustc-link-lib=static=quadmath");
    println!("cargo:rustc-link-lib=dylib=gfortran");
    // println!("cargo:rustc-link-lib=static=advapi32");
}
