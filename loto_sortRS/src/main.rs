use rand::Rng;
use std::io;

fn show_menu() {
    print!("\n");
    println!("======= lotoSort ========");
    println!("_________________________");
    println!("| - Jogos disponíveis - |");
    println!("|-----------------------|");
    println!("| 1 - Quina             |");
    println!("| 2 - Mega-Sena         |");
    println!("| 3 - LotoMania         |");
    println!("| 4 - LotoFácil         |");
    println!("|- - - - - - - - - - - -|");
    println!("| 5 - Sair              |");
    println!("|-----------------------|");
    println!("Digite o número do tipo de sorteio: ");
}

fn main() {
    show_menu();

    let mut opcao = String::new();

    io::stdin()
        .read_line(&mut opcao)
        .expect("Falha ao ler entrada");

    let opcao: u32 = opcao
        .trim()
        .parse()
        .expect("Por favor, digite apenas números");

    println!("Opção escolhida: {}", opcao);
}
