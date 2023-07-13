pub fn main() {
    #[cfg(any(target_family = "unix", target_os = "macos"))]
    {
        if pkg_config::find_library("libxml-2.0").is_err() {
            panic!("libxml2 not found");
        }
    }
}
