fn main() {
    cc::Build::new().file("src/main.f90").flag("--sysroot=test").compile("test");
    println!("cargo:rustc-link-lib=dylib=gfortran");
}
