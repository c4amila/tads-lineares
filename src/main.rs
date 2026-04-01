mod exercicios;

use exercicios::{
    ex01::inversao_vec,
    ex02::contador_ocorrencias,
    ex03::remocao_condicional,
    ex04::mescla_ordenada,
};

fn main() {
    println!("Exercício 01:");
    let resultado = inversao_vec();
    println!("Vetor invertido: {:?}", resultado);
    //----
    println!("\nExercício 02:");
    let resultado2 = contador_ocorrencias(vec!['a', 'f', 'f', 'f', 'c', 'a']);
    println!("Contagem de ocorrências: {:?}", resultado2);
    //----
    println!("\nExercício 03:");
    let resultado3 = remocao_condicional(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("Pares removidos: {:?}", resultado3);
    //----
    println!("\nExercício 04:");
    let resultado4 = mescla_ordenada(vec![1, 3, 5, 7], vec![2, 4, 6, 8]);
    println!("Terceiro vetor ordenado: {:?}", resultado4);
}
