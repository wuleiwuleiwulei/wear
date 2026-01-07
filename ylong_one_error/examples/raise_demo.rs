use ylong_one_error::*;

define_error! {
    (IO,std::io::Error),
    (ParseInt,std::num::ParseIntError)
}

fn give_me_five(value: u32) -> Result<u32, Error> {
    const MAX_VALUE: u32 = 100;
    if value > MAX_VALUE {
        raise!("value can not lesser than {}, value: {}", MAX_VALUE, value)
    }
    Ok(value + 5)
}

fn main() {
    match give_me_five(110) {
        Ok(v) => println!("thank you, value: {v}"),
        Err(err) => println!("so sad, something wrong, error: {err}"),
    }
}
