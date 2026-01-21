fn main() {
    
    // Valmiit tiedot tapahtumista
    let tapahtumat = [
        (15, 1, 1945, "manhattan project started"),
        (16, 1, 1547, "Ivan the Terrible crowned Tsar of Russia"),
        (17, 1, 1595, "France Declares War on Spain"),
        (18, 1, 1943, "Siege of Leningrad ends"),
        (19, 1, 1978, "Last Volkswagen Beetle made in Germany"),
        (20, 1, 1961, "JFK Inaugurated as 35th U.S. President"),
        (21, 1, 1903, "Houdini escapes from Halvemaansteeg police station in Amsterdam"),
        (22, 1, 1528, "England and France declare war on Holy Roman Emperor Charles V"),
    ];
    let testipaiva = 17;
    let testikuukausi = 1; 
    // Eka osa: Tietyn päivän tapahtumat
    println!("--- TESTATAAN PÄIVÄÄ {}.{}. ---", testipaiva, testikuukausi);

    for tapahtuma in tapahtumat.iter() {
        if tapahtuma.0 == testipaiva && tapahtuma.1 == testikuukausi {
            println!("{}.{}.{}: {}", tapahtuma.0, tapahtuma.1, tapahtuma.2, tapahtuma.3);
        }
    }

    println!(""); 

    // TOinen osa: Viikon tapahtumat
    
println!("--- Week (15.1.-22.1.) ---");
    println!("");

    for paiva in 15..=22 {
        println!("Events of January {}", paiva);

        for tapahtuma in tapahtumat.iter() {
            if tapahtuma.0 == paiva && tapahtuma.1 == 1 {
                println!("  - {} ({})", tapahtuma.3, tapahtuma.2);
            }
        }
        println!(""); 
    }
}