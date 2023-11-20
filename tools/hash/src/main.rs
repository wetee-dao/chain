use sp_core::hashing::twox_128;

fn main() {
	let arr = twox_128(&b"PacsDeposit"[..]).to_vec();
	println!("{:?}", hex::encode(arr.clone()));

	let arr2 = twox_128(&b"ComOfReport"[..]).to_vec();
	println!("{:?}", hex::encode(arr2.clone()));

	let arr4 = twox_128(&b"PacsDepositComOfReport"[..]).to_vec();
	println!("{:?}", hex::encode(arr4.clone()));
	let arr3 = [twox_128(b"PacsDeposit"), twox_128(b"ComOfReport")].concat();
	println!("{:?}", hex::encode(arr3.clone()));

	let hash_real2 = [twox_128(b"11113")].concat();
	println!("11113 {:?}", hex::encode(hash_real2.clone()));

	let hash_real3 = [twox_128(b"PacsDeposit"), twox_128(b"Reports")].concat();
	println!("{:?}", hex::encode(hash_real3.clone()));

	let hash_real = [
		twox_128(b"PacsDeposit"),
		twox_128(b"Reports"),
		twox_128(b"map"),
	]
	.concat();
	println!("{:?}", hex::encode(hash_real.clone()));

	let hash_real2 = [twox_128(b"Art"), twox_128(b"Arts")].concat();
	println!("{:?}", hex::encode(hash_real2.clone()));
}
