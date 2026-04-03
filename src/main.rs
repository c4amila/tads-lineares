mod exercicios;

use exercicios::{
    ex01::inversao_vec,
    ex02::contador_ocorrencias,
    ex03::remocao_condicional,
    ex04::mescla_ordenada,
    ex05::avaliar_expressao,
    ex06::HistoricoNavegacao,
    ex07::EditorTextoMinimalista,
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
    
    //----
    println!("\nExercício 05:");
    let expressao = "3 4 + 2 *";
    let resultado5 = avaliar_expressao(expressao);

    println!("Expressão: {}", expressao);
    println!("Resultado: {}", resultado5);
    
    //----
    println!("\nExercício 06:");
    let mut nagevacao = HistoricoNavegacao::new();//pagina_atual = pagina inicial
    println!("Página inicial: {}\n", nagevacao.mostrar_pagina());

    nagevacao.nova_pagina("ulife.com".to_string());
    println!("Navegou para: {}", nagevacao.mostrar_pagina());
    // Back: ["pagina inicial"]
    // Atual: "ulife.com"

    nagevacao.voltar();
    println!("Voltou para: {}", nagevacao.mostrar_pagina());
    nagevacao.avancar();
    println!("Avançou para: {}", nagevacao.mostrar_pagina());

    //nova pag de novo
    nagevacao.nova_pagina("linkedin.com".to_string());
    println!("Nova página carregada: {}", nagevacao.mostrar_pagina());
    
    //----
    println!("\nExercício 07:");
    let mut editor = EditorTextoMinimalista::new();
    editor.digitar("A");
    editor.digitar("B");

    println!("Texto atual: {}", editor.mostrar_texto());
    editor.desfazer();
    println!("Após desfazer: {}", editor.mostrar_texto());
    editor.desfazer();
    println!("Após desfazer novamente: {}", editor.mostrar_texto());
    editor.refazer();
    println!("Após refazer: {}", editor.mostrar_texto());
    
    //---

}
