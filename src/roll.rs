use rand::Rng;

#[derive(PartialEq, Eq, Debug)]
pub enum Roll {
    Sum(Box<Roll>, Box<Roll>),
    Dices { mul: u16, max: u16 },
}

impl Roll {
    pub fn roll(&self) -> u16 {
        let mut rng = rand::thread_rng();
        match self {
            Roll::Sum(left, right) => left.roll() + right.roll(),
            Roll::Dices { mul, max } => (0..*mul).map(|_| rng.gen_range(1..=*max)).sum(),
        }
    }
}

impl std::fmt::Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Roll::Sum(l, r) => {
                l.fmt(f)?;
                f.write_str(" + ")?;
                r.fmt(f)
            }
            Roll::Dices { mul, max } => {
                write!(f, "{mul}d{max}")
            }
        }
    }
}
