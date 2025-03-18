use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

fn double_hash(data: Vec<u8>) -> Vec<u8> {
    let mut temp = data;

    for _ in 0..2 {
        let mut hasher = Sha256::new();
        hasher.update(temp);
        temp = hasher.finalize().to_vec(); // Converte o resultado para Vec<u8>
    }

    temp[0..4].to_vec() // Retorna os primeiros 4 bytes como o checksum
}

pub fn generate(pub_key: [u8; 33]) -> String {
    let version: Vec<u8> = vec![0x00]; // Versão 0x00 para o endereço Bitcoin
    let public_key_bytes = pub_key;

    // Calcula o hash SHA-256 da chave pública
    let hashed_by_sha256 = Sha256::digest(&public_key_bytes);

    // Calcula o hash RIPEMD-160 do hash SHA-256
    let hashed_by_ripemd160 = Ripemd160::digest(&hashed_by_sha256);

    // Concatena a versão com o hash160
    let mut hash160_with_version = version;
    hash160_with_version.extend_from_slice(&hashed_by_ripemd160);

    // Calcular o checksum (primeiros 4 bytes do SHA256(SHA256))
    let checksum = double_hash(hash160_with_version.clone()); // Passa a versão + hash160

    // Concatenar hash160 com versão e checksum
    let mut hash160_with_version_and_checksum = hash160_with_version;
    hash160_with_version_and_checksum.extend_from_slice(&checksum);

    // Codificar em Base58
    let address = bs58::encode(hash160_with_version_and_checksum).into_string();

    address
}
