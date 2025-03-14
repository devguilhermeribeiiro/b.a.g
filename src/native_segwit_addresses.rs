use bech32::{hrp, segwit};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

pub fn generate(public_key: [u8; 33]) -> String {
    // 1. SHA-256 da chave pública
    let sha256_hash = Sha256::digest(public_key);

    // 2. RIPEMD-160 do resultado do SHA-256
    let ripemd160_hash = Ripemd160::digest(sha256_hash);

    // 3. Codificar como endereço SegWit (P2WPKH)
    let hrp = hrp::BC; // Prefixo "bc" para mainnet
    let witness_version = segwit::VERSION_0; // Versão 0 para P2WPKH
    let witness_program = ripemd160_hash.as_slice(); // Witness Program (20 bytes)

    // 4. Gerar o endereço Bech32 usando a API segwit
    let address = segwit::encode(hrp, witness_version, witness_program)
        .expect("Falha ao gerar endereço SegWit");

    address
}
