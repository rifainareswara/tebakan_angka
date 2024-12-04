use std::io;

use rand::Rng;
fn main() {
    println!("Selamat Datang di Game Tebak Angka!");

    let computer = rand::thread_rng().gen_range(1..101);
    // println!("Angka Computer: {}", computer);

    println!("Masukkan angka tebakan anda: ");
    let mut tebakan = String::new();
    io::stdin().read_line(&mut tebakan).expect("Masukan angka tebakan anda");

    let tebakan: u32 = match tebakan.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if tebakan == computer {
        println!("Selamat, tebakan anda benar!");
    } else {
        println!("Maaf, tebakan anda salah.");
    }
    println!("Angka tebakan computer: {}", computer);
}
