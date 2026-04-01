pub fn mescla_ordenada(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    /*com extend e sort:
    let mut vec3 = vec1;
    vec3.extend(vec2);
    vec3.sort();*/

    let mut vec3 = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < vec1.len() && j < vec2.len(){//pega o tamanho dos dois vetores
        if vec1[i] <= vec2[j]{ //compara as posições dos 2 vetores
            vec3.push(vec1[i]);//adiciona no terceiro vetor
            i += 1;//vai p proximo indice
        }
        else{
            vec3.push(vec2[j]);//coloca no segundo vetor
            j += 1;
        }
    }

    vec3.extend_from_slice(&vec1[i..]);
    vec3.extend_from_slice(&vec2[j..]);

    vec3
}