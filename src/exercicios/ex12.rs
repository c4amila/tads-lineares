pub struct FilaCircular {
    buffer: Vec<Option<String>>,
    capacidade: usize,
    inicio: usize,
    fim: usize,
    tamanho: usize,
}

impl FilaCircular {
    pub fn new(capacidade: usize) -> Self {
        Self {
            buffer: vec![None; capacidade],
            capacidade,
            inicio: 0,
            fim: 0,
            tamanho: 0,
        }
    }

    fn vazio(&self) -> bool {
        self.tamanho == 0
    }

    fn cheio(&self) -> bool {
        self.tamanho == self.capacidade
    }

    pub fn enfileirar_mensagem(&mut self, mensagem: String) {
        if self.cheio() {
            self.buffer[self.fim] = Some(mensagem);
            self.inicio = (self.inicio + 1) % self.capacidade;
            self.fim = (self.fim + 1) % self.capacidade;
        } else {
            self.buffer[self.fim] = Some(mensagem);
            self.fim = (self.fim + 1) % self.capacidade;
            self.tamanho += 1;
        }
    }

    pub fn desenfileirar_mensagem(&mut self) -> Option<String> {
        if self.vazio() {
            None
        } else {
            let mensagem = self.buffer[self.inicio].take();
            self.inicio = (self.inicio + 1) % self.capacidade;
            self.tamanho -= 1;
            mensagem
        }
    }

    pub fn listar_mensagens(&self) -> Vec<String> {
        let mut mensagens = Vec::new();

        for i in 0..self.tamanho {
            let index = (self.inicio + i) % self.capacidade;
            if let Some(ref mensagem) = self.buffer[index] {
                mensagens.push(mensagem.clone());
            }
        }
        mensagens
    }

}