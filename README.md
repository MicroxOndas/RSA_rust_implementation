
# RSA Toolkit en Rust

Este proyecto es una implementación en Rust del algoritmo RSA, que incluye la generación de claves pública y privada, así como herramientas para la encriptación, desencriptación y firma de mensajes. El objetivo es proporcionar una forma segura de comunicación mediante criptografía asimétrica.

## Características

- **Generación de claves RSA**: Crea claves públicas y privadas.
- **Encriptación**: Permite encriptar mensajes utilizando una clave pública.
- **Desencriptación**: Desencripta mensajes cifrados con la clave privada correspondiente.
- **Firmas digitales**: Genera firmas digitales utilizando la clave privada.
- **Verificación de firmas**: Verifica la autenticidad de una firma utilizando la clave pública.

## Requisitos

- Rust (versión 1.56 o superior)

Para instalar Rust, sigue las instrucciones de la [documentación oficial](https://www.rust-lang.org/tools/install).

## Instalación

1. Clona este repositorio:

   ```bash
   git clone https://github.com/tu_usuario/rsa-rust.git](https://github.com/MicroxOndas/RSA_rust_implementation)
   cd RSA_rust_implementation
   ```

2. Construye el proyecto:

   ```bash
   cargo build --release
   ```

## Uso

### 1. Generación de claves

El programa permite generar un par de claves (pública y privada).

```rust
mod key_generator;

let (n, e, d) = key_generator::generate_rsa_keys();
```

### 2. Encriptación

Puedes encriptar un mensaje utilizando la clave pública. Esto es útil cuando deseas enviar un mensaje que solo el destinatario (que posee la clave privada) puede desencriptar.

```rust
mod encryption;

let message_encrypted = encryption::encrypt(&message,&n,&e);
```

### 3. Desencriptación

El destinatario del mensaje puede desencriptarlo utilizando su clave privada.

```rust
mod encryption;

let message_decrypted = encryption::decrypt(&message_encrypted, &n, &d).to_string();
```

### 4. Firma de mensajes

Se puede generar una firma digital para un mensaje utilizando la clave privada, lo que garantiza la autenticidad del mensaje.

```rust
mod encryption;

let sign_encrypted = encryption::encrypt(&sign,&n,&d);
```

### 5. Verificación de firmas

El receptor puede verificar si una firma es válida usando la clave pública del emisor.

```rust
mod encryption;

let sign_decrypted = encryption::decrypt(&sign,&n,&e);
```

## Main

Al ejecutar  `main.rs` se ejecutará un pequeño menú command-line based.

Puedes ejecutar el proyecto mediante:

```bash
cargo run
```

## Dependencias

Este proyecto utiliza las siguientes dependencias:

```toml
[dependencies]
num-bigint = { version = "0.4", features = ["rand"] }
num-traits = "0.2"
rand = "0.8"
```

## Contribución

¡Las contribuciones son bienvenidas! Si tienes sugerencias o encuentras algún bug, por favor abre un issue o envía un pull request.

1. Haz un fork del repositorio.
2. Crea una nueva rama (`git checkout -b feature/nueva-feature`).
3. Realiza tus cambios.
4. Haz un commit (`git commit -am 'Añadir nueva funcionalidad'`).
5. Sube tu rama (`git push origin feature/nueva-feature`).
6. Crea un pull request.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT. Para más detalles, consulta el archivo [LICENSE](./LICENSE).
