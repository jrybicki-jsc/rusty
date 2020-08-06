use std::fmt;

enum Sex {
    Male,
    Female,
}

pub struct Rat {
    weight: u16,
    sex: Sex,
}

impl fmt::Display for Rat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.weight, self.sex)
    }
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Male => write!(f, "Male"),
            _ => write!(f, "Female"),
        }
    }
}

fn main() {
    let r = Rat {
        weight: 19,
        sex: Sex::Male,
    };

    println!("Got rat {}", r);
}
