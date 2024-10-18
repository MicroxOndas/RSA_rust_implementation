use num_bigint::BigUint;
use std::io;
use std::str::FromStr;

mod key_generator;
mod encryption;

fn main() {
    loop {
        println!("\n");
        println!("Bienvenid@ a RSA encoder");
        println!("--------------------------------------------");
        println!("  1) Encriptar un mensaje");
        println!("  2) Desencriptar un mensaje");
        println!("  3) Cifrar firma");
        println!("  4) Descifrar firma");
        println!("  5) Generar claves pública y privada");
        println!("  S) Salir");
        println!("--------------------------------------------");
        println!("\n");
        println!("Por favor escoja una opción: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        let input = input.trim();
        
        match input {
            "1" => encrypt_menu(),
            "2" => decrypt_menu(),
            "3" => sign_encrypt_menu(),
            "4" => sign_decrypt_menu(),
            "5" => gen_keys(),
            "S" => break,
            "s" => break,
            _ => println!("Opción no reconocida, por defecto"),
        }
    }
}

fn encrypt_menu(){
    println!("Por favor ingresa tu mensaje: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let message: String = String::from(input.trim());
    println!("Mensaje a encriptar: {message}");

    println!("Por favor ingresa n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let n = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Clave pública n: {n}");

    println!("Por favor ingresa e: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let e = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Clave pública e: {e}");

    let message_encrypted = encryption::encrypt(&message,&n,&e);

    println!("Mensaje encriptado: {message_encrypted}");
}

fn decrypt_menu(){

    println!("Por favor ingresa tu mensaje: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let message_encrypted = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Mensaje a desencriptar: {message_encrypted}");

    println!("Por favor ingresa n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let n = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Clave pública n: {n}");

    println!("Por favor ingresa d: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let d = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Clave privada d: {d}");

    let message_decrypted = encryption::decrypt(&message_encrypted, &n, &d).to_string();

    println!("Mensaje desencriptado: {message_decrypted}");
}

fn sign_encrypt_menu(){
    println!("Por favor ingresa tu firma pública: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let sign: String = String::from(input.trim());
    println!("Firma a encriptar: {sign}");

    println!("Por favor ingresa n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let n = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Clave pública n: {n}");

    println!("Por favor ingresa d: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let d = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Clave privada d: {d}");

    let message_encrypted = encryption::encrypt(&sign,&n,&d);

    println!("Firma cifrada: {message_encrypted}");
}

fn sign_decrypt_menu(){
    println!("Por favor ingresa la firma a descifrar: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let sign = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Firma a descifrar: {sign}");

    println!("Por favor ingresa n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let n = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Clave pública n: {n}");

    println!("Por favor ingresa e: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let e = match BigUint::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La cadena ingresada no es un número válido.");
            return;
        }
    };
    println!("Clave pública e: {e}");

    let sign_decrypted = encryption::decrypt(&sign,&n,&e);

    println!("Firma descifrada: {sign_decrypted}");
}

fn gen_keys() {
    let (n, e, d) = key_generator::generate_rsa_keys();
    println!("Public key n:\n{}\nPublic key e:\n{}\n", n, e);
    println!("Private key d:\n{}", d);
}