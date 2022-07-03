use lib::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("not enough args");
    }

    if args.len() >= 3 && args[2] == "512" {
        if let Ok(hash_arry) = hash_file::<Sha512>(&args[1]) {
            print!("{}", hex_to_string(&hash_arry));
        }
    } else {
        if let Ok(hash_arry) = hash_file::<Sha256>(&args[1]) {
            print!("{}", hex_to_string(&hash_arry));
        }
    }

    println!("  {}", args[1]);
}
