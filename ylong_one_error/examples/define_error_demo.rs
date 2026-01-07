use ylong_one_error::*;

define_error! {
    (IO,std::io::Error),
    (ParseInt,std::num::ParseIntError)
}

fn read_value(path: &str) -> Result<u32, Error> {
    let path = std::path::Path::new(path);
    let content = std::fs::read_to_string(path)?;
    Ok(content.parse::<u32>()?)
}

fn main() {
    let value = read_value("");
    match value {
        Ok(v) => println!("read value successfully, value: {v}"),
        Err(err) => match err {
            Error::IO(err) => println!("read value got io-error, err: {err}"),
            Error::ParseInt(err) => println!("parse value got error, err: {err}"),
            Error::CustomMessage(_) => todo!(),
        },
    }
}
