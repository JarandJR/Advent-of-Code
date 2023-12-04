pub fn read_file_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    Ok(data)
}

pub trait Parse {
    fn parse(line: &str) -> Self;
    }

pub fn parse_line<T: Parse>(line: &str) -> T {
    T::parse(line)
}

pub fn get_data_list<T: Parse>(data: String) -> Vec<T> {
    data.lines().into_iter()
    .map(|l| parse_line(l))
    .collect()
}

pub fn get_first_number(l: &str) -> i32 {
    let mut res = String::new();
    for c in l.chars().into_iter() {
        if c.is_numeric() {
            res.push(c);
        } else if c == ':' {
            break;
        }
    }
    res.parse().unwrap()
}
