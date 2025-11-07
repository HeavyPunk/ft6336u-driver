fn main() {
    if let Ok(val) = std::env::var("I2C_DEV") {
        println!("cargo:rustc-env=I2C_DEV={}", val);
    }
    if let Ok(val) = std::env::var("UINPUT_DEV") {
        println!("cargo:rustc-env=UINPUT_DEV={}", val);
    }
}
