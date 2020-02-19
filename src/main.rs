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
    dbg!(&args);
    // I'd make this const but all the functions are ctypes
    // and are not const compatable so I can't declare a IP address
    // in code as const
    args.mac
        .iter()
        .map(|m| send_packets(m, &args))
        .any(|r| r.is_err());
    Ok(())
}

fn send_packets(mac: &str, args: &Opt) -> Result<()> {
    println!(
        "Sending {n} packets to computer({comp}) via {dst}",
        n = args.number_of_packets,
        comp = mac,
        dst = args.destination
    );
    let wol = WolPacket::from_string(mac, ':')?;

    let interval = args.number_of_packets / 1000;
    for n in 0..args.number_of_packets {
        let res = wol.send_magic_to(args.source, args.destination)?;

        if args.number_of_packets <= 5000 || n % interval == 0 || n + 1 == args.number_of_packets {
            print!("{:>6}: Sent packet ({} bytes)\r", n + 1, res);
        }
    }
    println!();
    Ok(())
}
