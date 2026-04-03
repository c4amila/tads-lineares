# 📚 Estrutura de dados lineares (TADs)
Atividade proposta pelo professor Alexandre Montanha com objetivo de praticar a utilização de estrutura de dados lineares.

## Vec e Operações Básicas

1.**Inversão com Vec:** Dado `vec![1, 2, 3, 4, 5]`, inverta-o usando apenas `push/pop` (sem `.reverse()`).
<br>
**Complexidade:** Neste código, o vetor possui tamanho e valores fixos (5 elementos), sempre vai iterar 5 vezes, sendo então O(1). Mas imaginando um vetor com n elementos (entrada variável), o loop itera n vezes, removendo e adicionando 1 elemento por vez, ou seja, o crescimento é linear O(n).

2.**Contador de ocorrências:** Dado um `Vec<char>` com as letras de uma frase, conte quantas vezes
cada letra aparece usando apenas iteração com `for x in &vec`.
<br>
**Complexidade:** Há 2 loops aninhados, iterando n*n vezes cada elemento daquele vetor, um loop percorre todo o vetor, e o outro percorre fazendo a verificação e contagem das letras repetidas.

3.**Remoção condicional:** Remova todos os números pares de um `Vec<i32>` sem usar `.retain()`.
Analise o custo da sua solução.
<br>
**Complexidade:** Há apenas um loop que itera n vezes os elementos do vetor, sendo O(n), se eu colocar 100 números dentro do vetor, ele percorrerá 100 vezes.

4.**Mescla ordenada:** Dados dois `Vec<i32>` já ordenados, crie um terceiro vetor ordenado com todos
os elementos. Use apenas `extend` e `sort` ou implemente a mescla manualmente.
<br>
**Complexidade:** Considerando que os dois vetores terão sempre o mesmo tamanho, o loop faz uma única iteração verificando posição por posição de ambos os vetores e adiciona no terceiro vetor, então a complexidade O(2n) = O(n). O processo de verificação e inserção dos elementos é feito uma única vez, sendo O(1).

## Pilha (Stack)
5.**Calculadora RPN:** Implemente avaliação de expressões em notação polonesa reversa (ex: "3 4 + 2 *" = 14) usando `Vec<f64>`.
<br>
**Complexidade:** O acesso/inserção dos elementos da pilha é O(1), mas há um loop que está fazendo iterando uma única vez para verificar todos os elementos dentro da expressão e fazer os cálculos, sendo O(n).

6.**Histórico de navegação:** Simule o botão Voltar/Avançar de um navegador com duas pilhas:
`historico_back` e `historico_forward`.
<br>
**Complexidade:** função nova_pagina: O(1), o tempo é constante.
<br>
função voltar() e avançar(): O(1), pois ambas as funções estão utilizando pop() e push(), que é considerado uma operação de tempo constante.

7.**Desfazer/Refazer:** Implemente um editor de texto minimalista que suporte digitar(texto),
desfazer() e refazer() usando pilhas.
<br>
**Complexidade:** função digitar: O(n), consideramos que o texto da string pode ter tamanho n.
<br>
função desfazer() e refazer(): O(1), pois ambas as funções estão fazendo acesso nas pilhas para remover/adicionar elementos, que são de tempo constante.



