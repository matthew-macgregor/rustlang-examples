use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct User {
    id: String,
    loc: String
}

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_user_from_file_test() {
        let u = read_user_from_file("test.json").unwrap();
        println!("{:#?}", u);
        assert_eq!(&u.id, "abcd");
        assert_eq!(&u.loc, "mars");
    }
}