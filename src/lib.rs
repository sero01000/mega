use aes::Aes128;
use block_modes::{block_padding::NoPadding, BlockMode, Cbc};
use rand::Rng;
use ring::pbkdf2;

const NONZERO100000: std::option::Option<std::num::NonZeroU32> = std::num::NonZeroU32::new(100_000);
static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;

struct Mega {}

impl Mega {
    fn split_salt(start: &str, end: &str, line: &str) -> Option<String> {
        let split1 = line.splitn(2, start).collect::<Vec<&str>>();
        if split1.len() == 2 {
            let split2 = split1[1].splitn(2, end).collect::<Vec<&str>>();
            if split2.len() == 2 {
                Some(split2[0].to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn aes_cbc_encrypt(buffer: &mut [u8], key: [u8; 16]) {
        let cipher = Cbc::<Aes128, NoPadding>::new_from_slices(&key, &[0; 16]).unwrap();
        cipher.encrypt(&mut *buffer, 16).unwrap();
    }

    fn prepare_key(pwd_str: &str) -> [u8; 16] {
        let data: &[u8] = pwd_str.as_bytes();
        let mut pkey: [u8; 16] = [
            0x93, 0xC4, 0x67, 0xE3, 0x7D, 0xB0, 0xC7, 0xA4, 0xD1, 0xBE, 0x3F, 0x81, 0x01, 0x52,
            0xCB, 0x56,
        ];
        for _it in 0..65536 {
            for idx in data.chunks(16) {
                let mut temp: [u8; 16] = [0; 16];
                temp[..idx.len()].copy_from_slice(idx);
                Self::aes_cbc_encrypt(&mut pkey, temp);
            }
        }
        pkey
    }

    fn generate_user_handle(email: &str, key: &[u8; 16]) -> String {
        let email_bytes = email.as_bytes();
        let mut hash: [u8; 16] = [0; 16];

        for i in 0..email_bytes.len() {
            hash[i % 16] ^= email_bytes[i];
        }
        for _ in 0..16384 {
            Self::aes_cbc_encrypt(&mut hash, *key)
        }
        let mut hash_parts = [0u8; 8];
        hash_parts[0..4].copy_from_slice(&hash[0..4]);
        hash_parts[4..8].copy_from_slice(&hash[8..12]);
        let result: String = base64::encode_config(&hash_parts, base64::URL_SAFE_NO_PAD);
        result
    }

    fn hmac_hash(password: &str, salt: &str) -> Result<[u8; 32], std::io::Error> {
        if let Ok(salt_work) = base64::decode(salt) {
            let mut result: [u8; 32] = [0u8; 32];
            pbkdf2::derive(
                PBKDF2_ALG,
                NONZERO100000.unwrap(),
                &salt_work,
                password.as_bytes(),
                &mut result,
            );
            Ok(result)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error in salt",
            ))
        }
    }

    fn post_mega(url: &str, body: String) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let res = client.post(url).body(body).send()?;

        let body_return = res.text()?;
        Ok(body_return)
    }

    pub fn login_email_hash(email: &str, hash: &str, url: &str) -> Result<String, std::io::Error> {
        let post_data = format!(
            "[{{\"a\": \"us\", \"user\": \"{}\", \"uh\": \"{}\"}}]",
            email, hash
        );
        if let Ok(req2) = Self::post_mega(&url, post_data) {
            if req2.contains("[-13]") {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Wrong email or password",
                ))
            } else if req2.contains("\"csid\":\"") {
                Ok(req2)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Else: {:?}", req2),
                ))
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Connection error",
            ))
        }
    }

    pub fn login(email: &str, password: &str) -> Result<String, std::io::Error> {
        let post_data = format!("[{{\"a\":\"us0\",\"user\":\"{}\"}}]", email);
        let url = format!(
            "https://g.api.mega.co.nz/cs?id={}",
            rand::thread_rng().gen_range(0..10000000)
        );
        if let Ok(req1) = Self::post_mega(&url, post_data.to_string()) {
            if req1.contains("\"v\":1") {
                let key = Self::prepare_key(password);
                let hash = Self::generate_user_handle(email, &key);
                Self::login_email_hash(email, &hash, &url)
            } else if req1.contains("\"v\":2") {
                if let Some(salt) = Self::split_salt("\"s\":\"", "\"", &req1) {
                    if let Ok(hash_parsed) = Self::hmac_hash(password, &salt) {
                        let hash =
                            base64::encode_config(&hash_parsed[16..32], base64::URL_SAFE_NO_PAD);
                        Self::login_email_hash(email, &hash, &url)
                    } else {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Generating hash",
                        ))
                    }
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Parsing salt",
                    ))
                }
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No version in post data",
                ))
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Connection error",
            ))
        }
    }
}
