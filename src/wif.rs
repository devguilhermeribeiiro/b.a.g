use sha2::{Digest, Sha256};

fn double_hash(data: Vec<u8>) -> Vec<u8> {
    let mut temp = data;

    for _ in 0..2 {
        let mut hasher = Sha256::new();
        hasher.update(temp);
        temp = hasher.finalize().to_vec();
    }

    temp[0..4].to_vec()
}

pub fn generate(private_key: &[u8; 32]) -> String {
    let mut uncoded_wif = vec![0x80];
    uncoded_wif.extend_from_slice(private_key);

    let checksum = double_hash(uncoded_wif.clone());

    uncoded_wif.extend_from_slice(&checksum);

    bs58::encode(uncoded_wif).into_string()
}
