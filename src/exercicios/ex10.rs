use std::collections::VecDeque;
use rand::Rng;

struct Cliente{
    id_cliente: u32,
    tempo_chegada: u32,
    tempo_atendimento: u32,
}

pub fn simulador_fila_banco(){
    let mut fila: VecDeque<Cliente> = VecDeque::new();
    let mut rng = rand::thread_rng();

    let mut tempo_atual = 0;
    let total_clientes_fila = 10;
    let mut atendidos = 0;
    let mut criados = 0;

    let mut atendimento_atual: Option<Cliente> = None;
    let mut tempo_restante = 0;
    let mut soma_espera = 0;

    while atendidos < total_clientes_fila || !fila.is_empty(){
        //começa a simular a chegada aleatoria
        let chegou = rng.gen_bool(0.4);
        if criados < total_clientes_fila && chegou {
            let cliente = Cliente{
                id_cliente: criados + 1,
                tempo_chegada: tempo_atual,
                tempo_atendimento: rng.gen_range(1..10),
            };

            println!("Cliente {} chegou na fila em {} minutos. O atendimento levará {} minutos.", 
            cliente.id_cliente, cliente.tempo_chegada, cliente.tempo_atendimento);

            fila.push_back(cliente);
            criados += 1;
        }

        if atendimento_atual.is_none() {
            if let Some(proximo) = fila.pop_front() {
                let espera = tempo_atual - proximo.tempo_chegada;
                soma_espera += espera;
                println!("Cliente {} começou a ser atendido. Esperou {} minutos.",
                proximo.id_cliente, espera);

                tempo_restante = proximo.tempo_atendimento;
                atendimento_atual = Some(proximo);
            }
        }

        if let Some(cliente) = &atendimento_atual {
            tempo_restante -= 1;
            if tempo_restante == 0 {
                println!("Cliente {} terminou o atendimento.", cliente.id_cliente);
                atendimento_atual = None;
                atendidos += 1;
            }
        }

        tempo_atual += 1;
    }

    let calculo_media_espera = soma_espera as f64 / atendidos as f64;
    println!("\nTempo médio de espera: {:.2} minutos.", calculo_media_espera);
    println!("Clientes atendidos: {}", atendidos);

}