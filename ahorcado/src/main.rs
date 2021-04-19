use std::io::{stdin,stdout,Write};
/*
fn main() {
    println!("Bienvenido al ahorcado de FIUBA!");

    let mut intentos = 5;
    let palabra = "ahorcado";
    let palabraEscondida = "________";
    let mut letras = Vec::new();

    println!("La palabra hasta el momento es: {}", palabraEscondida);

    let letra = leer_tecla();
    letras.push(letra);

    if palabra.contains("a") {
        println!("Adivinaste las siguientes letras: {}", "a");
        print!("La palabra hasta el momento es: ");

        for c in palabra.chars() { 
            /*if letras.iter().any(|&i| i==String::c) {
                print!("{}", &c);
            } else {
                print!("_");
            }*/
            print!("{}", &c);
        }
        println!();

    } else {
        intentos -= 1;
        println!("Te quedan {} intentos.", intentos);
    }

}

fn leer_tecla() -> String {
    println!("Ingresa una letra: ");

    let mut v = String::new();
    io::stdin()
    .read_line(&mut v)
    .expect("Error leyendo la linea.");

    v
}*/


fn main() {
    println!("Bienvenido al ahorcado de FIUBA!");

    let mut intentos = 0;
    let gano = false;

    let palabra = "hola";

    loop {
        println!();
        println!("Te quedan {} intentos", 5 - intentos);
        
        let letra = leer_tecla();
        
        if palabra.contains(&letra) {
            println!("Adivinaste las siguientes letras: {}", letra);
        } else {
            intentos += 1;
        }
        

        if gano {
            println!("Felicitaciones! Acertaste la palabra!");
        } 

        if intentos == 5 {
            println!(" Perdiste :(");
            break;
        }
    } 
}

fn leer_tecla() -> String {

    let mut letra = String::new();

    print!("Ingresa una letra: ");
    let _=stdout().flush();

    stdin().read_line(&mut letra)
        .expect("No ingreso un caracter valido");

    if let Some('\n') = letra.chars().next_back() {
        letra.pop();
    }
    if let Some('\r') = letra.chars().next_back() {
        letra.pop();
    }

    letra
}


