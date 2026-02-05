#[derive(Debug)]
enum Category {
    History,
    Birth,
    Death,
    Culture,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Month {
    January,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MonthDay {
    month: Month,
    day: u32, // Estää negatiiviset pv
}

#[derive(Debug)]
struct Date {
    year: i32, // i koska vuodet voivat olla negatiivisia.
    month_day: MonthDay,
}

#[derive(Debug)]
struct Event {
    date: Date,
    description: String,
    category: Category,
}

fn main() {
// Tehdään lista tapahtumista
    let events = vec![
        Event {
            date: Date { year: 1929, month_day: MonthDay { month: Month::January, day: 15 } },
            description: String::from("Martin Luther King Jr. born"),
            category: Category::Birth,
        },
        Event {
            date: Date { year: 2001, month_day: MonthDay { month: Month::January, day: 15 } },
            description: String::from("Wikipedia goes online"),
            category: Category::History,
        },
        Event {
            date: Date { year: 1991, month_day: MonthDay { month: Month::January, day: 16 } },
            description: String::from("The Gulf War begins"),
            category: Category::History,
        },
        Event {
            date: Date { year: 1942, month_day: MonthDay { month: Month::January, day: 17 } },
            description: String::from("Muhammad Ali born"),
            category: Category::Birth,
        },
        Event {
            date: Date { year: 1941, month_day: MonthDay { month: Month::January, day: 18 } },
            description: String::from("David Ruffin born"),
            category: Category::Birth,
        },
        Event {
            date: Date { year: 1966, month_day: MonthDay { month: Month::January, day: 19 } },
            description: String::from("Indira Gandhi elected prime minister of India"),
            category: Category::History,
        },
        Event {
            date: Date { year: 2009, month_day: MonthDay { month: Month::January, day: 20 } },
            description: String::from("Barack Obama inaugurated as U.S. president"),
            category: Category::History,
        },
        Event {
            date: Date { year: 1924, month_day: MonthDay { month: Month::January, day: 21 } },
            description: String::from("Vladimir Lenin dies"),
            category: Category::Death,
        },
        Event {
            date: Date { year: 1901, month_day: MonthDay { month: Month::January, day: 22 } },
            description: String::from("Queen Victoria dies"),
            category: Category::Death,
        },
        Event {
            date: Date { year: 1989, month_day: MonthDay { month: Month::January, day: 23 } },
            description: String::from("Salvador Dalí dies"),
            category: Category::Death,
        },
        Event {
            date: Date { year: 1984, month_day: MonthDay { month: Month::January, day: 24 } },
            description: String::from("Apple Macintosh goes on sale"),
            category: Category::Culture,
        },
        Event {
            date: Date { year: 1924, month_day: MonthDay { month: Month::January, day: 25 } },
            description: String::from("First Winter Olympics open in Chamonix"),
            category: Category::History,
        },
        Event {
            date: Date { year: 2020, month_day: MonthDay { month: Month::January, day: 26 } },
            description: String::from("Kobe Bryant dies in helicopter crash"),
            category: Category::Death,
        },
        Event {
            date: Date { year: 1945, month_day: MonthDay { month: Month::January, day: 27 } },
            description: String::from("Auschwitz concentration camp is liberated"),
            category: Category::History,
        },
        Event {
            date: Date { year: 1986, month_day: MonthDay { month: Month::January, day: 28 } },
            description: String::from("Space Shuttle Challenger explodes"),
            category: Category::History,
        },
        Event {
            date: Date { year: 1886, month_day: MonthDay { month: Month::January, day: 29 } },
            description: String::from("Karl Benz applies for patent for first successful gasoline-driven automobile"),
            category: Category::History,
        },
    ];

    println!("--- Events from January 15 to January 29 ---");

// Käydään läpi päivät 15-29
    for day_num in 15..=29 {
        
        let current_search_day = MonthDay {
            month: Month::January,
            day: day_num,
        };

        println!("\nEvents of January {}", day_num);

        let mut found_events = false;

// Päivään osuvat tapahtumat
        for event in &events {
            if event.date.month_day == current_search_day {
                println!("  {}: {} [{:?}]", event.date.year, event.description, event.category);
                found_events = true;
            }
        }
// Jos ei löytynyt tapahtumia kerrotaan siitä käyttäjälle
        if !found_events {
            println!("  (Tälle päivälle ei löydetty tapahtumia)");
        }
    }
}