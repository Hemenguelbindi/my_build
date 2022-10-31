use std::io;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};


fn create_name(path: str){

}


fn main() {
    let mut name = String::new();
    println!("Приветствую в программе учета строения тела.");
    println!("Введите ваше имя.");
    io::stdin().read_line(&mut name).expect("Не получилось получить ввод");
    
}
