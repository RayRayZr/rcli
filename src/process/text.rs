use crate::process_genpass;
use crate::text::TextSignFormatter;
use crate::utils::{read_file, read_input};
use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

trait TextSign {
    fn sign(&self, data: Vec<u8>) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, data: Vec<u8>, sig: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

impl TextSign for Blake3 {
    fn sign(&self, data: Vec<u8>) -> Result<Vec<u8>> {
        Ok(blake3::keyed_hash(&self.key, &data).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, data: Vec<u8>, sig: &[u8]) -> Result<bool> {
        Ok(blake3::keyed_hash(&self.key, &data).as_bytes() == sig)
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

impl TextSign for Ed25519Signer {
    fn sign(&self, data: Vec<u8>) -> Result<Vec<u8>> {
        let sig = self.key.sign(&data);
        Ok(sig.to_vec())
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, data: Vec<u8>, sig: &[u8]) -> Result<bool> {
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&data, &sig).is_ok())
    }
}

pub fn sign_text(input: &str, key: &str, formatter: TextSignFormatter) -> anyhow::Result<String> {
    let input_data = read_input(input)?;
    let key_data = read_file(key)?;
    let siner: Box<dyn TextSign> = match formatter {
        TextSignFormatter::Blake3 => Box::new(Blake3 {
            key: (&key_data[..32]).try_into()?,
        }),
        TextSignFormatter::Ed25519 => Box::new(Ed25519Signer {
            key: SigningKey::from_bytes((&key_data[..32]).try_into()?),
        }),
    };
    let res = siner.sign(input_data)?;
    let res = STANDARD.encode(res);
    println!("{}", res);
    Ok(res)
}

pub fn verify_text(
    input: &str,
    key: &str,
    sign: &str,
    formatter: TextSignFormatter,
) -> Result<bool> {
    let input_data = read_input(input)?;
    let key_data = read_file(key)?;
    let sign_data = read_file(sign)?;
    let real_sig = STANDARD.decode(sign_data.as_slice())?;
    let verifyer: Box<dyn TextVerify> = match formatter {
        TextSignFormatter::Blake3 => Box::new(Blake3 {
            key: key_data[..32].try_into()?,
        }),
        TextSignFormatter::Ed25519 => Box::new(Ed25519Verifier {
            key: VerifyingKey::from_bytes((&key_data[..64]).try_into()?)?,
        }),
    };
    verifyer.verify(input_data, &real_sig)
}

pub fn generate_key(formatter: TextSignFormatter, output_path: &str) -> anyhow::Result<String> {
    match formatter {
        TextSignFormatter::Blake3 => {
            let key = process_genpass(true, true, true, true, 32)?;
            std::fs::write(output_path, key.as_bytes())?;
            Ok(key)
        }
        TextSignFormatter::Ed25519 => {
            let key = process_genpass(true, true, true, true, 32)?;
            std::fs::write(output_path, key.as_bytes())?;
            Ok(key)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};
    #[test]
    fn test_text_sign() -> Result<()> {
        let mut path = env::current_dir()?;
        path.push("fixtrues");
        path.push("test_data.txt");
        let filepath = path.display().to_string();
        path.pop();
        path.push("key.txt");
        let keypath = path.display().to_string();

        let format = TextSignFormatter::Blake3;
        let sig = sign_text(&filepath, &keypath, format)?;
        path.pop();
        path.push("sig.txt");
        let sigpath = path.display().to_string();
        fs::write(&sigpath, sig)?;
        let ret = verify_text(&filepath, &keypath, &sigpath, format)?;
        assert!(ret);
        Ok(())
    }
}
