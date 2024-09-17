use std::io;

fn main() {
    // funcao_loop();
    // usando_while();
    // looping_usando_matriz();
    // looping_usando_for();
    // looping_usando_range();
    // conversor_fahrenheit();
    musica_os_doze_dias_de_natal();
}

fn funcao_loop() {
    loop {
        println!("novamente!");
    }
}

fn usando_while() {
    let mut numero = 3;

    while numero != 0 {
        println!("{}!", numero);

        numero = numero - 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_usando_matriz() {
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < 5 {
        println!("O valor é: {}", a[indice]);

        indice = indice + 1;
    }
}

fn looping_usando_for() {
    let a = [10, 20, 30, 40, 50];

    for elemento in a.iter() {
        println!("O valor é: {}", elemento);
    }
}

fn looping_usando_range() {
    let a = [10, 20, 30, 40, 50];

    for numero in (0..5).rev() {
        println!("{}!", a[numero]);
    }

    println!("LIFTOFF!!!");
}

/**
 * Atividades pós-capítulo do Livro
 */

fn conversor_fahrenheit() {
    let mut fahrenheit = String::new();

    println!("Digite os graus em Fahrenheit");

    loop {
        io::stdin().read_line(&mut fahrenheit)
            .expect("Digite os graus em Fahrenheit:");

        let fahrenheit: u32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Digite um número válido!");
                continue;
            }
        };

        println!("Quantos graus Celsius equivalem {} Fahrenheit?", fahrenheit);

        let resposta = (fahrenheit - 32) * 5 / 9;

        println!("Resposta: {}", resposta);
        break;
    }
}

fn musica_os_doze_dias_de_natal() {
    let dias = [
        "primeiro",
        "segundo",
        "terceiro",
        "quarto",
        "quinto",
        "sexto",
        "sétimo",
        "oitavo",
        "nono",
        "décimo",
        "décimo primeiro",
        "décimo segundo"
    ];

    let enviados = [
        "E uma perdiz em uma pereira!",
        "Duas rolas",
        "Três galinhas francesas",
        "Quatro pássaros chamando",
        "Cinco anéis de ouro",
        "Seis gansos-de postura",
        "Sete cisnes uma natação",
        "Oito empregadas domésticas uma ordenha",
        "nove senhoras que dançam",
        "dez senhores a-salto",
        "onze gaiteiros que conduzem",
        "Doze bateristas tambores"
    ];

    let mut index = 1;

    while index <= dias.iter().count() {
        println!("No {} dia de Natal", dias[index - 1]);
        println!("meu verdadeiro amor enviou a mim");

        for indexEnviado in (0..index).rev() {
            println!("{}", enviados[indexEnviado]);
        }

        index = index + 1;

        println!("");
    }
}