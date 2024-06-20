use protobuf::{EnumOrUnknown, Message};

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use example::{ Person,PersonList};

use clap::{App, Arg};
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufRead, BufReader,Write};
use std::fmt;
use serde_json;
mod readinputfile;
mod readoutputfile;





fn main() {
    let matches = App::new("Data Processing Tool")
                    .arg(Arg::with_name("input-file-path")
                    .short('i')
                        .long("input-file-path")
                        .value_name("FILE")
                        .help("input file tp be read")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("output-file-path")
                        .short('o')
                        .long("output-file-path")
                        .value_name("FILE")
                        .help("Output file to write into")
                        .takes_value(true)
                        .required(true))
                    .get_matches();

                let input_file = PathBuf::from(matches.value_of("input-file-path").unwrap());
                let output_file = PathBuf::from(matches.value_of("output-file-path").unwrap());

                let _data=readinputfile::read_data(&input_file);
                // println!("DATA: {:?}",_data);
                let _r=readinputfile::generate_output(_data,&output_file);
                let _o=readoutputfile::read_output_data_and_write(&output_file);
                // println!("{:?}",o);

}





