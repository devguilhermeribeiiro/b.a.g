mod native_segwit_addresses;
mod private_key;
mod public_key;
mod segwit_address;
mod wif;

fn main() {
    let private_key = private_key::generate();

    let public_key = public_key::generate(&private_key);

    let wif = wif::generate(&private_key);

    let segwit_address = segwit_address::generate(public_key);

    let native_segwit_address = native_segwit_addresses::generate(public_key);

    println!(
        "Wif: {}\nSegwit address: {:?}\nNative segwit address: {:?}",
        wif, segwit_address, native_segwit_address
    );
}
