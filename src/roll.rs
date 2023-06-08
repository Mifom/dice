use rand::Rng;

#[derive(PartialEq, Eq, Debug)]
pub enum Roll {
    Sum(Box<Roll>, Box<Roll>),
    Dices {
        mul: u16,
        max: u16,
        advantage: Option<bool>,
    },
}

pub struct DiceResult(Vec<(Vec<u16>, Option<bool>)>);

impl Roll {
    pub fn roll(&self) -> DiceResult {
        let mut rng = rand::thread_rng();
        let mut results = Vec::new();
        self.roll_inner(&mut rng, &mut results);
        DiceResult(results)
    }

    fn roll_inner(&self, rng: &mut impl Rng, results: &mut Vec<(Vec<u16>, Option<bool>)>) {
        match self {
            Roll::Sum(left, right) => {
                left.roll_inner(rng, results);
                right.roll_inner(rng, results);
            }
            Roll::Dices {
                mul,
                max,
                advantage,
            } => {
                if *max == 1 {
                    results.push((vec![*mul], None));
                } else {
                    let dice_num = if advantage.is_some() { *mul + 1 } else { *mul };
                    let rolls: Vec<_> = (0..dice_num).map(|_| rng.gen_range(1..=*max)).collect();
                    results.push((rolls, advantage.clone()));
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
            Roll::Dices {
                mul,
                max,
                advantage,
            } => {
                let adv = match advantage {
                    Some(true) => " with adv",
                    Some(false) => " with disadv",
                    None => "",
                };
                if *max == 1 {
                    write!(f, "{mul}{adv}")
                } else {
                    write!(f, "{mul}d{max}{adv}")
                }
            }
        }
    }
}

impl DiceResult {
    pub fn calc(&self) -> u16 {
        self.0
            .iter()
            .cloned()
            .map(|(mut rolls, advantage)| {
                if let Some(bottom) = advantage.as_ref() {
                    let iter = rolls.iter().enumerate();
                    let removed = if *bottom {
                        iter.min_by_key(|(_, key)| *key)
                    } else {
                        iter.max_by_key(|(_, key)| *key)
                    };
                    rolls.remove(removed.unwrap().0);
                }
                rolls.into_iter().sum::<u16>()
            })
            .sum()
    }
}

impl std::fmt::Display for DiceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let center = self
            .0
            .iter()
            .cloned()
            .map(|(mut scope, advantage)| {
                let removed = if let Some(bottom) = advantage.as_ref() {
                    let iter = scope.iter().cloned().enumerate();
                    let removed = if *bottom {
                        iter.min_by_key(|(_, key)| *key)
                    } else {
                        iter.max_by_key(|(_, key)| *key)
                    }
                    .unwrap();
                    scope.remove(removed.0);
                    Some(removed.1)
                } else {
                    None
                };
                let basic = scope
                    .iter()
                    .map(|roll| roll.to_string())
                    .collect::<Vec<_>>()
                    .join(" + ");
                if let Some(removed) = removed {
                    format!("{basic} | {removed}")
                } else {
                    basic
                }
            })
            .collect::<Vec<_>>()
            .join(") + (");
        write!(f, "({center})")
    }
}
