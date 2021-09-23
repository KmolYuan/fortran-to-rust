fn main() {
    cc::Build::new().file("ten/ten.f90").flag("--sysroot=ten").compile("ten");
    // println!("cargo:rustc-link-lib=dylib=gcc");
    // println!("cargo:rustc-link-lib=static=quadmath");
    println!("cargo:rustc-link-lib=dylib=gfortran");
    // println!("cargo:rustc-link-lib=static=advapi32");
}
