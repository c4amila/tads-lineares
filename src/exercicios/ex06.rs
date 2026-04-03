pub struct HistoricoNavegacao {
    historico_back: Vec<String>,
    historico_forward: Vec<String>,
    pagina_atual: String,
}

impl HistoricoNavegacao {
    pub fn new() -> Self {
        Self {
            historico_back: Vec::new(),
            historico_forward: Vec::new(),
            pagina_atual: String::from("pagina inicial"),//para nao iniciar com string vazia
        }
    }

    pub fn nova_pagina(&mut self, nova: String) {
        self.historico_back.push(self.pagina_atual.clone()); //adiciona a pagina atual em back
        self.historico_forward.clear(); //limpa forward
        self.pagina_atual = nova; //atualiza pag atual
    }

    pub fn voltar(&mut self) {
        if let Some(pagina_anterior) = self.historico_back.pop() {//verifica se tem pagina anterior
            self.historico_forward.push(self.pagina_atual.clone());//adiciona a pagina atual em forward
            self.pagina_atual = pagina_anterior;
        }
    }

    pub fn avancar(&mut self) {
        if let Some(proxima_pagina) = self.historico_forward.pop() {//verifica se tem pagina para avançar
            self.historico_back.push(self.pagina_atual.clone());//adiciona a pagina atual em back
            self.pagina_atual = proxima_pagina;
        }
    }

    pub fn mostrar_pagina(&self) -> &str {
        &self.pagina_atual
    }
}
