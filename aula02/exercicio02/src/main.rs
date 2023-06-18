//2) Faça um programa que receba um valor por parâmetro e imprima um triangulo na tela com o caracter *.
//$> cargo run 6
//*
//**
//***
//****
//*****
//******
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {

      println!("quantidade de parametros incorretos");
      return;
    }

    let numero: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("erro argumento não é numero");
            return;
        }
    };


    for _linha in 0..numero{
        for _coluna in 0..(_linha + 1){
            print!("*");
        }
        println!("");
    }
}
