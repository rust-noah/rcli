use std::{fmt, fs::File, io::Read, path::Path, str::FromStr};

use anyhow::Result;
use clap::Parser;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

// region:    --- data structure
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubcommand),
}

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(
        name = "sign",
        about = "Sign a text with a private/session key and return a signature"
    )]
    Sign(TextSignOpts),
    #[command(
        name = "verify",
        about = "Verify a signature with a public/session key"
    )]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    /// input file
    #[arg(short, long, default_value = "-", value_parser = verify_file)]
    pub input: String,
    /// key file
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    /// format
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    // input 和 key 不能同时为 "-", 在 stdin 中会竞争
    // #[arg(short, long, value_parser = verify_file, default_value = "-")]
    // pub input: String,
    // #[arg(short, long, value_parser = verify_file)]
    // pub key: String,
    // #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    // pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    /// input file
    #[arg(short, long)]
    pub input: String,
    /// key file
    #[arg(short, long)]
    pub key: String,
    /// sig file
    #[arg(short, long)]
    pub sig: String,
    /// format
    #[arg(short, long)]
    pub format: TextSignFormat,
}

pub struct Blake3 {
    key: [u8; 32],
}

// 然后 Ed25519 需要一个 Signer, 还需要一个 Verifier, 因此需要两个结构体
// 这两个是 Ed25519 的签名算法的实现, 可以与其他的签名算法库配合使用(ed25519_dalek, ring-compat)
pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}
// endregion: --- data structure

// region:    --- traits
pub trait TextSigner {
    // 返回值是一个 Vec<u8> 比较通用, 一般 sign 完成是一个签名(signature)
    // 所以返回值用 Vec<u8> 比较合适, 方便进行后续的处理
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}
pub trait TextVerifier {
    // verifier 的结果一般是 true/false, 因此用 bool 作为返回值还是比较合适的
    // 对于一个 hash 值的 verifier, 无非就是用 key 和 input 重新 hash 一遍, 然后比较 ret 是否 == sig
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}
// endregion: --- traits

// region:    --- impls
// 进一步的思考, trait 要实现在类型上, 因此需要一个数据结构来承载这些 trait
// 根据面向对象的原则, 我们需要一个 strut 叫做 Blake3, 里面包含一个 field 叫做 key

// 关于 Blake3 这个对象的思考
// 1. Blake3 相关的逻辑, 都封装在这个对象中, 在处理相关逻辑的时候, 只需要 new 一个对象, 然后调用这个对象相关的方法
impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    // 只要一个类型实现了 AsRef<[u8]>, 那么就可以通过 key.as_ref() 来获取到 &[u8]
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
}
impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes() == sig)
    }
}

impl Ed25519Signer {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
}
impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}
impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&sig[..64]).try_into()?;
        let signature = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
// endregion: --- impls

// region:    --- Parse Value Function
fn parse_text_sign_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

fn verify_file(filename: &str) -> Result<String, anyhow::Error> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(anyhow::anyhow!("File does not exist"))
    }
}
// endregion: --- Parse Value Function

// region:    --- Functions

// 解析成一个 Read
// 1. 无论是 stdin 还是 file, 都实现了 Read trait, 因此返回值为 Box<dyn Read> 就可以解决返回值不一样的问题了
// 2. 通过 anyhow::Result, 我们只需要知道我们的返回值需要什么既可 -> Result<Value Type>
// 3. 可以看成是其他语言正常需要什么返回值, 在 rust 中无非就是多包了一个 Result
// 4. 然后如果返回值的类型不一样, 我们可以用 trait object -> dyn Read
// 5. 由于 trait object 在编译时大小未知, 因此需要用 Box 包裹, Box 是一个指针(编译时大小确定), 指向在堆上分配的内存
/// 通过 input 解析出 reader -> 用于读
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
/// 通过 input 解析出具体内容(Vec<u8>) 类型 -> 直接返回读出来的内容
pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

fn process_text_sign(reader: &mut dyn Read, key: &[u8], format: TextSignFormat) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };
    signer.sign(reader)
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    verifier.verify(reader, sig)
}
// endregion: --- Functions

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        // region:    --- old code
        // SubCommand::Text(subcmd) => match subcmd {
        //     TextSubcommand::Sign(opts) => match opts.format {
        //         // todo!()
        //         // 下次从这里开始, 先查看 opts 类型的内容, 不需要跳转, 鼠标移动上去既可
        //         // 先把逻辑写在这里, 然后吧重复的逻辑再抽取到函数中
        //         // TextSignFormat::Blake3 => {
        //         //     let key = opts.key;
        //         //     let key: &[u8] = key.as_ref();
        //         //     let key = (&(key[..32])).try_into().unwrap();
        //         //     let mut reader = get_reader(&opts.input)?;
        //         //     let mut input = Vec::new();
        //         //     reader.read_to_end(&mut input);
        //         //     let hash = blake3::keyed_hash(key, &input);
        //         //     hash.as_bytes().to_vec();
        //         //     Ok(())
        //         // }
        //         // TextSignFormat::Ed25519 => {
        //         //     let key = opts.key;
        //         //     let key: &[u8] = key.as_ref();
        //         //     let key = (&(key[..32])).try_into().unwrap();
        //         //     let mut reader = get_reader(&opts.input)?;
        //         //     Ok(())
        //         // }

        //         // 思考一下这部分逻辑应该怎么写
        //         // 1. 肯定是需要通过 format 来判断使用哪个签名算法
        //         // 2. 然后通过 key 和 input 进行签名
        //         // 2.1 需要先把 input 读取出来
        //         // 2.1.1 读取 input 有两种形式, 一种是从 stdin 读取, 另一种是从 file 读取
        //         // 2.1.2 stdin 和 file 有一个共同的特点, 他们都是 Read trait 的实现 (可以看官方文档(docsrs))
        //         // 2.2 将 input 读取出来之后, 由于再从文件中读出 key, 通过 input 和 key 进行签名

        //         // 进一步的动作: 可以把相同的行为抽象出来
        //         // 1. 不管是 Blake3 还是 Ed25519, 都需要 sign 和 verify
        //         // 1.1 因此可以抽象成 -> TextSign Trait 和 TextVerify Trait
        //         TextSignFormat::Blake3 => {
        //             // let key = get_content(&opts.key)?;
        //             // let key = (&(key[..32])).try_into()?;
        //             // let input = get_content(&opts.input)?;
        //             // let hash = blake3::keyed_hash(key, &input);
        //             // println!("Signature: {}", hex::encode(hash.as_bytes()));
        //         }
        //         TextSignFormat::Ed25519 => {
        //             let mut signing_key = SigningKey::generate(&mut OsRng);
        //             // sign message
        //             let input = get_content(&opts.input)?;
        //             let signature = signing_key.sign(&input);
        //             println!("Signature: {}", hex::encode(signature.to_bytes()));
        //             // key 呢?
        //         }
        //     },
        //     TextSubcommand::Verify(opts) => {
        //         // println!("Verify: {:?}", opts);
        //         todo!()
        //     }
        // },
        // endregion: --- old code
        SubCommand::Text(subcmd) => match subcmd {
            TextSubcommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                process_text_sign(&mut reader, &key, opts.format)?;
            }
            TextSubcommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = get_content(&opts.sig)?;
                process_text_verify(&mut reader, &key, &sig, opts.format)?;
            }
        },
    }
    Ok(())
}
