use protobuf::{EnumOrUnknown, Message};

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use example::{ Person,PersonList};

use std::path::PathBuf;
use std::fs::{File};
use std::io::{Write};
use std::fmt;

#[derive(Debug)]
pub struct out{
    pub outsize:usize,
    pub bytearr:Vec<u8>
}

impl fmt::Display for out {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?} ", self.outsize, self.bytearr)
    }
}

pub fn read_data(file_path: &PathBuf)->Vec<out>{
    use std::fs::File;
    use std::io::{BufReader, BufRead};



    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    // let mut o1=out::new();

    let mut out_msgnew = PersonList::new();
    // let mut arrays: Vec<Vec<u8>> = Vec::new();

    let mut arrays: Vec<out> = Vec::new();

    


    let mut lines = reader.lines();

        for line in lines {
            if let Ok(line) = line {
                let fields: Vec<&str> = line.split(',').collect();
                if fields.len() >= 3 {
                    let mut out_msg = Person::new();
                    out_msg.lname = fields[0].to_string();
                    out_msg.fname = fields[1].to_string();
                    out_msg.dob = fields[2].to_string();

                    let out_bytes1: Vec<u8> = out_msg.write_to_bytes().unwrap();
                    // println!("Message request in bytes:\nout_bytes {:?}", out_bytes1);
                    let s=out_bytes1.len();

                    out_msgnew.people.push(out_msg);
                    let mut o1=out{
                        outsize:s,
                        bytearr:out_bytes1
                    };
                    arrays.push(o1);

                    
                    // println!("ARRAYS:{:?}",arrays);

                    // Print or process out_msgnew here
                    
                    // // Decode example request
                    // let in_msg = Person::parse_from_bytes(&out_bytes).unwrap();
                }
            }
        }
        // let out_bytes: Vec<u8> = out_msgnew.write_to_bytes().unwrap();
        let size=arrays.len();
        arrays
        
                
    }





pub fn generate_output(data: Vec<out>, output_file: &PathBuf) -> Result<(), std::io::Error> {
    let mut ofile = File::create(output_file)?;
    // println!("GENOUT{:?}",data);
    for item in &data {
        writeln!(
            ofile,
            "{}",
            item 
        )?;
    }
    
    println!("Data written into {:?}", output_file);
    Ok(())
}