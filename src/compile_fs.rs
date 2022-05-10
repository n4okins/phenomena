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
        WHITESPACE, NEWLINE
    }
    let mut operator = HashMap::new();
    operator.insert("=", Operator::Equal);
    operator.insert("+", Operator::Add);
    operator.insert("-", Operator::Sub);
    operator.insert(";", Operator::END);
    operator.insert(" ", Operator::WHITESPACE);
    operator.insert("\n", Operator::NEWLINE);
    operator.insert("\r\n", Operator::NEWLINE);
    operator.insert("UNDEFINED", Operator::UNDEFINED);




    let text_size_lim = 128;

    // contentの定義
    let content = read("src/raw/step01.phenom");

    let mut namespace = HashMap::new();

    for i in 1..10 {
        namespace.insert(format!("0{}", i), i);
    } // テクニック#1

    let mut content_chars: Vec<char> = content.chars().collect();
    for _i in 0..(text_size_lim-content_chars.len()){
        content_chars.extend(" ".chars());
    }

    for i in 0..text_size_lim {
        // 1文字ずつStringとして読み込む
        let checking_str = content_chars[i].to_string();

        // もし演算子として登録されていたら、
        if operator.contains_key(&*checking_str) {
            match operator[&*checking_str] {
                Operator::END => {
                    println!("<END>");
                },
                Operator::Add => {
                    println!("+");
                },
                Operator::Sub => {
                    println!("-");
                },
                Operator::Equal => {
                    println!("=");
                },
                Operator::WHITESPACE => {
                },
                Operator::NEWLINE => {},
                Operator::UNDEFINED => {},
            }
        } else {
            // 演算子以外だったら
            print!("{} | bytes: [", content_chars[i]);
            for b in content_chars[i].to_string().bytes(){
                print!("{}, ", b);
            }
            println!("]");
        }
    };
}