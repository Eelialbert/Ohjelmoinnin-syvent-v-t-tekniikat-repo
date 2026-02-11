use chrono::{Datelike, Local, NaiveDate};
use std::env;

fn main() {
    // Luetaan syntymäpäivä muuttujista
    let synntymapaiva = NaiveDate::parse_from_str(
        &env::var("BIRTHDATE").expect("Error: BIRTHDATE not set."),
        "%Y-%m-%d"
    ).expect("Error: (Use YYYY-MM-DD).");

    // Nykyisen pv haku
    let tanaan = Local::now().date_naive();
    
    // Onko tänään syntymäpäivä?
    if synntymapaiva.month() == tanaan.month() && synntymapaiva.day() == tanaan.day() {
        println!("Happy birthday!");
    }

    // Ikä Pv
    let paivat = tanaan.signed_duration_since(synntymapaiva).num_days();
    
    // Eri tilanteitten tulostus
     match paivat {
        p if p > 0 => {
            print!("You are {} days old.", p);
            if p % 1000 == 0 { 
                print!(" That's a nice, round number!"); 
            }
            println!();
        }
        p if p < 0 => println!("Are you from the future?"),
        _ => println!("Looks like you're new here."),
    }
}