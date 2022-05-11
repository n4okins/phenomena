use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn read(path: &str) -> String{
    println!("show: @/{}", path);
    let mut f = File::open(path).expect("Failed. File not found.");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("Failed. Something wrong occurred in reading file.");
    // return
    content
}


pub fn step01() {
    enum Operator {
        Equal, Add, Sub, END,
        UNDEFINED,
        WHITESPACE, LINEFEED, CARRIAGERETURN
    }
    let mut operator = HashMap::new();
    operator.insert("=", Operator::Equal);
    operator.insert("+", Operator::Add);
    operator.insert("-", Operator::Sub);
    operator.insert(";", Operator::END);
    operator.insert(" ", Operator::WHITESPACE);
    operator.insert("\n", Operator::LINEFEED);
    operator.insert("\r", Operator::CARRIAGERETURN);
    operator.insert("UNDEFINED", Operator::UNDEFINED);

    // contentの定義
    let content = read("src/raw/step01.phenom");

    let mut namespace = HashMap::new();

    for i in 1..10 {
        namespace.insert(format!("0{}", i), i);
    } // テクニック#1

    for c in content.chars() {
        let checking_string = c.to_string();

        // もし演算子として登録されていたら、
        if operator.contains_key(&*checking_string) {
            match operator[&*checking_string] {
                Operator::END => {
                    println!("<_END_>");
                },
                Operator::Add => {
                    println!("<ADD>");
                },
                Operator::Sub => {
                    println!("<SUB>");
                },
                Operator::Equal => {
                    println!("<EQL>");
                },
                Operator::WHITESPACE => {
                    println!("<SPACE>");
                },
                Operator::LINEFEED => {
                    println!("<LF>");
                },
                Operator::CARRIAGERETURN => {
                    println!("<CR>");
                },
                Operator::UNDEFINED => {},
            }
        } else {
            println!("{}", checking_string);
        }
    };
}