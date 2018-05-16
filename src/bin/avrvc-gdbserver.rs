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

use avrvc::models::xmega_au::XmegaA4U::ATxmega128A4U;
use avrvc::models::AvrModel;

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

    let vm = ATxmega128A4U.create_vm();

    // Start the runtime and spin up the server
    let mut runtime = Runtime::new().unwrap();
    gdbserver::serve(vm, &addr, &mut runtime);
    runtime.shutdown_on_idle().wait().unwrap();
}
