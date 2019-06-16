use wakey::WolPacket;

#[derive(Debug)]
enum Error {
	MissingArgument,
	Wakey(wakey::Error)
	// BadFormat,
}

impl std::convert::From<wakey::Error> for Error {
	fn from(error: wakey::Error) -> Self {
		Error::Wakey(error)
	}
}

fn main() -> Result<(), Error> {
	let arg = get_mac()?;
	let wol = WolPacket::from_string(&arg, ':')?;
	dbg!(&wol);
	for _ in 0..get_num() {
		println!("Sent packet ({} bytes)", wol.send_magic()?);
	}

	Ok(())
}

fn get_mac() -> Result<String, Error> {
	std::env::args()
		.nth(1)
		.ok_or(Error::MissingArgument)
}

fn get_num() -> usize {
	std::env::args()
		.nth(2)
		.and_then(|i| i.parse::<usize>().ok())
		.unwrap_or(10)
}
