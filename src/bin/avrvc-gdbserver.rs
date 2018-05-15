#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate avrvc;
extern crate tokio;

use docopt::Docopt;
use avrvc::gdb as gdbserver;
use tokio::runtime::Runtime;
use tokio::prelude::*;
use avrvc::core::AvrVmInfo;

const USAGE: &'static str = "
Naval Fate.

Usage:
  avrvc-gdbserver <addr> [--arch=<arch>]

Options:
  -h --help      Show this screen.
  --version      Show version.
  --arch=<arch>  Architecture
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_addr: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let addr = args.arg_addr[0].parse().unwrap();
    println!("Listening to {} ...", addr);

    let info = AvrVmInfo { pc_bytes: 3, xmega: true, flash_bytes: 100, memory_bytes: 200 };

    // Start the runtime and spin up the server
    let mut runtime = Runtime::new().unwrap();
    gdbserver::serve(&info, &addr, &mut runtime);
    runtime.shutdown_on_idle().wait().unwrap();
}
