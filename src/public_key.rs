use secp256k1::{PublicKey, Secp256k1, SecretKey};

/// Gera uma chave pública comprimida `[u8; 33]` a partir da chave privada em hexadecimal
pub fn generate(priv_key: &[u8; 32]) -> [u8; 33] {
    let secp = Secp256k1::new();

    if priv_key.len() != 32 {
        panic!("A chave privada deve ter exatamente 32 bytes");
    }

    let private_key = SecretKey::from_slice(priv_key).expect("Chave privada inválida");
    let public_key = PublicKey::from_secret_key(&secp, &private_key);
    public_key.serialize()
}
