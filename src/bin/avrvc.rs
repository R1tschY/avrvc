
#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate avrvc;

use docopt::Docopt;

use avrvc::state::AvrState;
use avrvc::core::AvrVm;
use avrvc::core::AvrVmInfo;
use avrvc::decoder::AvrDecoder;
use avrvc::executable::read_executable_file;



const USAGE: &'static str = "
Naval Fate.

Usage:
  avrvc run <name> [--arch=<arch>]

Options:
  -h --help      Show this screen.
  --version      Show version.
  --arch=<arch>  Architecture
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_name: Vec<String>,
    cmd_run: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_run {
        let flash = read_executable_file(&args.arg_name[0]);
        let info = AvrVmInfo {
            pc_bytes: 3, xmega: true, flash_bytes: flash.len(), memory_bytes: 0x70D0 };
        let mut state = AvrState { core: AvrVm::new(&info) };
        let decoder = AvrDecoder::new();

        loop {
            let instr = decoder.decode(&flash, state.core.pc);
            instr.execute(&mut state.core);
        }
    }
}
