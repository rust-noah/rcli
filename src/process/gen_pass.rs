use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

// region:    --- Code before decoupling
// pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<String> {
//     let mut rng = rand::thread_rng();
//     let mut password = Vec::new();
//     let mut chars = Vec::new();

//     if opts.uppercase {
//         chars.extend_from_slice(UPPER);
//         password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
//     }
//     if opts.lowercase {
//         chars.extend_from_slice(LOWER);
//         password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
//     }
//     if opts.number {
//         chars.extend_from_slice(NUMBER);
//         password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
//     }
//     if opts.symbol {
//         chars.extend_from_slice(SYMBOL);
//         password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
//     }

//     for _ in 0..(opts.length - password.len() as u8) {
//         let c = chars
//             .choose(&mut rng)
//             .expect("chars won't be empty in this context");
//         password.push(*c);
//     }

//     password.shuffle(&mut rng);

//     Ok(String::from_utf8(password)?)
// }
// endregion: --- Code before decoupling

// 把数据结构拆解出来, 不与 opts.rs 产生耦合
// 好处: 下次想把这部分逻辑拆解分来, 放到其他 repo 的时候, 就会比较方便了
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    // Detecting in the executor!
    // let estimate = zxcvbn(&password, &[]);
    // println!("Password strength: {}", estimate.score());

    Ok(password)
}
