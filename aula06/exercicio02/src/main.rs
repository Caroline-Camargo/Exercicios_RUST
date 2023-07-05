//2) Crie uma enumeração chamada Shape que represente diferentes formas geométricas, como círculo, quadrado, triângulo e retângulo. 
//Cada variante deve ter campos associados para armazenar informações relevantes sobre a forma geométrica, como raio, lado ou base/altura. 
//Em seguida, escreva uma função que recebe uma forma geométrica como argumento e calcula e imprime sua área.

enum Shape {
    Circulo(f64),
    Quadrado(f64),
    Triangulo(f64, f64),
    Retangulo(f64, f64)
}

fn calcular_area(shape: Shape){
    match shape {
        Shape::Circulo(raio) => {
            println!("Area Circulo: {}", f64::powf(raio, 2.0)*3.14);
        },
        Shape::Quadrado(lado) => {
            println!("Area quadrado: {}", lado*lado);
        },
        Shape::Triangulo(base, altura) => {
            println!("Area Triangulo: {}", (base*altura)/2.0);
        },
        Shape::Retangulo(base, altura) => {
            println!("Area Retangulo: {}", base*base);

        }

    }
}

fn main() {
    println!("----");

    let mut figura1 = Shape::Triangulo(5.0,2.0);
    calcular_area(figura1);

    figura1 = Shape::Quadrado(10.0);
    calcular_area(figura1);

    figura1 = Shape::Circulo(44.6);
    calcular_area(figura1);

    figura1 = Shape::Retangulo(25.0, 1.0);
    calcular_area(figura1);


}