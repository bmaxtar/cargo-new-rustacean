fn main() {
    // Par défaut, Rust utilise le type i32 pour les entiers.
    let x = 10; // Type i32 par inférence

    // Vous pouvez spécifier explicitement le type.
    // Voici un entier non signé de 32 bits.
    let y: u32 = 100;

    // Et voici un entier signé de 64 bits.
    let z: i64 = -5000;

    println!("La valeur de x (i32) est : {}", x);
    println!("La valeur de y (u32) est : {}", y);
    println!("La valeur de z (i64) est : {}", z);
}