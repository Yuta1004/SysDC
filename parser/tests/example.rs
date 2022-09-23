use std::fs;

use sysdc_parser::Parser;

#[test]
fn parse_example_box() {
    parse_files(&["../example/box/box.def"]);
}

#[test]
fn parse_example_logger() {
    parse_files(&[
        "../example/logger/logger.def",
        "../example/logger/std.def",
        "../example/logger/std/io.def",
        "../example/logger/std/time.def",
    ])
}

fn parse_files(pathes: &[&str]) {
    let mut parser = Parser::default();
    for path in pathes {
        let s8 = fs::read(path).unwrap();
        let s = String::from_utf8(s8).unwrap();
        parser.parse(path.to_string(), &s).unwrap();
    }
    parser.check().unwrap();
}
