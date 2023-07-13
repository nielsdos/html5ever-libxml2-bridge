pub fn main() {
    if pkg_config::find_library("libxml-2.0").is_err() {
        panic!("libxml2 not found");
    }
}
