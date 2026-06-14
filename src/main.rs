//use std::error::Error;
use std::{collections::HashMap, fs};
//symbol, code

fn main() {
    let asmcode = fs::read_to_string("Pong.asm").unwrap();
    let aa = asmcode.lines();
    let asmcode: Vec<&str> = aa
        .map(|b| b.trim())
        .filter(|b| !b.starts_with("//") || *b != "")
        .collect();
    let mut symbols = HashMap::<&str, usize>::new();
    let mut linenumber = 0;
    let mut vnumber = 16;
    symbols.insert("R0", 0);
    symbols.insert("R1", 1);
    symbols.insert("R2", 2);
    symbols.insert("R3", 3);
    symbols.insert("R4", 4);
    symbols.insert("R5", 5);
    symbols.insert("R6", 6);
    symbols.insert("R7", 7);
    symbols.insert("R8", 8);
    symbols.insert("R9", 9);
    symbols.insert("R10", 10);
    symbols.insert("R11", 11);
    symbols.insert("R12", 12);
    symbols.insert("R13", 13);
    symbols.insert("R14", 14);
    symbols.insert("R15", 15);
    symbols.insert("SCREEN", 16384);
    symbols.insert("KBD", 24576);
    symbols.insert("SP", 0);
    symbols.insert("LCL", 1);
    symbols.insert("ARG", 2);
    symbols.insert("THIS", 3);
    symbols.insert("THAT", 4);

    for line in &asmcode {
        if line.starts_with("(") {
            let aa = line.strip_prefix("(").unwrap();
            let aa = aa.strip_suffix(")").expect("Error");

            symbols.insert(aa, linenumber);
        } else if line.starts_with("//") {
        } else if *line == "" {
        } else {
            linenumber += 1;
        }
    }
    for line in &asmcode {
        if line.starts_with("@") {
            let aa = line.strip_prefix("@").unwrap();

            if !symbols.contains_key(aa) {
                let isnum = aa.parse::<usize>().is_err();
                if isnum {
                    println!("{},{}", aa, vnumber);
                    symbols.insert(aa, vnumber);
                    vnumber += 1;
                }
            }
        }
    }
    println!("{:#?}", symbols);
    let mut hackcode = "".to_string();
    for line in &asmcode {
        if line.starts_with("@") {
            let aa = line.strip_prefix("@").unwrap();
            let dest;
            if let Ok(aa) = aa.parse::<usize>() {
                dest = format!("{:015b}", aa);
            } else {
                //println!("{}", line);
                dest = format!("{:015b}", symbols.get(aa).unwrap());
            }
            let code = format!("0{}", dest);
            hackcode = format!("{}{}\n", hackcode, code);
        } else if line.starts_with("(") {
        } else if line.starts_with("//") {
        } else if *line == "" {
        } else {
            let mut code = "111".to_string();
            let mut dest ;//= "".to_string();
            let mut jump = "";
            let mut comp = "";
            if line.contains("=") {
                let mut aa: Vec<&str> = line.split("=").collect();
                if aa[1].contains(";") {
                    let tt: Vec<&str> = aa[1].split(";").collect();
                    aa[1] = tt[0];
                }

                if aa[0].contains("A") {
                    dest = "1".to_string();
                } else {
                    dest = "0".to_string()
                }
                if aa[0].contains("D") {
                    dest = format!("{}1", dest);
                } else {
                    dest = format!("{}0", dest);
                }
                if aa[0].contains("M") {
                    dest = format!("{}1", dest);
                } else {
                    dest = format!("{}0", dest);
                }
            } else {
                dest = format!("000");
            }
            let mut a: Vec<&str> = vec![];
            if line.contains("=") {
                a = line.split("=").collect();
                if a[1].contains(";") {
                    let tt: Vec<&str> = a[1].split(";").collect();
                    a[1] = tt[0];
                }
            } else if line.contains(";") {
                a = line.split(";").collect();
                let temp = a[0];
                a[0] = a[1];
                a[1] = temp;
            }
            if a[1] == "0" {
                comp = "0101010"
            } else if a[1] == "1" {
                comp = "0111111";
            } else if a[1] == "-1" {
                comp = "0111010";
            } else if a[1] == "D" {
                comp = "0001100";
            } else if a[1] == "A" {
                comp = "0110000";
            } else if a[1] == "M" {
                comp = "1110000";
            } else if a[1] == "!D" {
                comp = "0001101";
            } else if a[1] == "!A" {
                comp = "0110001";
            } else if a[1] == "!M" {
                comp = "1110001";
            } else if a[1] == "-D" {
                comp = "0001111";
            } else if a[1] == "-A" {
                comp = "0110011";
            } else if a[1] == "-M" {
                comp = "1110011";
            } else if a[1] == "D+1" {
                comp = "0011111";
            } else if a[1] == "A+1" {
                comp = "0110111";
            } else if a[1] == "M+1" {
                comp = "1110111";
            } else if a[1] == "D-1" {
                comp = "0001110";
            } else if a[1] == "A-1" {
                comp = "0110010";
            } else if a[1] == "M-1" {
                comp = "1110010";
            } else if a[1] == "D+A" {
                comp = "0000010";
            } else if a[1] == "D+M" {
                comp = "1000010";
            } else if a[1] == "D-A" {
                comp = "0010011";
            } else if a[1] == "D-M" {
                comp = "1010011";
            } else if a[1] == "A-D" {
                comp = "0000111";
            } else if a[1] == "M-D" {
                comp = "1000111";
            } else if a[1] == "D&A" {
                comp = "0000000";
            } else if a[1] == "D&M" {
                comp = "1000000";
            } else if a[1] == "D|A" {
                comp = "0010101";
            } else if a[1] == "D|M" {
                comp = "1010101";
            }

            if line.contains(";") {
                let aa: Vec<&str> = line.split(";").collect();
                if aa[1] == "JGT" {
                    jump = "001";
                }
                if aa[1] == "JEQ" {
                    jump = "010";
                }
                if aa[1] == "JGE" {
                    jump = "011";
                }
                if aa[1] == "JLT" {
                    jump = "100";
                }
                if aa[1] == "JNE" {
                    jump = "101";
                }
                if aa[1] == "JLE" {
                    jump = "110";
                }
                if aa[1] == "JMP" {
                    jump = "111";
                }
            } else {
                jump = "000";
            }
            code = format!("{}{}{}{}", code, comp, dest, jump);
            hackcode = format!("{}{}\n", hackcode, code);
        }
    }
    let _ = fs::write("result.hack", hackcode.trim());
}
