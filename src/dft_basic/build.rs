
fn main() {
    println!("cargo:rustc-cfg=feature=\"logger\"");
    println!("cargo:rustc-cfg=feature=\"basic\"");
}