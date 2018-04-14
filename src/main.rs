fn main() {
    let mut args = std::env::args();
    match args.nth(1)
        .unwrap_or_else(|| String::from("missing argument"))
        .as_ref()
    {
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
            ).unwrap();
            println!("{}", decoded) // ocr
        }
        s => {
            eprintln!("unknown task '{}'", s);
            std::process::exit(1)
        }
    }
}
