use std::collections::VecDeque;

struct Impressao {
    nome: String,
    paginas: u32,
}

pub fn impressora_compartilhada(){
    let mut fila: VecDeque<Impressao> = VecDeque::new();

    //adiciona as impressoes, jogando smp pro final da fila
    fila.push_back(Impressao { nome: String::from("impressao1.pdf"), paginas: 10 });

    fila.push_back(Impressao { nome: String::from("impressao2.pdf"), paginas: 8 });

    fila.push_back(Impressao { nome: String::from("impresssao3.pdf"), paginas: 50 });

    fila.push_back(Impressao { nome: String::from("impressao4.pdf"), paginas: 25 });

    while let Some(impressao) = fila.pop_front(){
        println!("Imprimindo {} | {} paginas", impressao.nome, impressao.paginas);
    }

}