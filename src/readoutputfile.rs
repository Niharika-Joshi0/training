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

pub fn read_output_data_and_write(output_file: &Path) -> Result<(), std::io::Error> {
    let input_file = File::open(output_file)?;
    let reader = BufReader::new(input_file);
    let mut output_file = File::create("../files/finalout.txt")?;

    for line in reader.lines() {
        if let Ok(line) = line {
            let fields: Vec<&str> = line.splitn(2, ' ').collect();
            if fields.len() == 2 {
                let size: usize = fields[0].parse().unwrap_or(0);
                let bytearr_str = fields[1].trim();

                if let Ok(bytearr) = serde_json::from_str::<Vec<u8>>(bytearr_str) {
                    if !bytearr.is_empty() {
                        if let Ok(person) = Person::parse_from_bytes(&bytearr) {
                            writeln!(
                                &mut output_file,
                                "{},{},{}",
                                person.lname, person.fname, person.dob
                            )?;
                        } else {
                            println!("Failed to parse Person from byte array");
                        }
                    } else {
                        println!("Empty byte array");
                    }
                } else {
                    println!("Failed to deserialize byte array JSON");
                }
            } else {
                println!("Invalid line format: {:?}", line);
            }
        } else {
            println!("Failed to read line");
        }
    }

    println!("Data written into new output file");
    Ok(())
}