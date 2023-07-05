//struct Ponto { x: i32, y: i32 }
//impl Ponto {
//    fn cria(x: i32, y: i32) -> Ponto { Ponto { x, y } }
//    fn mover(&mut self, dx: i32, dy: i32) { self.x += dx; self.y += dy; }
//}

//4) Refaça o exemplo do ponto usando uma estrutura para Retangulo.
//5) Inclua na estrutura Retangulo um campo área e um método para calcular a área.
//6) Faça uma função que crie retângulos com tamanhos aleatórios (delimite o tamanho dos lados entre 2 e 20).
//7) Implemente um programa que manipule um vetor de retângulos, ordenando-os por tamanho.
//8) Implemente uma função que receba um retangulo e um vetor ordenado de retangulos e insira retângulo recebido na posição correta do vetor (use o método insert).

use rand::Rng;

use rand::Rng;

struct Retangulo {
    pontoPartida: Ponto,
    largura: u32,
    altura: u32,
    area: u32,
}

impl Retangulo {
    fn cria(x: i32, y: i32, largura: u32, altura: u32) -> Retangulo {
        let pontoPartida = Ponto::cria(x, y);
        let area = largura * altura;
        Retangulo {pontoPartida,largura, altura, area}
    }

    fn mover(&mut self, dx: i32, dy: i32) {
        self.pontoPartida.mover(dx, dy);
    }

    fn calcular_area(&mut self) -> u32{
        self.largura * self.altura
    }
}

struct Ponto {
    x: i32,
    y: i32,
}

impl Ponto {
    fn cria(x: i32, y: i32) -> Ponto {
        Ponto { x, y }
    }

    fn mover(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

fn CriaRetangulosAleatorios() -> Retangulo{
    let mut retanguloAleatorio = Retangulo::cria(rand::thread_rng().gen_range(0, 100), rand::thread_rng().gen_range(0, 100), rand::thread_rng().gen_range(2, 20), rand::thread_rng().gen_range(2, 20));
    retanguloAleatorio
}

fn geraVetorRetangulos(size: usize) -> Vec<Retangulo> {
    let mut vet = Vec::with_capacity(size);
    for _ in 0..size {
        vet.push(CriaRetangulosAleatorios());
    }
    vet
}

fn bubble_sort(vetor: & mut Vec<Retangulo>){
    for j in 0..(vetor.len()){
        for i in 0..(vetor.len()) - j - 1 {
           if vetor[i].area > vetor[i+1].area {
               vetor.swap(i, i+1);
           }
        }
    }
}

fn insereRetangulo(vetor: & mut Vec<Retangulo>, retangulo: Retangulo) {
    for i in 0..(vetor.len()){
        if retangulo.area < vetor[i].area {
            if i == 0{
                vetor.insert(0, retangulo);
            } else {
                vetor.insert(i, retangulo);
            }
            return;
        }
    }
    vetor.push(retangulo);
}

fn main() {
    let mut retangulo = CriaRetangulosAleatorios();

    println!("\nArea: {}", retangulo.calcular_area());

    println!("\nAntes do movimento: x = {}, y = {}", retangulo.pontoPartida.x, retangulo.pontoPartida.y);
    retangulo.mover(2, 2);
    println!("Após o movimento: x = {}, y = {}", retangulo.pontoPartida.x, retangulo.pontoPartida.y);


    let mut vetRetangulo = geraVetorRetangulos(3);


    println!("\nVetor antes de ser ordenado: [0]: {},  [1]: {},  [2]: {}", vetRetangulo[0].area, vetRetangulo[1].area, vetRetangulo[2].area);
    bubble_sort(&mut vetRetangulo);
    println!("\nVetor depois de ser ordenado: [0]: {},  [1]: {},  [2]: {}", vetRetangulo[0].area, vetRetangulo[1].area, vetRetangulo[2].area);

    insereRetangulo(&mut vetRetangulo, retangulo);
    println!("\nVetor depois de inserir um novo retangulo: [0]: {},  [1]: {},  [2]: {}, [3]: {}", vetRetangulo[0].area, vetRetangulo[1].area, vetRetangulo[2].area, vetRetangulo[3].area);

}