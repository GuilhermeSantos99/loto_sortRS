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
    let nums_to_generate = 5;
    let mut n = 0;

    std::process::Command::new("clear").status().unwrap();

    print!("\n");

    while n < nums_to_generate {
        let num_sorted = rand::thread_rng().gen_range(1..81);

        print!("{}; ", num_sorted);

        n = n + 1;
    }

    print!("\n");
}

fn sena() {}

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
