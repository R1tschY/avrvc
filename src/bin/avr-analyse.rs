#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate stderrlog;
extern crate avrvc;
extern crate docopt;
extern crate indexmap;

use docopt::Docopt;
use stderrlog::Timestamp;
use avrvc::models::xmega_au::XmegaA4U::ATxmega128A4U;
use avrvc::executable::read_executable_file;
use std::collections::HashMap;
use avrvc::instruction_set::Instruction;
use avrvc::instruction_set::Instruction::*;
use std::collections::HashSet;
use avrvc::models::AvrModel;
use std::ops::Range;
use indexmap::set::IndexSet;
use std::path::Path;

const USAGE: &'static str = "
Naval Fate.

Usage:
  avrvc-gdbserver <flash> [--arch=<arch>] [-q] [-v...]

Options:
  -h --help      Show this screen.
  --version      Show version.
  --arch=<arch>  Architecture
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_q: bool,
    flag_v: usize,
    arg_flash: String,
    flag_arch: Option<String>,
}

struct Function {
    start: usize,
    length: usize,
    name: String,
}

struct Flag {
    addr: usize,
    name: String,
}

struct Info {
    functions: Vec<Function>,
    comments: HashMap<usize, String>,
    flags: Vec<Flag>,
}

fn create_basic_block(bin: &Vec<Instruction>, addr: usize) -> Option<Range<usize>> {



    None
}

fn create_functions(bin: &Vec<Instruction>, addr: usize) -> Option<Function> {
//    let blocks: Vec<Range<usize>> = Vec::with_capacity(1);
//
//    let mut todo = IndexSet::<usize>::new();
//    let mut done = IndexSet::<usize>::new();
//
//    todo.insert(addr);
//    while !todo.is_empty() {
//        let entry = todo.pop();
//        create_basic_block(bin, addr)
//    }


    None
}

fn analyse_functions(bin: &Vec<Instruction>, info: &mut Info) {
    let entries: HashSet<usize> = bin.iter().filter_map(|instr| {
        if let Call { k: addr } = instr {
            Some(*addr as usize)
        } else {
            None
        }
    }).collect();

    info.functions = entries.iter().filter_map(|&addr| create_functions(bin, addr)).collect();
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

    let mut vm = ATxmega128A4U.create_vm();
    let flash = read_executable_file(Path::new(&args.arg_flash));
    vm.write_flash(0, &flash);


}