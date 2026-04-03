
pub fn verificador_simbolos(expressao: &str) -> bool {
    let mut pilha: Vec<char> = Vec::new();

    for simbolo in expressao.chars() {
        match simbolo {
            '(' | '[' | '{' => pilha.push(simbolo),//empilha as aberturas
            ')' | ']' | '}' => {
                if let Some(topo) = pilha.pop(){//verifica o simbolo do topo da pilha
                    let esperado = match topo{//verifica os fechamentos corretos
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        _ => unreachable!(),
                    };
                    if simbolo != esperado {
                        return false;//se o simbolo de fechamento não corresponder ao esperado
                    }

                }else{
                    return false;//se a pilha estiver vazia e encontrar um fechamento, incorreto
                }
            }
            _ => continue,
        }
    }
    pilha.is_empty()
}