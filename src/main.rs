use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rsa::pkcs8::DecodePrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    // println!("{:?}", args);
    args.next().expect("No arguments");

    // Path to private RSA key in PEM format
    let privkey_path = args.next().expect("No arguments");

    // Path to session key in Base64URI format encoded for RSA key above
    let sesskey_path = args.next().expect("No arguments");

    // Path to csv file with encrypted messages
    let _msgscsv_path = args.next().expect("No arguments");

    // Open the private key file and read the contents into a string
    let mut privkey_pem = String::new();
    File::open(privkey_path)
        .map_err(|e| format!("Could not open private key file {}", e))?
        .read_to_string(&mut privkey_pem)?;

    let private_key = RsaPrivateKey::from_pkcs8_pem(&privkey_pem)
        .map_err(|e| format!("RSA-key import failed: {}", e))?;

    // Open the session key file and read the contents into a string
    let mut sesskey_b64uri: String = String::new();
    File::open(sesskey_path)
        .map_err(|e| format!("Could not open session key file {}", e))?
        .read_to_string(&mut sesskey_b64uri)?;

    // Decode sesskey with URL-safe Base64 encoding
    let sesskey_enc = URL_SAFE_NO_PAD
        .decode(sesskey_b64uri)
        .map_err(|e| format!("Session key decoding failed:  {}", e))?;

    // Decode sesskey with private RSA key
    let _sesskey_bin = private_key
        .decrypt(Pkcs1v15Encrypt, &sesskey_enc)
        .map_err(|e| format!("Session key decryption failed: {}", e))?;

    // Open the message csv file and read the contents line by line

    // file.write(&URL_SAFE_NO_PAD.decode(&input_string.as_bytes()).unwrap())
    Ok(())
}
