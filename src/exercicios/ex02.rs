pub fn contador_ocorrencias(vec: Vec<char>) -> Vec<(char, usize)>{//retorna a (letra, qtd ocorrencias)
    let mut ocorrencias: Vec<(char, usize)> = Vec::new(); //inicia vetor vazio e mutavel
    for &letra in &vec{//primeira iteração sobre os itens do vetor
        let mut encontrado = false; //apenas para mostrar se a letra ja foi encontrada

        for elemento in &mut ocorrencias{//segunda iteração sobre as ocorrencias, vendo se já existe a letra
            if elemento.0 == letra{ //se o elemento desta posicao for igual a letra,
                elemento.1 += 1; //incrementa
                encontrado = true;               
            }
        }
        if !encontrado{
            ocorrencias.push((letra, 1));//adiciona no vetor
        }
    }
    ocorrencias
}