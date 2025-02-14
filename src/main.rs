use std::io::{self, Write};

fn main() {
    println!("Program Kalkulator Sederhana");
    println!("---------------------------");

    // Input bilangan pertama
    print!("Masukkan bilangan pertama: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let bilangan1: i32 = input.trim().parse().unwrap();

    // Input bilangan kedua
    print!("Masukkan bilangan kedua: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let bilangan2: i32 = input.trim().parse().unwrap();

    // Tampilkan menu pilihan
    println!("\nPilih Operasi:");
    println!("1. Penambahan");
    println!("2. Pengurangan");
    println!("3. Perkalian");
    print!("Masukkan pilihan (1-3): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let pilihan: i32 = input.trim().parse().unwrap();

    // Proses menggunakan match (equivalent dengan switch di bahasa lain)
    println!("\nHasil:");
    match pilihan {
        1 => println!("{} + {} = {}", bilangan1, bilangan2, bilangan1 + bilangan2),
        2 => println!("{} - {} = {}", bilangan1, bilangan2, bilangan1 - bilangan2),
        3 => println!("{} × {} = {}", bilangan1, bilangan2, bilangan1 * bilangan2),
        _ => println!("Pilihan tidak valid!"),
    }
}