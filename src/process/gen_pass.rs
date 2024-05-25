use anyhow::Result;
use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPERCASE: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abacdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
    length: u8,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        password.push(*UPPERCASE.choose(&mut rng).unwrap());
    }
    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        password.push(*LOWERCASE.choose(&mut rng).unwrap());
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap());
    }

    for _ in 0..length {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password).unwrap();
    // 打印密码
    print!("{}", password);

    // 打印强度
    let estimate = zxcvbn(&password, &[]).unwrap();
    eprintln!("密码强度: {}", estimate.score());
    Ok(password)
}
