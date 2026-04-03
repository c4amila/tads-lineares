pub struct EditorTextoMinimalista{
    texto_atual: String,
    desfazer: Vec<String>,
    refazer: Vec<String>,
}

impl EditorTextoMinimalista {
    pub fn new() -> Self {
        Self {
            texto_atual: String::new(),
            desfazer: Vec::new(),
            refazer: Vec::new(),
        }
    }

    pub fn digitar(&mut self, texto: &str) {
        self.desfazer.push(self.texto_atual.clone());//adiciona o estado atual
        self.texto_atual.push_str(texto);//adiciona o texto novo
        self.refazer.clear();//limpa o historico de refazer
    }

    pub fn desfazer(&mut self) {
        if let Some(estado_anterior) = self.desfazer.pop() {
            self.refazer.push(self.texto_atual.clone());
            self.texto_atual = estado_anterior;
        }
    }

    pub fn refazer(&mut self) {
        if let Some(estado_refazer) = self.refazer.pop() {
            self.desfazer.push(self.texto_atual.clone());
            self.texto_atual = estado_refazer;
        }
    }

    pub fn mostrar_texto(&self) -> &str {
        &self.texto_atual
    }
}