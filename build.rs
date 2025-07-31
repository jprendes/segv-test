fn main() {
    let mut cc = cc::Build::new();

    println!("cargo:rerun-if-changed=src/impl/common.c");
    cc.file("src/impl/common.c");

    let file = if std::env::var_os("CARGO_CFG_UNIX").is_some() {
        "src/impl/unix.c"
    } else if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        cc.flag("/std:c11");
        "src/impl/windows.c"
    } else {
        panic!("Unsupported platform");
    };

    println!("cargo:rerun-if-changed={file}");
    cc.file(file);

    cc.compile("segv_test");
}
