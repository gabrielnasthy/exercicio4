// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021

use std::io;

fn main() {
    println!("Digite M para masculino ou F para feminino:");

    let mut sexo = String::new();
    io::stdin()
        .read_line(&mut sexo)
        .expect("Falha ao ler a linha");

    sexo = sexo.trim().to_uppercase();

    if sexo == "M" || sexo == "F" {
        println!("Sexo válido");
    } else {
        println!("Sexo inválido");
    }
}