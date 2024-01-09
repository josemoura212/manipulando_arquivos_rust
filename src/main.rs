mod arquivo;

use arquivo::{criar, ler_diretorio, obter_caminho_usuario};

fn main() {
    let caminho = obter_caminho_usuario().unwrap();

    criar(&caminho, &"mangatrix.txt");

    match ler_diretorio(&caminho) {
        Ok(_) => {}
        Err(e) => println!("Erro:{}", e),
    }
}
