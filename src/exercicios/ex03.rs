pub fn remocao_condicional(numeros: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::new(); 
    
    for elemento in numeros { //percorre o vetor uma vez
        if elemento % 2 != 0{ //se for impar
            resultado.push(elemento); //adiciona só eles no novo vetor
        }
    }
    resultado
}