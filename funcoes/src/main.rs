fn main() {
    println!("Funções em Rust!");
    println!("---");

    primeira_funcao(50);
    println!("---");
    segunda_funcao(32, 68);
    println!("---");
    println!("O valor de retorno da função 'cinco' é: {}", cinco());
    println!("---");
    println!("A soma do valor 5 mais um valor u8 é: {}", soma(10));
}

fn primeira_funcao(x: u8) {
    println!("Exibindo o valor de x: {}", x);
}

fn segunda_funcao(x: u8, y: u8) {
    println!("Exibindo o valor de x: {}", x);
    println!("Exibindo o valor de y: {}", y);
}

fn cinco() -> u8 {
    5
}

fn soma(y: u8) -> u8 {
    5 + y // Expressões não terminam com ; ! Caso você adicione ; , isso vira uma declaração e não retornará nada.
}