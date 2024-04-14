use super::key::CONST_KEY;
use super::RequestData;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Key};
use imlogger::*;
use rand::{thread_rng, Rng};
use rconn::server::serde_json::from_str;
use sha2::{Digest, Sha256};

pub fn decrypt(data: &Vec<u8>, key: &String) -> Result<RequestData, ()> {
    if key.len() != 32 {
        error!("Key Len is not 32!");
        return Err(());
    }
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hasher.update(CONST_KEY);
    let hash = hex::encode(hasher.finalize()).to_lowercase();
    let akey = &hash[2..18];
    let nonce = &hash[6..18];
    let akey = Key::<Aes128Gcm>::from_slice(akey.as_bytes());
    let chiper = <Aes128Gcm>::new(&akey);

    let de = chiper.decrypt(nonce.as_bytes().into(), data.as_ref());
    let de = match de {
        Ok(de) => de,
        Err(e) => {
            debug!("Aes Error: {}", e.to_string());
            return Err(());
        }
    };
    let de = String::from_utf8(de).unwrap();

    match from_str(&de) {
        Ok(v) => Ok(v),
        Err(_) => {
            error!("Deserialize data Failed");
            debug!("Error data: {}", de);
            Err(())
        }
    }
}

pub fn encrypt(data: &Vec<u8>) -> Result<(Vec<u8>, String), ()> {
    let key = gen_key();
    let mut hasher = Sha256::new();
    hasher.update(&key);
    hasher.update(CONST_KEY);
    let hash = hex::encode(hasher.finalize()).to_lowercase();
    let akey = &hash[2..18];
    let nonce = &hash[6..18];
    let akey = Key::<Aes128Gcm>::from_slice(akey.as_bytes());
    let chiper = Aes128Gcm::new(&akey);

    let en = chiper.encrypt(nonce.as_bytes().into(), data.as_ref());
    let en = match en {
        Ok(en) => en,
        Err(_) => return Err(()),
    };
    Ok((en, String::from_utf8(Vec::from(key)).unwrap()))
}

fn gen_key() -> [u8; 32] {
    let mut rng = thread_rng();
    let mut arr = [0u8; 32];
    for i in 0..32 {
        arr[i] = rng.gen_range(65u8..(65u8 + 25u8));
    }
    arr
}
