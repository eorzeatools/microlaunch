// persistent data storage
//
// WARNING: THIS MODULE STORES SENSITIVE INFORMATION!
// THIS MODULE STORES YOUR SQUARE ENIX PASSWORD AND TOKENS.
// IT IS STORED IN ENCRYPTED FORM, BUT IT IS STILL STORED.

use std::{path::PathBuf, time::SystemTime};

use aes_gcm::{Key, Aes128Gcm, Nonce};
use aes_gcm::NewAead;
use aes_gcm::aead::Aead;
use rand::{SeedableRng, RngCore};
use rand_chacha::ChaCha20Rng;
use serde::{Serialize, Deserialize};

lazy_static::lazy_static! {
    pub static ref PERSISTENT: EncryptedPersistentData = {
        init_persistent_data()
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedPersistentData {
    pub autologin: bool,
    pub sqex_id: String,
    pub password: String,
}

fn get_secure_rng() -> ChaCha20Rng {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    ChaCha20Rng::seed_from_u64(since_epoch.as_secs())
}

// 16 bytes. AES-128 key.
fn get_actual_hwid() -> Vec<u8> {
    if cfg!(target_os="linux") {
        // This is secure, I think
        // (probably not what you want to hear when dealing with encryption)
        let machine_id = std::fs::read_to_string("/etc/machine-id").unwrap();
        let machine_id_bin = data_encoding::HEXLOWER.decode(machine_id.trim().as_bytes()).unwrap();
        machine_id_bin
    } else {
        // THIS FILE CONTAINS A HOPEFULLY UNIQUE SYSTEM ID
        // IT SHOULD NOT CHANGE OTHERWISE THE ENCRYPTION WILL BREAK
        let enc_path = get_encrypted_file_path().parent().unwrap().join("system_id");
        if !enc_path.exists() {
            let mut secure_rng = get_secure_rng();
            let id_low = secure_rng.next_u64();
            let id_high = secure_rng.next_u64();
            let mut vec_of_key = Vec::new();

            let low_bytes = id_low.to_le_bytes();
            let high_bytes = id_high.to_le_bytes();

            vec_of_key.append(&mut low_bytes.iter().collect::<Vec<_>>());
            vec_of_key.append(&mut high_bytes.iter().collect::<Vec<_>>());
            let vec_of_key = vec_of_key.iter().map(|x| *x.clone()).collect::<Vec<u8>>();
            std::fs::create_dir_all(enc_path.parent().unwrap()).unwrap();
            std::fs::write(enc_path, data_encoding::HEXLOWER.encode(&vec_of_key)).unwrap();
            vec_of_key
        } else {
            let key_str = std::fs::read_to_string(enc_path).unwrap();
            let id_bin = data_encoding::HEXLOWER.decode(key_str.as_bytes()).unwrap();
            id_bin
        }
    }
}

fn decrypt_binary_data(nonce: &str, binary_data: &str) -> EncryptedPersistentData {
    let encrypted_binary = data_encoding::BASE64.decode(binary_data.as_bytes()).unwrap();

    let nonce_bin = data_encoding::HEXLOWER.decode(nonce.as_bytes()).unwrap();    

    let hwid = get_actual_hwid();
    let key = Key::from_slice(&hwid);
    let cipher = Aes128Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bin);

    let decrypt = cipher.decrypt(nonce, encrypted_binary.as_slice()).expect("decryption error");

    bincode::deserialize::<EncryptedPersistentData>(&decrypt).expect("decode error")
}

fn encrypt_persistent_data(data: &EncryptedPersistentData) -> String {
    let mut secure_rng = get_secure_rng();
    let mut nonce: [u8; 12] = [0u8; 12];
    for i in 0..12 {
        nonce[i] = (secure_rng.next_u32() & 0xFF) as u8;
    }
    // needs to be 96-bit
    let base_bin = bincode::serialize(data).unwrap();
    let hwid = get_actual_hwid();
    let key = Key::from_slice(&hwid);
    let nonce = Nonce::from_slice(&nonce);
    let aes = Aes128Gcm::new(key);
    let encrypt = aes.encrypt(nonce, base_bin.as_slice()).expect("encryption error");
    
    let mut out = String::new();
    out += &data_encoding::HEXLOWER.encode(nonce);
    out += "\n";
    out += &data_encoding::BASE64.encode(&encrypt);

    out
}

fn flush_persistent_data(data: &EncryptedPersistentData) -> std::io::Result<()> {
    let file_location = get_encrypted_file_path();
    let encrypted_data = encrypt_persistent_data(data);
    std::fs::write(file_location, encrypted_data)
}

mod tests {
    #[test]
    pub fn test_get_hwid() {
        let hwid = super::get_actual_hwid();
        println!("{}", data_encoding::HEXLOWER.encode(&hwid));
    }

    #[test]
    pub fn test_persistent() {
        super::init_persistent_data();
    }

    #[test]
    pub fn test_encrypt() {
        let data = super::EncryptedPersistentData {
            autologin: true,
            sqex_id: "penis".into(),
            password: "thepenisman420".into()
        };
        let encrypt = super::encrypt_persistent_data(&data);
        println!("{}", encrypt);
    }

    #[test]
    pub fn test_decrypt() {
        let b64 = "JdMQj1bGZPLclUlb8fb3hldo/oUCEGZoO1RQ0rYznzLaZsWVG7m3Qf5cqSL93VWrA1oZ5w==";
        let nonce = "d8dc83325fae274016f72f35";
        // This test only works on my computer
        // Too bad
        let decrypt = super::decrypt_binary_data(nonce, b64);
        println!("{:#?}", decrypt);
    }
}

fn init_persistent_data() -> EncryptedPersistentData {
    // /*
    let path = get_encrypted_file_path();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    if !path.exists() {
        // File does not exist
        let data = EncryptedPersistentData {
            autologin: false,
            sqex_id: "".into(),
            password: "".into()
        };

        flush_persistent_data(&data).expect("Failed to save encrypted data file!");

        data
    } else {
        // Decrypt the file
        let file = std::fs::read_to_string(path).expect("Failed to read file!");
        let split = file.split("\n").collect::<Vec<&str>>();
        let nonce = split[0];
        let binary_data = split[1];

        decrypt_binary_data(nonce, binary_data)
    }
    // */
}

fn get_default_encrypted_path() -> PathBuf {
    dirs::config_dir()
        .expect("You do not have a config directory!? What!?")
        .join("microlaunch")
        .join("sensitive_data.enc")
}

pub fn get_encrypted_file_path() -> PathBuf {
    if let Some(experimental) = &crate::config::CONFIG.experimental {
        if let Some(path) = &experimental.encrypted_data_path {
            PathBuf::from(path).join("sensitive_data.enc")
        } else {
            get_default_encrypted_path()
        }
    } else {
        get_default_encrypted_path()
    }
}