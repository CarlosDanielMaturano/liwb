pub fn read_file(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|err| format!("Error reading the file: {err}"))
}
