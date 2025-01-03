use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPER);
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
    }
    if numbers {
        chars.extend_from_slice(NUMBER);
    }
    if symbols {
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);
    let password = String::from_utf8(password.clone())?;
    // println!("{:?}", password.clone());
    let estimate = zxcvbn(&password, &[]);
    println!("{}", estimate.score());

    Ok(String::from_utf8(password.into())?)
}
