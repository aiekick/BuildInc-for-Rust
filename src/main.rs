use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Write};

// https://stackoverflow.com/questions/31046763/does-rust-have-anything-like-scanf
// let output = scan!("1000:10",":", i32, i32);
// fn main() {
//     let output = scan!("1000:10",":", i32, i32);
//     println!("{:?}", output); // (Some(1000), Some(10))
// }
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() -> std::io::Result<()>
{
    /*
    the build id is : MajorNumber.MinorNumber.BuildNumber
    if the rule is "1000:10" this correspond to :
    if (BuildNumber > 1000)
        ++MinorNumber;
    if (MinorNumber > 10)
        ++MajorNumber;
    */

    let mut max_build:i32 = 0;
    let mut max_minor:i32 = 0;
    let mut file_name:String = "".to_string();

    // parse args
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        let arr = scan!(arg,":", i32, i32);
        if arr.0 != None && arr.1 != None {
            max_build = arr.0.unwrap();
            max_minor = arr.1.unwrap();
        } else {
            file_name = arg.clone();
        }
    }

    if max_build == 0 || max_minor == 0 || file_name.is_empty() || args.len() != 3
    {
        println!("-------------------- BuildInc --------------------");
        println!("-- this func will increment in a c/c++ include file, 3 vars : MajorNumber, MinorNumber and BuildNumber, according to a rule");
        println!("-- the syntax is : BuildInc rule include_file");
        println!("-- the rule is 'max_build_number:max_minor_number'");
        println!("-- by ex with a rule of 1000:10 the corresponding pseudo code will be :");
        println!("-- if (BuildNumber > 1000) ++MinorNumber;");
        println!("-- if (MinorNumber > 10) ++MajorNumber");
        println!("-- the Build id will be MajorNumber.MinorNumber.BuildNumber");
        println!("--------------------------------------------------");
    }
    else {
        // parse file
        let mut build_number = 0;
        let mut minor_number = 0;
        let mut major_number = 0;

        let file_to_open = File::open(&file_name);
        match file_to_open {
            Ok(file) => {
                let lines = io::BufReader::new(file).lines();
                for one_line in lines {
                    if let Ok(line_str) = one_line {
                        if line_str.contains("BuildNumber") {
                            println!("BuildNumber : ");
                        }
                        else if line_str.contains("MinorNumber") {
                            println!("MinorNumber : ");
                        }
                        else if line_str.contains("MajorNumber") {
                            println!("MajorNumber : ");
                        }
                    }
                }
            }
            _ => println!("Fail to found file {}", &file_name)
        }

        // algo
        build_number += 1;
        if build_number > max_build {
            build_number = 0;
            minor_number += 1;
        }
        if minor_number > max_minor {
            minor_number = 0;
            major_number += 1;
        }

        // write file
        let file_to_write = File::create(&file_name);
        match file_to_write {
            Ok(mut file) => {
                if file.write_all("#pragma once\n".as_ref()).is_err() ||
                    file.write_fmt(format_args!("#define BuildNumber {}\n", build_number)).is_err() ||
                    file.write_fmt(format_args!("#define MinorNumber {}\n", minor_number)).is_err() ||
                    file.write_fmt(format_args!("#define MajorNumber {}\n", major_number)).is_err() ||
                    file.write_fmt(format_args!("#define BuildId {}.{}.{}\n", major_number, minor_number, build_number)).is_err() ||
                    file.sync_all().is_err() {
                    println!("Fail to save file {}", &file_name)
                }
            }
            _ => println!("Fail to create file {}", &file_name)
        }
    }

    Ok(())
}
