//5) Faça um programa que crie um array com 10 elementos iniciados com valores gerados randomicamente.

fn main() {
    let tamanho = 10;
    let mut vet_aleatorio: Vec<i32> = Vec::new();

    for i in 0..tamanho{
        let mut j: i32;
        j = i;
        vet_aleatorio.push(j);
        print!("{}  ", vet_aleatorio[j]);
    }
}
