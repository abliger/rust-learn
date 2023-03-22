use std::error::Error;
use std::{env, fs};
pub struct Config<'a> {
    pub search: &'a str,
    pub path: &'a str,
    pub case: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("minigrep 必须出现两个参数");
        }
        let case = env::var("CASE").is_ok();
        println!("{case}");
        Ok(Config {
            search: &args[1],
            path: &args[2],
            case,
        })
    }
}

pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}

pub fn search<'a>(content: &'a str, search: &'a str) -> Vec<&'a str> {
    let mut vec: Vec<&str> = vec![];
    for line in content.lines() {
        if line.contains(search) {
            vec.push(line);
            println!("{line}");
        }
    }
    vec
}
pub fn search_ignore_case<'a>(content: &'a str, search: &'a str) -> Vec<&'a str> {
    let mut vec: Vec<&str> = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(&search.to_lowercase()) {
            vec.push(line);
            println!("{line}");
        }
    }
    vec
}
#[cfg(test)]
mod test {
    use crate::search;

    #[test]
    fn search_test() {
        let str = "Tortor in turpis montes tortor eu gravida est arcu tristique ligula metus elit. \nMassa ullamcorper consequat non, eu dui arcu, sit nascetur vestibulum maximus vitae porta pellentesque luctus tempus consequat tempor elit.\nDui sed enim rhoncus dictum maximus, ante est consectetur tempor, ante praesent. Faucibus nunc sed fusce pretium nulla,\nsed eleifend nisl nulla odio molestie faucibus. Arcu adipiscing ac varius,\nefficitur lorem porttitor, nullam ipsum eros magna varius turpis ut dignissim finibus nisi ac morbi nulla lorem suspendisse ultrices eros dapibus,\nconvallis. Amet duis fermentum posuere vel vestibulum porta aenean varius aliquet et vel id suspendisse integer vitae aenean.";
        assert_eq!(search(str, "in"),vec!["Tortor in turpis montes tortor eu gravida est arcu tristique ligula metus elit. ",
        "sed eleifend nisl nulla odio molestie faucibus. Arcu adipiscing ac varius,","efficitur lorem porttitor, nullam ipsum eros magna varius turpis ut dignissim finibus nisi ac morbi nulla lorem suspendisse ultrices eros dapibus,","convallis. Amet duis fermentum posuere vel vestibulum porta aenean varius aliquet et vel id suspendisse integer vitae aenean."])
    }
}
