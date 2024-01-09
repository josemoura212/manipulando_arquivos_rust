use std::{
    env,
    fs::{self, metadata, File},
};
// use std::io::Write;

pub fn criar(caminho: &str, nome_arquivo: &str) {
    // File::create(caminho);
    println!("Criando arquivo no caminho: {}{}", caminho, nome_arquivo);

    let caminho_completo = format!(r"{}\rust\{}", caminho, nome_arquivo);

    match File::open(&caminho_completo) {
        Ok(_) => {
            println!("Ja existe um arquivo com esse nome nesta pasta");
            return;
        }
        Err(_) => {}
    }

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

pub fn existe(caminho_completo: &str) -> Result<(), &str> {
    if metadata(caminho_completo).is_ok() {
        Ok(())
    } else {
        Err("Arquivo inexistente")
    }
}

pub fn ler_diretorio(caminho: &str) -> Result<(), std::io::Error> {
    let items = fs::read_dir(caminho)?;

    for item in items {
        let item = item?;

        let item_caminho = item.path();
        if item_caminho.is_dir() {
            println!("Diretorio: {}", item_caminho.display())
        } else {
            println!("Arquivo: {}", item_caminho.display())
        }
    }

    Ok(())
}
