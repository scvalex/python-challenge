use std::fs::File;
use std::io::prelude::Read;
use std::collections::HashMap;
use std::char;
use std::fmt;

#[derive(Debug)]
struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

fn main() {
    let mut args = std::env::args();
    match solve_task(
        args.nth(1)
            .unwrap_or_else(|| String::from("missing argument"))
            .as_ref(),
    ) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error: '{}'", err);
            std::process::exit(1)
        }
    }
}

fn solve_task(task: &str) -> Result<(), Box<std::error::Error>> {
    match task {
        "0" => println!("{}", (2 as i64).pow(38)), // 274877906944
        "1" => {
            // let encoded = "g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj. ";
            let encoded = "map";
            let decoded = String::from_utf8(
                encoded
                    .as_bytes()
                    .iter()
                    .map(|&ch| {
                        if (b'a' <= ch) && (ch <= b'z') {
                            ((ch - b'a') + 2) % 26 + b'a'
                        } else {
                            ch
                        }
                    })
                    .collect::<Vec<u8>>(),
            )?;
            println!("{}", decoded) // ocr
        }
        "2" => {
            let mut file = File::open("/home/scvalex/proj/python-challenge/ocr.txt")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let mut occurences = HashMap::new();
            let mut order = vec![];
            contents.as_bytes().iter().for_each(|&ch| {
                let counter = occurences.entry(ch).or_insert(0);
                *counter += 1;
                if *counter == 1 {
                    order.push(ch)
                }
            });
            for &k in order.iter() {
                println!(
                    "{} => {}",
                    char::from_u32(k as u32).unwrap(),
                    occurences.get(&k).unwrap()
                );
            }
            // equality
        }
        s => {
            return Err(Box::new(Error {
                message: format!("unknown task '{}'", s),
            }))
        }
    }
    Ok(())
}
