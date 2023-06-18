//1) Faça um programa que receba dois valores inteiros como parâmetro e imprima o maior dos dois valores recebidos.
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

    println!("--- Maior numero ---");

    if numero1 > numero2{
        println!("{}", numero1);
    } else if numero2 > numero1{
        println!("{}", numero2);
    } else {
        println!("Os números são iguais");
    }
}
