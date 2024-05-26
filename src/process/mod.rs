mod b64;
mod csv_convert;
mod gen_pass;
mod http_serve;
mod jwt;
mod text;

pub use crate::utils::verify_input_file;
pub use b64::{decode_base64, encode_base64};
pub use csv_convert::process_csv;
pub use gen_pass::process_genpass;
pub use http_serve::process_http_serve;
pub use jwt::{process_jwt_sign, process_jwt_verify};
pub use text::{generate_key, process_decrypt, process_encrypt, sign_text, verify_text};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("Cargo.toml1"),
            Err("File does not exists")
        );
    }
}
