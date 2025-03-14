use sha2::{Digest, Sha256};

pub fn generate() -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(
        b"Conteudo que voce quer usar como base para a criacao da chave privada com Sha-256",
    );

    let priv_key = hasher.finalize();

    let private_key: [u8; 32] = priv_key.into();

    private_key
}
