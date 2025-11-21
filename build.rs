fn main() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("mmx") {
            println!("cargo::rustc-check-cfg=cfg(__MMX__)");
            println!("cargo::rustc-cfg=__MMX__");
        }
    }
}
