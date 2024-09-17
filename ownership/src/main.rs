fn main() {
    // string_nao_literal();
    exemplo_em_funcoes()
}

fn string_nao_literal() {
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);
}

fn exemplo_deep_clone() {
    // let s1 = String::from("Hello");
    // let s2 = s1;
    //
    // println!("{}", s1);
    //
    // Esse código acima retorna um erro. A variável s1 foi invalidada e movida (MOVE) para a variável s2.
    // Apenas os dados da pilha foram copiados para a variável s2, Rust não faz deep clone dos dados da heap.
    //
    //
    // let s1 = String::from("Hello");
    // let s2 = s1.clone();
    //
    // println!("s1 = {}, s2 = {}", s1, s2);
    //
    // Esse código acima mostra um deep clone (copia os dados da heap), e talvez seja custoso.
}

fn exemplo_em_funcoes() {
    let s = String::from("texto");  // s entra em escopo.

    toma_posse(s);                  // move o valor de s para dentro da função...
    // ... e ele não é mais válido aqui.

    // println!("{}", s); erro: value borrowed here after move

    let x = 5;                      // x entra em escopo.

    faz_uma_copia(x);               // x seria movido para dentro da função,
    // mas i32 é Copy, então está tudo bem em
    // usar x daqui para a frente.

    println!("Usando x após ser movido (possui o trait Copy): {}", x);

} // Aqui, x sai de escopo, e depois s. Mas como o valor de s foi movido, nada
// de especial acontece.

fn toma_posse(uma_string: String) { // uma_string entra em escopo.
    println!("{}", uma_string);
} // Aqui, uma_string sai de escopo, e o metodo `drop` é chamado. A memória que
// guarda seus dados é liberada.

fn faz_uma_copia(um_inteiro: i32) { // um_inteiro entra em escopo.
    println!("{}", um_inteiro);
} // Aqui, um_inteiro sai de escopo. Nada de especial acontece.
