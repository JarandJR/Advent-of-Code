pub fn read_file_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    Ok(data)
}
