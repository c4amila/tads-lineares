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

8.** Sequências de símbolos:** Verifique se expressões como `"{[()]}"`, `"([)]"`, `"((("` têm
delimitadores balanceados (parênteses, colchetes e chaves).
<br>
**Complexidade:** o loop deste código tem complexidade O(n), pois ele está iterando sobre os símbolos uma única vez. As operações de match são O(1), estará verifiando símbolo por símbolo uma vez. As operações de push e pop também são O(1).

9.**Pilha com mínimo:** Implemente `StackMin` com operação min() em O(1), que retorna o menor
elemento atual da pilha. (Dica: use uma pilha auxiliar.)
<br>
**Complexidade:** as funções de push, pop e min são O(1) pois em push() está fazendo a adição do valor na pilha de minimos, em pop() está removendo a pilha e a pilha minima, e min() faz acesso direto no topo da pilha_min.

## Fila (Queue)
10.**Simulador de fila de banco:** Crie uma simulação onde clientes chegam com intervalos aleatórios e
são atendidos em ordem. Registre o tempo médio de espera.
<br>
**Complexidade:** O código é O(n), mas as operações dentro do loop, faz com que os clientes sejam inseridos e removidos da fila uma vez usando push_back e pop_front, sendo O(1).

11.**Impressora compartilhada:** Simule uma fila de impressão onde cada trabalho tem um nome e
número de páginas. Imprima os trabalhos na ordem de chegada, reportando cada um.
<br>
**Complexidade:** O(n), pois ele pode fazer a inserção de n impressões, as operações de push_back e pop_front para adicionar as impressões no final da fila é O(1).

12.**Buffer de mensagens:** Implemente um buffer de capacidade fixa usando `FilaCircular` que
descarta a mensagem mais antiga quando cheio (comportamento overwrite).
<br>
**Complexidade:** As operações de enfileirar e desenfileirar as mensagens é O(1) pois estão acessando a fila, calculando índice e incrementando uma vez, a função listar é O(n) pois está iterando n vezes sobre a quantidade de mensagens na fila. 


