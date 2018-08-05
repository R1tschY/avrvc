extern crate avrvc;
extern crate stderrlog;
extern crate futures;
extern crate regex;
#[macro_use] extern crate pretty_assertions;

mod common;

use avrvc::models::xmega_au::XmegaA4U::ATxmega128A4U;
use avrvc::core::CpuSignal;
use common::setup_emulator;
use common::run_emulator;
use common::setup_test;
use futures::stream::Stream;
use futures::Future;
use std::fs;
use common::get_tests_dir;
use std::ffi::OsStr;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use regex::Captures;
use std::path::Path;
use std::char;


static MAX_CYCLES: usize = 1000 * 1000;


fn find_all(input: &str, pat: &str) -> Vec<usize> {
    let mut input: &str = input;
    let mut result: Vec<usize> = vec![];

    while let Some(index) = input.find(pat) {
        input = &input[index+pat.len()..];
        result.push(index);
    }

    result
}

fn unescape_string(input: &str) -> String {
    let re = Regex::new(r##"\\(x\d{1,2}|\d{1,3}|.)"##).unwrap();
    re.replace_all(input, |captures: &Captures| -> String {
        let capture = &captures[1];
        if capture.len() == 1 {
            match capture.chars().next().unwrap() {
                't' => "\t".into(), 'n' => "\n".into(), 'r' => "\r".into(), '\\' => "\\".into(),
                '0' ... '9' => char::from_u32(capture.parse().unwrap()).unwrap().to_string(),
                c @ _ => c.to_string()
            }
        } else if capture.starts_with("x") {
            char::from_u32(u32::from_str_radix(&capture[1..], 16).unwrap()).unwrap().to_string()
        } else {
            char::from_u32(u32::from_str_radix(&capture[1..], 8).unwrap()).unwrap().to_string()
        }
    }).to_string()
}

fn write_file_contents(path: &Path, contents: &mut String) {
    contents.clear();

    let mut file = File::open(&path).expect("Unable to open the source file");
    file.read_to_string(contents).expect("Unable to read the source file");
}

struct UsartTest {
    input: String,
    output: String,
    channel: String
}

fn find_usart_tests(file_contents: &str) -> Vec<UsartTest> {
    let re = Regex::new(r##"// TEST: "([^"]*)" >> USART(\w+) >> "([^"]*)""##).unwrap();

    let captures_list: Vec<_> = re.captures_iter(&file_contents).collect();
    assert!(!captures_list.is_empty(), "source file without test spec");
    assert_eq!(find_all(&file_contents, "// TEST:").len(), captures_list.len(), "source file with corrupt test spec");

    captures_list.iter().map(|captures| UsartTest {
        input: unescape_string(&captures[1]),
        channel: captures[2].into(),
        output: unescape_string(&captures[3])
    }).collect()
}



fn execute_all_tests() {
    let testsdir = get_tests_dir();
    let sourcesdir = testsdir.join("sources");
    let mut contents = String::new();

    for source_path in fs::read_dir(sourcesdir).unwrap() {
        let source_path = source_path.unwrap().path();
        if source_path.extension() == Some(OsStr::new("c")) {
            println!("{:?}", source_path);

            let relative_path = source_path.strip_prefix(&testsdir).unwrap().clone();

            write_file_contents(&source_path, &mut contents);

            for test in find_usart_tests(&contents) {
                execute_test(&relative_path, &test);
            }
        }
    }
}

fn execute_test(relative_path: &Path, test: &UsartTest) {
    let mut emulator = setup_emulator(relative_path, &ATxmega128A4U, &[]);
    let tx = {
        let usart = emulator.usarts.get(&test.channel as &str).unwrap();
        usart.lock().unwrap().push(test.input.as_bytes());
        usart.lock().unwrap().connect_to_tx()
    };

    assert_eq!(run_emulator(&mut emulator, MAX_CYCLES), Some(CpuSignal::Break));
    drop(emulator);

    assert_eq!(
        String::from_utf8_lossy(&tx.collect().wait().unwrap()),
        test.output
    );
}


#[test]
fn run_tests() {
    setup_test();

    execute_all_tests();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape_string() {
        assert_eq!(unescape_string("abc"), "abc");
        assert_eq!(unescape_string("a\nbc"), "a\nbc");
    }
}