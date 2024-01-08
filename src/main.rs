mod arquivo;

use arquivo::{criar, obter_caminho_usuario};

fn main() {
    let caminho = obter_caminho_usuario().unwrap();

    criar(&caminho, &"mangatrix.txt");
}
