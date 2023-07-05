//1) Faça uma função que receba um valor inteiro que represente a idade de uma pessoa e imprima a sua faixa etária. Bebê: Recém-nascido até 2 anos de idade. 
//Criança: 2 anos até 12 anos de idade. 
//Adolescente: 13 anos até 19 anos de idade. 
//Jovem adulto: 20 anos até 29 anos de idade. 
//Adulto jovem: 30 anos até 39 anos de idade. 
//Adulto de meia-idade: 40 anos até 59 anos de idade. 
//Idoso: 60 anos em diante. 
//Lembre: 13..19 inclui o 13, mas não o 19. 13..=19, inclui tanto o 13 como o 19

enum Opcao {
    Valor(i32),
    Nenhum
}

fn imprime(idade: i32){
    let op1 = Opcao::Valor(idade);

    match op1 {
        Opcao::Valor(x) if x <= 2 => {
            println!("Bebê");
        },

        Opcao::Valor(x) if x <= 12 => {
            println!("Crianca");
        },

        Opcao::Valor(x) if x <= 19 => {
            println!("Adolescente");
        },

        Opcao::Valor(x) if x <= 29 => {
            println!("Jovem adulto");
        },

        Opcao::Valor(x) if x <= 39 => {
            println!("Adulto jovem");
        },

        Opcao::Valor(x) if x <= 59 => {
            println!("Adulto de meia-idade");
        },

        Opcao::Valor(x) if x <= 200 => {
            println!("Idoso");
        },

        Opcao::Nenhum => {
            println!("Opção vazia");
        }
        _ => {
            println!("Opção inválida");
        }
    }
}

fn main() {
    println!("--Faixa Etária--");
    imprime(29);
}

