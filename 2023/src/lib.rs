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

pub fn get_first_number_on_line(l: &str) -> i32 {
    let mut res = String::new();
    let mut it = l.chars().into_iter();
    while let Some(c) = it.next() {
        if c.is_numeric() {
            let mut c = c;
            loop {
                if !c.is_numeric() {
                    break;
                }
                res.push(c);
                c = it.next().unwrap();
            }
            break;
        }
    }
    res.parse().unwrap()
}

pub fn get_numbers_on_line<T>(l: &str) -> Vec<T> 
    where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
    let mut res = Vec::new();
    let mut it = l.chars().into_iter();
    while let Some(c) = it.next() {
        if c.is_numeric() {
            let mut c = Some(c);
            let mut num = String::new();
            loop {
                if c.is_none() || !c.unwrap().is_numeric(){
                    break;
                }
                num.push(c.unwrap());
                c = it.next();
            }
            res.push(num.parse().unwrap());
        }
    }
    res
}
