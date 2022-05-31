
fn main() {
    println!("cargo:rustc-cfg=feature=\"logger\"");
    println!("cargo:rustc-cfg=feature=\"basic\"");
    println!("cargo:rustc-cfg=feature=\"burnable\"");
    println!("cargo:rustc-cfg=feature=\"mintable\"");
    println!("cargo:rustc-cfg=feature=\"batch_mint\"");
    println!("cargo:rustc-cfg=feature=\"batch_transfer\"");
}