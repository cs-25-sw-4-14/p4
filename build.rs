fn main() {
    println!("cargo:rerun-if-changed=src/lossy.lalrpop");
    lalrpop::process_root().unwrap();
}
