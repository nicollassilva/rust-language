fn main() {
    basico_referencia();

    println!("---");

    modificando_referencia();

    println!("---");

    entendendo_referencias_mutaveis();

    println!("---");

    entendendo_referencias_mutaveis_2();

    println!("---");

    referencia_solta();
}

fn basico_referencia() {
    let s1 = String::from("Texto");

    let len = calcula_tamanho(&s1);

    println!("O tamanho de '{}' é: {}", s1, len);
}

fn calcula_tamanho(texto: &String) -> usize { // texto é uma referência para uma String
    texto.len()
} // Aqui, texto sai de escopo. Mas como ela não possui o valor a que se refere,
// nada acontece.

fn modificando_referencia() {
    let mut s = String::from("Texto"); // dados não literais são imutáveis por padrão, utilize mut para deixa-las modificáveis

    println!("Antes de modificar: {}", s);

    modifica(&mut s); // você precisa atribuir `mut` junto ao caractere de referência para sinalizar que a variável é modificável

    println!("Depois de modificar: {}", s);
}

fn modifica(uma_string: &mut String) { // o tipo também muda, como na chamada da função modifica() acima, você precisa passar `mut` junto ao caractere de referência
    uma_string.push_str(" longo");
}

fn entendendo_referencias_mutaveis() {
    // Só podemos ter uma referencia mutável para um determinado dado em um determinado escopo.
    let mut s = String::from("Referencia Mutável");

    // let s1 = &mut s;
    // let s2 = &mut s;

    // println!("Primeira referencia mutável: {}", s1);
    // println!("Segunda referencia mutável: {}", s2);

    // O código comentado acima faz o Rust entrar em pânico: `second mutable borrow occurs here`

    // O código abaixo deve funcionar:

    {
        let s1 = &mut s;
        println!("Primeira referencia mutável: {}", s1);
    } // Aqui a referencia mutável será destruida pelo Drop, então podemos criar outra referencia mutável novamente.

    let s2 = &mut s;
    println!("Segunda referencia mutável: {}", s2);

}

fn entendendo_referencias_mutaveis_2() {
    // O código abaixo não vai funcionar:

    // let mut s = String::from("Referência mutável");
    //
    // let _s1 = &s;
    // let s2 = &s;
    //
    // let s3 = &mut s;
    //
    // println!("O texto de s3 é: {}", s3);
    //
    // s3.push_str(". Alterando!");
    //
    // println!("O texto de s2 é: {}", s2);
}

fn referencia_solta() {
    let referencia_solta = gerando_uma_referencia_solta();
}

fn gerando_uma_referencia_solta() -> &String { // retorna uma referência a uma String
    let referencia_solta = String::from("Referencia solta"); // a String é criada

    &referencia_solta // retornamos a referência de String
} // Aqui, String sai do escopo e é destruido. Perigo! A solução é retornar a string direto (sem o caracter &)