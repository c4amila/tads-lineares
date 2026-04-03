pub struct StackMin {
    valor_min: Vec<i32>,
    pilha: Vec<i32>,
}
    
impl StackMin {
    pub fn new() -> Self {
        StackMin { valor_min: Vec::new(),pilha: Vec::new() }
    }

    pub fn push(&mut self, valor: i32) {
        self.pilha.push(valor);
        if self.valor_min.is_empty() || valor <= *self.valor_min.last().unwrap() {//se o valor for <= ao minimo atual, poe na valor_min
            self.valor_min.push(valor);
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        let valor = self.pilha.pop()?;
        if let Some(&min) = self.valor_min.last() { //se o minimo atual for igual ao valor que foi removido, remove da valor_min
            if valor == min {
                self.valor_min.pop();
            }
        }
        Some(valor)
    }

    pub fn min(&self) -> Option<i32> {
        self.valor_min.last().cloned()
    }
}