use std::io;

pub fn ler_dados() -> String {
    let mut dados = String::new();
    io::stdin().read_line(&mut dados).expect("Falha ao ler a entrada");
    dados.trim().to_string()
}

pub fn ler_dados_id() -> usize {
    let mut dados = String::new();
    io::stdin().read_line(&mut dados).expect("Falha ao ler a entrada");
    dados.trim().parse().expect("Erro ao converter para inteiro")
}