use clap::App;
use clap::Arg;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::path::Path;
extern crate clap;
mod proto;
use proto::foo::Person;
use protobuf::Message;
fn main() {
    let m = App::new("Exercise-1")
        .about("about:Assignment")
        .arg(
            Arg::with_name("Arg1")
                .required(true)
                .help("enter input file name(input.txt)"),
        )
        .arg(
            Arg::with_name("Arg2")
                .required(true)
                .help("enter output file name(file-name.txt"),
        )
        .get_matches();
    let output = m.value_of("Arg2").unwrap();
    File::create(output).expect("no such file exists");

    if let Ok(lines) = read_lines(m.value_of("Arg1").unwrap()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(',').collect();

                let mut out_msg = Person::new();
                out_msg.set_last_name(v[0].to_string());
                out_msg.set_firt_name(v[1].to_string());
                out_msg.set_date(v[2].to_string());
                let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(output)
                    .expect("cannot open file");
                writeln!(file, "{:?},{:?}", out_bytes.len(), out_bytes).expect("scd");
            }
        }
    }
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
