mod arquivo;

use arquivo::{criar, ler, ler_diretorio, obter_caminho_usuario};

fn main() {
    let caminho = obter_caminho_usuario().unwrap();

    criar(&caminho, &"mangatrix.txt");

    let mut caminho_completo = caminho.clone();

    caminho_completo.push_str("mangatrix.txt");

    match ler_diretorio(&caminho) {
        Ok(_) => {}
        Err(e) => println!("Erro:{}", e),
    }

    ler(&caminho_completo);
}
