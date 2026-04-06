use std::collections::VecDeque;

struct Item<T> {
    valor: T,
    ordem: usize,
    prioridade: i32,
}

pub struct FilaPrioridade<T>{
    itens: VecDeque<Item<T>>,
    contador: usize,
}

impl<T> FilaPrioridade<T> {
    pub fn new() -> Self {
        FilaPrioridade {
            itens: VecDeque::new(),
            contador: 0,
        }
    }

    pub fn enfileirar(&mut self, prioridade: i32, valor: T) {
        self.itens.push_back(Item {
            valor,
            ordem: self.contador,
            prioridade,
        });
        self.contador += 1;
    }

    pub fn desenfileirar(&mut self) -> Option<T> {
        if self.itens.is_empty() {
            return None;
        }
        
        let mut indice_prioridade = 0;
        for i in 1..self.itens.len() {
            if self.itens[i].prioridade > self.itens[indice_prioridade].prioridade ||
               (self.itens[i].prioridade == self.itens[indice_prioridade].prioridade &&
                self.itens[i].ordem < self.itens[indice_prioridade].ordem) {
                indice_prioridade = i;
            }
        }
        self.itens.remove(indice_prioridade).map(|item| item.valor)
    }
}