fn main() {
    let x = 5;

    println!("O valor de x é: {}", x);

    let x = 6;

    println!("O valor de x mudou, agora é: {}", x);

    // Podem ser de vários tipos. Os métodos para obter os valores são: desestruturizar ou
    // acessando via index - touplas também começam cm índice 0).
    // Exemplo: tupla.0, tupla.1
    // let (a, um, falso) = tupla
    // Caso voce nao use as variaveis de uma tupla, faça-a começar com _ para o compilador entender que é intencional
    let tupla: (char, u8, bool) = ('a', 1, false);

    println!("O valor boolean da tupla é: {}", tupla.2);

    let (_a, um, _falsa) = tupla;

    println!("O valor da variável um é: {}", um);

    // Matriz em Rust é igual um array em Javascript, porém ela possui o tamanho fixo (não aumenta e nem diminui)

    let matriz: [i32; 9] = [2, 3, 4, 6, 7, 4, 3, 1, 2];

    println!("O valor do 4º índice da matriz é: {}", matriz[3]);

    // Acessar valores fora do tamanho da matriz gera pânico em tempo de execução
    // - println!("Esse valor está fora do tamanho da matriz: {}", matriz[15]);
}
