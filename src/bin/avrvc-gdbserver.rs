#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate docopt;
extern crate avrvc;
extern crate tokio;
extern crate stderrlog;

use docopt::Docopt;
use avrvc::gdb as gdbserver;
use tokio::runtime::Runtime;
use tokio::prelude::*;

use avrvc::models::xmega_au::XmegaA4U::ATxmega128A4U;
use avrvc::models::AvrModel;
use stderrlog::Timestamp;
use avrvc::executable::read_executable_file;

const USAGE: &'static str = "
Naval Fate.

Usage:
  avrvc-gdbserver <flash> [--addr=<addr>] [--arch=<arch>] [-q] [-v...]

Options:
  -h --help      Show this screen.
  --version      Show version.
  --addr=<addr>  IP4-Address to bind gdbserver to
  --arch=<arch>  Architecture
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_q: bool,
    flag_v: usize,
    arg_flash: String,
    flag_addr: Option<String>,
    flag_arch: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    stderrlog::new()
        .quiet(args.flag_q)
        .verbosity(if args.flag_v == 0 { 3 } else { args.flag_v })
        .timestamp(Timestamp::Off)
        .init()
        .unwrap();

    let addr = args.flag_addr.unwrap_or(String::from("127.0.0.1:2159")).parse().unwrap();
    info!("Listening to {} ...", addr);

    println!();
    println!("Connect GDB with:");
    println!("        avr-gdb -ex 'target remote {}' {}", addr, &args.arg_flash);
    println!();

    let mut vm = ATxmega128A4U.create_vm();
    let flash = read_executable_file(&args.arg_flash);
    vm.write_flash(0, &flash);

    // Start the runtime and spin up the server
    let mut runtime = Runtime::new().unwrap();
    gdbserver::serve(vm, &addr, &mut runtime);
    runtime.shutdown_on_idle().wait().unwrap();
}
