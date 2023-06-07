use rand::Rng;

#[derive(PartialEq, Eq, Debug)]
pub enum Roll {
    Sum(Box<Roll>, Box<Roll>),
    Dices { mul: u16, max: u16 },
}

pub struct DiceResult(Vec<Vec<u16>>);

impl Roll {
    pub fn roll(&self) -> DiceResult {
        let mut rng = rand::thread_rng();
        let mut results = Vec::new();
        self.roll_inner(&mut rng, &mut results);
        DiceResult(results)
    }

    fn roll_inner(&self, rng: &mut impl Rng, results: &mut Vec<Vec<u16>>) {
        match self {
            Roll::Sum(left, right) => {
                left.roll_inner(rng, results);
                right.roll_inner(rng, results);
            }
            Roll::Dices { mul, max } => {
                if *max == 1 {
                    results.push(vec![*mul]);
                } else {
                    results.push((0..*mul).map(|_| rng.gen_range(1..=*max)).collect())
                }
            }
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
                if *max == 1 {
                    write!(f, "{mul}")
                } else {
                    write!(f, "{mul}d{max}")
                }
            }
        }
    }
}

impl DiceResult {
    pub fn calc(&self) -> u16 {
        self.0.iter().flatten().copied().sum()
    }
}

impl std::fmt::Display for DiceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let center = self
            .0
            .iter()
            .map(|scope| {
                scope
                    .iter()
                    .map(|roll| roll.to_string())
                    .collect::<Vec<_>>()
                    .join(" + ")
            })
            .collect::<Vec<_>>()
            .join(") + (");
        write!(f, "({center})")
    }
}
