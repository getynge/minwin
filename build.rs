fn main() {
    let deps = vec![
        "user32",
        "kernel32"
    ];

    println!("cargo:rustc-link-arg=/entry:_start");
    println!("cargo:rustc-link-arg=/subsystem:windows");

    for d in deps {
        println!("cargo:rustc-link-lib={}", d);
    }
}