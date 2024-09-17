fn main() {
    let numero = 5;

    if numero < 3 {
        println!("A condição era verdadeira");
    } else {
        println!("A condição era falsa");
    }

    let numero = 6;

    if numero % 4 == 0 {
        println!("o número é divisível por 4");
    } else if numero % 3 == 0 {
        println!("o número é divisível por 3");
    } else if numero % 2 == 0 {
        println!("o número é divisível por 2");
    } else {
        println!("número não é divisível por 4, 3 ou 2");
    }

    if_em_declaracao_let()
}

fn if_em_declaracao_let() {
    let condicao = true;
    let x = if condicao {
        5
    } else {
        4
    };

    println!("O valor de x é: {}", x);
}
