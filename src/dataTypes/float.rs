fn main() {
    // Rust utilise le type f64 par défaut pour les nombres à virgule flottante.
    let x = 2.0; // Type f64 par inférence

    // Vous pouvez spécifier explicitement le type.
    let y: f32 = 3.0; // Type f32

    println!("La valeur de x (f64) est : {}", x);
    println!("La valeur de y (f32) est : {}", y);
}