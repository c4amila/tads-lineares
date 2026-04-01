pub fn inversao_vec() -> Vec<i32> {
    let mut valores = vec![1, 2, 3, 4, 5];
    let mut invertido = Vec::new(); //novo vetor vazio mutavel
    while let Some(valor) = valores.pop() {//remove valor no final e guarda dentro de valor
        invertido.push(valor); //o que ficou guardado no valor vai pro novo vetor
    }
    invertido
}