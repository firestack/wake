use std::net::SocketAddr;
use structopt::StructOpt;
use wakey::WolPacket;

type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt, Debug)]
#[structopt(name = "MAC")]
struct Opt {
	#[structopt(short = "n", long = "num_packets", default_value = "10")]
	number_of_packets: usize,

	#[structopt(short = "d", long = "destination", default_value = "255.255.255.255:9")]
	destination: SocketAddr,

	#[structopt(name = "MAC", min_values = 1, required = true)]
	mac: Vec<String>,

	#[structopt(
		short = "s",
		short = "src",
		long = "source",
		default_value = "0.0.0.0:0"
	)]
	source: SocketAddr,

	#[structopt(short = "v", parse(from_occurrences), required = false)]
	pub verbosity: usize,
}

#[derive(Debug)]
enum Error {
	// MissingArgument,
	Wakey(wakey::Error), // BadFormat
}

impl std::convert::From<wakey::Error> for Error {
	fn from(error: wakey::Error) -> Self {
		Error::Wakey(error)
	}
}

fn main() -> Result<()> {
	let args = Opt::from_args();
	// I'd make this const but all the functions are ctypes
	// and are not const compatable so I can't declare a IP address
	// in code as const
	for mac_addr in &args.mac {
		let (name, mac) = match mac_addr
			.find("=")
			.map(|eq_index| mac_addr.split_at(eq_index))
		{
			Some((name, mac)) => (Some(name), mac.trim_start_matches('=')),
			None => (None, mac_addr.as_ref()),
		};

		if let Err(e) = send_packets(mac, name, &args) {
			println!("\t[FAILED({})] error message: {:?}", mac_addr, e);
		}
	}
	Ok(())
}

fn send_packets(mac: &str, computer_name: Option<&str>, args: &Opt) -> Result<()> {
	println!(
		"Sending {n} packets to computer({computer_name}={comp}) via {dst}",
		n = args.number_of_packets,
		comp = mac,
		computer_name = computer_name.unwrap_or(""),
		dst = args.destination
	);

	let wol = WolPacket::from_string(mac, ':')?;

	let interval = print_interval(args.verbosity, args.number_of_packets);
	for n in 0..args.number_of_packets {
		let res = wol.send_magic_to(args.source, args.destination)?;

		if ((n + 1) % interval) == 0 || (n + 1) == args.number_of_packets {
			print!("{:>6}: Sent packet ({} bytes)\r", n + 1, res);
		}
	}
	println!();
	Ok(())
}

fn print_interval(verbosity: usize, packet_count: usize) -> usize {
	if packet_count < 100 {
		return std::usize::MAX;
	}
	match verbosity {
		0 => std::usize::MAX,
		1 => 10,
		v => {
			if v % 2 == 0 {
				(packet_count + 1) / v
			} else {
				((packet_count + 1) / v) + 1
			}
		}
	}
}
