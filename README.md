# 📚 Estrutura de dados lineares (TADs)
Atividade proposta pelo professor Alexandre Montanha com objetivo de praticar a utilização de estrutura de dados lineares.

## Vec e Operações Básicas

1. **Inversão com Vec:** Dado *vec![1, 2, 3, 4, 5]*, inverta-o usando apenas *push/pop* (sem *.reverse()*).
<br>
**Complexidade:** O(n) -> Apesar de ter um vetor com tamanho e valores fixos (5 elementos), ele sempre vai iterar 5 vezes. Mas imaginando um vetor com n elementos (entrada variável), o loop itera n vezes, removendo e adicionando 1 elemento por vez, ou seja, o crescimento é linear.
<br>
<br>
2. **Contador de ocorrências:** Dado um *Vec< char >* com as letras de uma frase, conte quantas vezes
cada letra aparece usando apenas iteração com *for x in &vec*.
<br>
**Complexidade:** O(n²) -> Há 2 loops aninhados, iterando n*n vezes, cada elemento daquele vetor, vai ter outra busca no vetor de ocorrências para verificar e contar quantas vezes aquela letra apareceu.
<br>
<br>
3. **Remoção condicional:** Remova todos os números pares de um *Vec< i32 >* sem usar *.retain()*.
Analise o custo da sua solução.
<br>
**Complexidade:** O(n) -> Há apenas um loop que itera n vezes (elementos do vetor), se eu colocar 100 números dentro do vetor, ele percorrerá 100 vezes.
<br>
<br>
4. **Mescla ordenada:** Dados dois *Vec< i32 >* já ordenados, crie um terceiro vetor ordenado com todos
os elementos. Use apenas *extend* e *sort* ou implemente a mescla manualmente.
<br>
**Complexidade:** O(n) -> Considerando que os dois vetores terão sempre o mesmo tamanho, temos a complexidade O(2n) = O(n). O crescimento é linear.
<br>
<br>




