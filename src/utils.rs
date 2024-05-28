use std::io::{ErrorKind, Read};

pub fn read_file(path: impl Into<std::path::PathBuf>) -> Result<String, String> {
    let path: std::path::PathBuf = path.into();

    if path.is_dir() {
        return Err(format!("Expected a file, found a directory."));
    }

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(&path)
        .map_err(|err| {
            let path = path.to_str().unwrap();
            match err.kind() {
                ErrorKind::NotFound => format!("No such file or directory: {path}."),
                ErrorKind::PermissionDenied => {
                    format!("Could not open {path}. Permission Denied.")
                }
                _ => format!("An unknown type of error ocurred: {err}"),
            }
        })?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|err| format!("Could not read the file content due a unknown error: {err}"))?;
    Ok(content)
}
