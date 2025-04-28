use aes::Aes128;
use aes::cipher::{BlockDecryptMut, KeyIvInit, block_padding::Pkcs7};
use base64::{
    Engine as _, engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD,
};
use csv::Reader;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str;

/// New type for AES128 CBC decryptor.
type Aes128CbcDec = cbc::Decryptor<Aes128>;

/// CSV file structure:
/// headers: name,message
/// values in quotes `"`
#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct Record {
    name: String,
    message: String,
}

/// RocketChat Message structure
#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct Message {
    msg: String,
}

/// Parses CSV file and decrypt each message.
fn read_csv<P: AsRef<Path>>(filename: P, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);

    // Iterate CSV file line by line
    for result in &mut rdr.deserialize() {
        let record: Record = result?;
        // Try to trim message from quotes
        let msg_b64 = record.message.replace('"', "");
        // Get complex message
        let compl = STANDARD.decode(&msg_b64[12..msg_b64.len()])?;
        // Get vector from conplex
        let iv = &compl[0..16];
        // Get encrypted message from conplex
        let ciphertext = &compl[16..compl.len()];
        let decryptor = Aes128CbcDec::new_from_slices(key, iv)?;
        // Decrypt message
        let plaintext = decryptor
            .decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
            .map_err(|e| format!("Ciphertext decryption error: {}", e))?;
        let plaintext = str::from_utf8(&plaintext)?;
        let msg = serde_json::from_str::<Message>(plaintext)?;
        // Print decrypted message
        println!("{}-> {}", record.name, msg.msg);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    // println!("{:?}", args);
    args.next().expect("No arguments");

    // Path to private RSA key in PEM format
    let privkey_path = args.next().expect("No arguments");

    // Path to session key in Base64URI format encoded for RSA key above
    let sesskey_path = args.next().expect("No arguments");

    // Path to csv file with encrypted messages
    let msgscsv_path = args.next().expect("No arguments");

    // Open the private key file and read the contents into a string
    let mut privkey_pem = String::new();
    File::open(privkey_path)
        .map_err(|e| format!("Could not open private key file {}", e))?
        .read_to_string(&mut privkey_pem)?;

    let private_key = RsaPrivateKey::from_pkcs1_pem(&privkey_pem)
        .map_err(|e| format!("RSA-key import failed: {}", e))?;

    // Open the session key file and read the contents into a string
    let mut sesskey_b64uri: String = String::new();
    File::open(sesskey_path)
        .map_err(|e| format!("Could not open session key file {}", e))?
        .read_to_string(&mut sesskey_b64uri)?;

    // dbg!(sesskey_b64uri.clone());

    // Decode sesskey with URL-safe Base64 encoding
    let sesskey_enc = URL_SAFE_NO_PAD
        .decode(sesskey_b64uri.trim())
        .map_err(|e| format!("Session key decoding failed:  {}", e))?;

    // Decode sesskey with private RSA key
    let sesskey_bin = private_key
        .decrypt(Pkcs1v15Encrypt, &sesskey_enc)
        .map_err(|e| format!("Session key decryption failed: {}", e))?;

    // Open the message csv file and read the contents line by line
    read_csv(msgscsv_path, &sesskey_bin)?;

    // file.write(&URL_SAFE_NO_PAD.decode(&input_string.as_bytes()).unwrap())
    Ok(())
}
