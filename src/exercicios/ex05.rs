pub fn avaliar_expressao(expressao: &str) -> f64 {
    let mut pilha: Vec<f64> = Vec::new();

    for elemento in expressao.split_whitespace() {//para tirar espaço em branco da frase
        //"switch" em rust
        match elemento {
            "+" => {
                let b = pilha.pop().expect("pilha vazia");
                let a = pilha.pop().expect("pilha vazia");
                pilha.push(a + b);
            }
            "-" => {
                let b = pilha.pop().expect("pilha vazia");
                let a = pilha.pop().expect("pilha vazia");
                pilha.push(a - b);
            }
            "*" => {
                let b = pilha.pop().expect("pilha vazia");
                let a = pilha.pop().expect("pilha vazia");
                pilha.push(a * b);
            }
            "/" => {
                let b = pilha.pop().expect("pilha vazia");
                let a = pilha.pop().expect("pilha vazia");
                pilha.push(a / b);
            }
            _ => {
                let numero: f64 = elemento.parse().expect("Elemento inválido");
                pilha.push(numero);
            }
        }
    }

    pilha.pop().expect("Expressão inválida")
}