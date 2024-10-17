use core::str;

use num_bigint::BigUint;

fn string_to_biguint_utf8(s: &str) -> BigUint {
    // Convertir la cadena en bytes UTF-8
    let bytes = s.as_bytes();

    // Crear un BigUint a partir de los bytes en forma de big-endian (orden de bytes más significativo primero)
    BigUint::from_bytes_be(bytes)
}

fn biguint_to_utf8(biguint: &BigUint) -> Result<String, &'static str> {
    // Convertimos BigUint a bytes (Big-Endian, los más significativos primero)
    let bytes = biguint.to_bytes_be();
    
    // Intentamos interpretar esos bytes como una cadena UTF-8
    match str::from_utf8(&bytes) {
        Ok(utf8_string) => Ok(utf8_string.to_string()),  // Si es UTF-8 válido
        Err(_) => Err("No es una cadena UTF-8 válida"),  // Si no es UTF-8 válido
    }
}

pub fn encrypt(m: &String ,n: &BigUint, e: &BigUint) -> BigUint{
    let m = string_to_biguint_utf8(&m);
    m.modpow(&e,&n)
}

pub fn decrypt(m: &BigUint, n: &BigUint, d: &BigUint) -> String {
    let ret: BigUint = m.modpow(&d, &n);

    // Intentamos convertir BigUint a UTF-8 y manejar el resultado
    match biguint_to_utf8(&ret) {
        Ok(utf8_str) =>return utf8_str,
        Err(_) => return "Error al decodificar".to_string(),
    }
}
