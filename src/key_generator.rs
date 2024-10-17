extern crate rand;
extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, BigUint, RandBigInt, Sign};
use num_traits::{One, Zero};

// Función para generar un número aleatorio de bits usando BigUint
fn generate_random_bit_number(size: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    rng.gen_biguint(size)
}

// Función para hacer la exponenciación modular: (base^exp) % modulus
fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    base.modpow(exp, modulus)
}

// Test de primalidad de Miller-Rabin para BigUint
fn miller_rabin_test(n: &BigUint, k: u32) -> bool {
    if n <= &BigUint::one() || n == &BigUint::from(4_u32) {
        return false;
    }
    if n <= &BigUint::from(3_u32) {
        return true;
    }

    let one = BigUint::one();
    let mut d = n - &one;
    let mut t = 0;

    // Encuentra d tal que d * 2^r = n - 1
    while &d % 2_u32 == BigUint::zero() {
        d >>= 1;
        t += 1;
    }

    'witness_loop: for _ in 0..k {
        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint_range(&BigUint::from(2_u32), &(n - &one)); // Elige a en [2, n-2]
        let mut x = mod_exp(&a, &d, &n);

        if x == one || x == n - &one {
            continue;
        }

        for _ in 0..t - 1 {
            x = mod_exp(&x, &BigUint::from(2_u32), &n);
            if x == n - &one {
                continue 'witness_loop;
            }
        }
        return false;
    }
    true
}

// Función para generar un número primo de n bits utilizando Miller-Rabin
fn generate_large_prime(size: u64) -> BigUint {
    loop {
        let candidate = generate_random_bit_number(size);
        if miller_rabin_test(&candidate, 100) { // 100 iteraciones de Miller-Rabin
            return candidate;
        }
    }
}

// Función auxiliar para calcular el inverso modular usando el algoritmo extendido de Euclides
fn mod_inverse(a: &BigUint, m: &BigUint) -> BigUint {
    let a: BigInt = BigInt::from_biguint(Sign::Plus, a.clone());
    let m: BigInt = BigInt::from_biguint(Sign::Plus, m.clone());
    let m0: BigInt = m.clone(); // Guardamos el valor original de m
    let mut y = BigInt::zero(); // Inicializa y
    let mut x = BigInt::one(); // Inicializa x

    let mut a = a.clone(); // Crea una copia de a
    let mut m = m.clone(); // Crea una copia de m

    if m == BigInt::one() {
        return BigUint::zero(); // El inverso de 0 mod 1 es 0
    }

    while a > BigInt::one() {
        let q = &a / &m; // Calcula la parte entera de a/m
        //      println!("q: {}",q);
        let t = m.clone(); // Almacena el valor actual de m
        //      println!("t: {}",t);
        m = &a % &m; // Calcula el nuevo m
        //      println!("m: {}",m);
        a = t; // Actualiza a al valor antiguo de m
        //      println!("a: {}",a);

        let t = y.clone(); // Almacena el valor actual de y
        //      println!("t: {}",t);
        // Calcula el nuevo y asegurando que no sea negativo
        y = (&x - &q * &y) % &m0; // Modulo con m0 para evitar negativos
        //      println!("y: {y} = ({x} - {q} * {y}) % {m0}");

        // Si y es negativo, ajusta para que esté dentro del rango
        if &y < &BigInt::zero() {
            y += &m0; // Asegúrate de que y sea positivo
            //      println!("y: {}",y);
        }

        x = t; // Actualiza x al antiguo valor de y
        //      println!("x: {}",x);
    }

    if x < BigInt::zero() {
        x += m0;
    }

    x.to_biguint().unwrap() // Devolvemos el inverso modular
}




// Generación de claves RSA (n, e, d)
pub fn generate_rsa_keys() -> (BigUint, BigUint, BigUint) {
    let p = generate_large_prime(1024);
    let q = generate_large_prime(1024);
    //      println!("Generated p:{} \nand\n q:{}",p,q);
    let n = &p * &q;
    //      println!("n:{} ",n);
    let phi_n = (&p - BigUint::one()) * (&q - BigUint::one());
    //      println!("phi:{} ",phi_n);
    // Elegimos e = 65537 por razones de eficiencia y seguridad
    let e = generate_large_prime(1025);
    //      println!("e:{} ",e);
    // Aseguramos que e es coprimo con phi(n)
    //      println!("Y");
    let d = mod_inverse(&e, &phi_n); // Calculamos d, el inverso modular de e mod phi_n

    (n, e, d) // Devolvemos las claves públicas y privadas
}
