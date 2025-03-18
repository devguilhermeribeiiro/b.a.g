mod legacy_address;
mod native_segwit_addresses;
mod private_key;
mod public_key;
mod wif;

fn main() {
    let private_key = private_key::generate();

    let public_key = public_key::generate(&private_key);

    let wif = wif::generate(&private_key);

    let legacy_address = legacy_address::generate(public_key);

    let native_segwit_address = native_segwit_addresses::generate(public_key);

    println!(
        "Wif: {}\nLegacy_address address: {:?}\nNative segwit address: {:?}",
        wif, legacy_address, native_segwit_address
    );
}
