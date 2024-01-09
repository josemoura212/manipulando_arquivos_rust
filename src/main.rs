mod arquivo;

use arquivo::{criar, existe, ler, ler_diretorio, obter_caminho_usuario};

fn main() {
    let caminho = obter_caminho_usuario().unwrap();

    let mut caminho_completo = caminho.clone();

    caminho_completo.push_str(r"\mangatrix.txt");

    if existe(&caminho_completo).is_ok() {
        println!("Arquivo ja existe");
        return;
    } else {
        criar(&caminho, &"mangatrix.txt");

        ler(&caminho_completo);

        match ler_diretorio(&caminho) {
            Ok(_) => {}
            Err(e) => println!("Erro:{}", e),
        }
    }
}
