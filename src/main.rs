use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("kim.fasta").expect("Can't open this file.");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Can't read this file.");

    println!("Size : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    let tokens: Vec<&str> = contents.split("\n").collect();
    let mut buffer = String::new();

    for (index. char) in tokens[1].chars().enumerate() {
        let str1:String = (&char).to_string();

        if index % n == 0 {
            if index != 0 {
                buffer.push_str(&str1);
                buffer.push_str(&"\n");
            }
        } else {
            buffer.push_str(&str1);
        }
    }
    let ent = String::from("\n");
    let convert_string: String = tokens[0].to_owned() + &ent + &buffer;

    let mut convert_f = File::create("output.fasta").expect("Can't create this file");
    convert_f
        .write_all(convert_string.as_bytes())
        .expect("Can't write this string");
        println!("bye");
}