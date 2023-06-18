//5) Faça um programa que crie um array com 10 elementos iniciados com valores gerados randomicamente.
use rand::Rng;

fn main() {
    let tam_max = 10;
    let mut vet: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    
    for _i in 0..tam_max{
        vet.push(0);
        let num: i32 = rng.gen_range(0..999);
        vet[_i] = num
    }
    println!("{:?}", vet);
}
