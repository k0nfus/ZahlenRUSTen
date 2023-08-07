use rand::Rng;
use std::io;

fn main() {
    loop {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..=1000);
        let mut tries = 0;

        println!("Neues Spiel gestartet! Raten Sie eine Zahl zwischen 0 und 1000:");

        loop {
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Fehler beim Lesen der Eingabe");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Ungültige Eingabe. Bitte geben Sie eine Zahl ein.");
                    continue;
                }
            };

            tries += 1;

            if guess < random_number {
                println!("Die gesuchte Zahl ist größer.");
            } else if guess > random_number {
                println!("Die gesuchte Zahl ist kleiner.");
            } else {
                println!(
                    "Herzlichen Glückwunsch, die gesuchte Zahl war {}. Sie haben {} Versuche benötigt.",
                    random_number, tries
                );
                break;
            }
        }

        println!("Möchten Sie ein neues Spiel spielen? (y/n)");

        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Fehler beim Lesen der Eingabe");

        if play_again.trim().to_lowercase() != "y" {
            break;
        }
    }
}
