//3) Faça um programa que receba dois valores por parâmetro e imprima um retângulo na tela com o caracter * de forma que a base seja mais "comprida" que a altura. Exemplo:
//$> cargo run 6 8
//********
//********
//********
//********
//********
//********
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
      println!("quantidade de parametros incorretos");
      return;
    }

    let numero1: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("erro argumento 1 não é numero");
            return;
        }
    };

    let numero2: i32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("erro argumento 2 não é numero");
            return;
        }
    };

    println!("--- Retângulo ---");

    if numero1 >= numero2{
        quadrado(numero2, numero1);
    } else{
        quadrado(numero1, numero2);
    } 
}

fn quadrado(num1: i32, num2:i32){
    for _linha in 0..num1{
        for _coluna in 0..num2{
            print!("*");
        }
        println!("");
    }
}

