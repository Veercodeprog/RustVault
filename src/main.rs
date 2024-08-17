use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

fn main() {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("Unable to access OS random generator");
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    println!("Private Key: {:?}", secret_key);
    println!("Public Key: {:?}", public_key);
}
