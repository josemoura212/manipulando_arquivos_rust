use std::io::prelude::*;
use std::{env, fs::File};
// use std::io::Write;

pub fn criar(caminho: &str, nome_arquivo: &str) {
    // File::create(caminho);
    println!("Criando arquivo no caminho: {}{}", caminho, nome_arquivo);

    let caminho_completo = format!(r"{}\rust\{}", caminho, nome_arquivo);

    match File::create(&caminho_completo) {
        Ok(_) => println!(
            "Arquivo criado com sucesso no caminho: {}",
            &caminho_completo
        ),
        Err(e) => println!("Erro ao criar arquivo. erro: {}", e),
    }
}

pub fn obter_caminho_usuario() -> Option<String> {
    if let Some(caminho_home) = env::var_os("OneDrive") {
        Some(caminho_home.into_string().unwrap())
    } else {
        None
    }
}
