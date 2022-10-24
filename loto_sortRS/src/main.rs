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

fn quina() {
    println!("Jogo quina");
}

fn sena() {
    println!("Sena");
}

fn mania() {
    println!("Mania");
}

fn facil() {
    println!("Fácil");
}

fn main() {
    loop {
        show_menu();

        let mut opcao = String::new();

        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler entrada");

        let opcao: i32 = opcao
            .trim()
            .parse()
            .expect("Por favor, digite apenas números");

        println!("Opção escolhida: {}", opcao);

        match opcao {
            1 => quina(),
            2 => sena(),
            3 => mania(),
            4 => facil(),
            5 => break,
            _ => println!("Opção inválida!"),
        }
    }
}
