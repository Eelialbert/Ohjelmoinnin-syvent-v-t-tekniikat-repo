// Yksinkertainen tapa erotella ja kategorisoida tehtäviä.

#[derive(Debug, PartialEq)]
pub struct Category {
    pub primary: String,
    pub secondary: Option<String>,
}

impl Category {
    pub fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.into(),
            secondary: Some(secondary.into()),
        }
    }

    pub fn primary_only(primary: &str) -> Self {
        Self {
            primary: primary.into(),
            secondary: None,
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s.split_once('/') {
            Some((p, s)) => Self::new(p, s),
            None => Self::primary_only(s),
        }
    }
}

fn main() {}
// Testit on eritelty ettei koko ohjelmaa tarvitse ajaa joka kerta.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separate_primary_and_secondary() {
        assert_eq!(
            Category::new("Work", "Email"),
            Category {
                primary: "Work".into(),
                secondary: Some("Email".into()),
            }
        );
    }

    #[test]
    fn primary_only() {
        assert_eq!(
            Category::primary_only("Personal"),
            Category {
                primary: "Personal".into(),
                secondary: None,
            }
        );
    }

    #[test]
    fn parse_with_slash() {
        assert_eq!(
            Category::from_string("Home/Gardening"),
            Category {
                primary: "Home".into(),
                secondary: Some("Gardening".into()),
            }
        );
    }

    #[test]
    fn parse_without_slash() {
        assert_eq!(
            Category::from_string("Health"),
            Category {
                primary: "Health".into(),
                secondary: None,
            }
        );
    }
}