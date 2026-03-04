use chrono::NaiveDate;
use crate::{Event, Category, EventProvider};
pub struct TestProvider;

impl EventProvider for TestProvider {
    fn name(&self) -> String {
        String::from("Test Provider")
    }

    fn get_events(&self, events: &mut Vec<Event>) {
    let event1 = Event::new_singular(
            NaiveDate::from_ymd_opt(1917, 12, 6).expect("Virheellinen päivämäärä"),
            String::from("Suomen itsenäistyminen"),
            Category::from_string("History/Finland"), 
        );
    let event2 = Event::new_singular(
            NaiveDate::from_ymd_opt(1969, 7, 20).expect("Virheellinen päivämäärä"),
            String::from("Ensimmäinen ihminen kuussa"),
            Category::primary_only("Science"),
        );
        events.push(event1);
        events.push(event2);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_returns_events() {
        let provider = TestProvider;
        let mut events: Vec<Event> = Vec::new();
        
        provider.get_events(&mut events);
        assert!(events.len() > 0, "Tuottajan pitäisi palauttaa tapahtumia");
        assert_eq!(events.len(), 2, "Tapahtumien määrä ei täsmää");
    }
    #[test]
    fn test_first_event_has_correct_data() {
        let provider = TestProvider;
        let mut events: Vec<Event> = Vec::new();
        
        provider.get_events(&mut events);
        let first_event = &events[0];
        
        assert_eq!(first_event.description, "Suomen itsenäistyminen");
    }
}