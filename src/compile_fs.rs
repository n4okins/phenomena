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
    let content = read("src/raw/step01.phenom");

    let mut namespace = HashMap::new();
    for i in 1..10 {
        namespace.insert(format!("0{}", i), i);
    } // テクニック#1
    let mut txt: Vec<char> = content.chars().collect();
    txt.extend(Vec::with_capacity(1024 - txt.len()));
    for pc in 1..txt.len(){ // Stringから1文字ずつ取得
        if txt[pc] == "\n" || txt[pc] == "\r" || txt[pc] == " " || txt[pc] == "\t" || txt[pc] == ";" {
            continue;
        } else if txt[pc + 1] == "=" && txt[pc + 3] == ";" { // 代入
            namespace[txt[pc]] = namespace[txt[pc + 2]];
        } else if txt[pc + 1] == "=" && txt[pc + 3] == "+" && txt[pc + 5] == ";" { // 加算
            namespace[txt[pc]] = namespace[txt[pc + 2]] + namespace[txt[pc + 4]];
        } else if txt[pc + 1] == "=" && txt[pc + 3] == "-" && txt[pc + 5] == ";" { // 減算
            namespace[txt[pc]] = namespace[txt[pc + 2]] - namespace[txt[pc + 4]];
        } else if txt[pc] == "p" && txt[pc + 1] == "r" && txt[pc + 5] == " " { // pr*** X (print x)
            println!("{}", txt[pc + 6]);
        }
    }
}