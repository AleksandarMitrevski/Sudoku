use std::path::PathBuf;

pub fn is_numeric(val : &str) -> bool {
    for c in val.chars() {
        if !c.is_digit(10) {
            return false;
        }
    }
    true
}

pub fn exe_dir() -> PathBuf {
    use std::env::current_exe;
    let mut path = current_exe().expect("can not get EXE directory");
    path.pop();   // strips EXE name path component
    path
}