fn main() {
    // Rust permet aussi d'utiliser des littéraux pour différents types d'entiers.
    let decimal = 98_222; // Le `_` est un séparateur pour la lisibilité
    let hex = 0xff; // Hexadécimal
    let octal = 0o77; // Octal
    let binary = 0b1111_0000; // Binaire
    let byte = b'A'; // Byte (u8 uniquement)

    println!("\nAutres littéraux entiers :");
    println!("Décimal avec séparateur : {}", decimal);
    println!("Hexadécimal (0xff) : {}", hex);
    println!("Octal (0o77) : {}", octal);
    println!("Binaire (0b1111_0000) : {}", binary);
    println!("Byte (b'A') : {}", byte);
}