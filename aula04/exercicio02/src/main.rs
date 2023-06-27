//2) Implemente uma função compare-and-swap. Esta função recebe três parâmetros inteiros, antigo, novo e condicao. Se o valor antigo e o valor condição forem iguais, não faz nada. Se forem diferentes, o valor antigo deve ser alterado para o valor novo. Algo do tipo:
//if( antigo != comparacao ) antigo = novo;

use std::io;

fn compareAndSwap(antigo: &mut i32, novo: & i32, condicao: & i32){
    if antigo != condicao {
        *antigo = *novo;
    }
}

fn lendoInteiroTeclado() -> i32{
    println!("\nPor favor, digite um numero:");
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("falha ao ler entrada do teclado");
    
    let num: i32 = str.trim().parse().expect("Valor invalido");
    num
}

fn main() {
    let mut n1 = lendoInteiroTeclado();
    let n2 = lendoInteiroTeclado();
    let n3 = lendoInteiroTeclado();

    println!("\n\nnum1: {}, num2: {}, num3: {}", n1, n2, n3);
    compareAndSwap(&mut n1, & n2, & n3);
    println!("\n--- compareAndSwap ---\nnum1: {}, num2: {}, num3: {}", n1, n2, n3);
}
