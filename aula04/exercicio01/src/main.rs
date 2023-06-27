//1) Implemente uma função "swap", que receba dois valores inteiros e os troque.
use std::env;

fn swap(num1: &mut i32, num2: &mut i32){
  let temp = *num1;
  *num1 = *num2;
  *num2 = temp;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("quantidade de parâmetros incorretos");
        return;
    }

    let mut numero1: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("erro argumento 1 não é numero");
            return;
        }
    };

    let mut numero2: i32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("erro argumento 2 não é numero");
            return;
        }
    };
    
    println!("Num1: {}, Num2:{}", numero1, numero2);
    swap(&mut numero1, &mut numero2);
    println!("\n--Swap--\nNum1: {}, Num2:{}", numero1, numero2);
}
