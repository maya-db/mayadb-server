use std::env::current_dir;


pub fn mount_path(p: &str) -> String {
    let path = current_dir().unwrap();

    let path = path.to_str().unwrap();

    format!(
        "{path}{p}",
    )
}