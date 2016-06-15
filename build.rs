fn main() {
    std::fs::create_dir_all("target/include").unwrap();
    std::fs::copy("cheader/kdri.h", "target/include/kdri.h").unwrap();
}
