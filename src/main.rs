mod wallet;
use anyhow::Result;

fn main() -> Result<()> {
    let (secret_key, pub_key) = wallet::generate_keypair();

    println!("secret key: {}", &secret_key.to_string());
    println!("public key: {}", &pub_key.to_string());

    let pub_address = wallet::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);
    let crypto_wallet = wallet::Wallet::new(&secret_key, &pub_key);
    println!("crypto_wallet: {:?}", &crypto_wallet);

    let wallet_file_path = "crypto_wallet.json";
    crypto_wallet.save_to_file(wallet_file_path)?;
    let loaded_wallet = wallet::Wallet::from_file(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);

    Ok(())
}
