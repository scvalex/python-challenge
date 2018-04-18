#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;

use regex::Regex;
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

lazy_static!{
    static ref NOTHING_REGEX: Regex = Regex::new(r"and the next nothing is ([0-9]+)").unwrap();
    static ref NOTHING_REGEX2: Regex = Regex::new(r"Next nothing is ([0-9]+)").unwrap();
}

pub fn solve_task(task: &str) -> Result<(), Box<std::error::Error>> {
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
            let mut file = File::open("ocr.txt")?;
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
            for &k in &order {
                println!(
                    "{} => {}",
                    char::from_u32(u32::from(k)).unwrap(),
                    occurences[&k]
                );
            }
            // equality
        }
        "3" => {
            let mut file = File::open("equality.txt")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let re = Regex::new(r"([a-z][A-Z]{3}[a-z][A-Z]{3}[a-z])")?;
            for cap in re.captures_iter(&contents) {
                println!("{}", &cap[0].chars().nth(4).unwrap())
            }
            // linkedlist.php (instead of .html)
        }
        "4" => {
            find_nothing("8022")?
            // 8022
            // peak.html
        }
        "5" => {
            // import pickle
            // print "\n".join(["".join([ch * num for (ch, num) in l]) for l in pickle.load(open('banner.p', 'r'))])
            // channel
        }
        "6" => find_nothing2("90052")?,
        s => {
            return Err(Box::new(Error {
                message: format!("unknown task '{}'", s),
            }))
        }
    }
    Ok(())
}

fn find_nothing(nothing: &str) -> Result<(), Box<std::error::Error>> {
    let text = reqwest::get(&format!(
        "http://www.pythonchallenge.com/pc/def/linkedlist.php?nothing={}",
        nothing
    ))?.text()?;
    println!("{}", text);
    match NOTHING_REGEX.captures(&text) {
        None => Ok(()),
        Some(captures) => {
            println!("{}", &captures[1]);
            find_nothing(&captures[1])
        }
    }
}

fn find_nothing2(nothing: &str) -> Result<(), Box<std::error::Error>> {
    let mut file = File::open(&format!("channel/{}.txt", nothing))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    match NOTHING_REGEX2.captures(&contents) {
        None => Ok(()),
        Some(captures) => {
            println!("{}", &captures[1]);
            find_nothing2(&captures[1])
        }
    }
}
