// 2. Format a response that includes individual dice
//    e.g. 4d6 => d6(3, 5, 1, 6)
// 3. impl and iterator that would support roll_dice("3d6").to_iter().take(6);
// 5. Module docs
//      #![warn(missing_docs)]
// 8. Publish to crates.io

extern crate rand;
extern crate regex;

use rand::{thread_rng, Rng};
use regex::Regex;

/// Roll struct contains the `DieRollExpression` (if applicable), the values of
/// the individual rolls, and the calculated total.
#[derive(Debug)]
pub struct Roll {
    // die roll expression
    drex: String,
    // individual die roll results
    values: Vec<(DieRollTerm, Vec<i8>)>,
    // result of the drex
    total: i32,
}

/// NEED DOCS
#[derive(Debug, Clone)]
pub enum DieRollTerm {
    DieRoll { multiplier: i8, sides: u8 },
    Modifier(i8),
}


impl DieRollTerm {
    fn parse(drt: &str) -> DieRollTerm {
        if drt.to_lowercase().contains('d') {
            let v: Vec<&str> = drt.split("d").collect();
            DieRollTerm::DieRoll {
                multiplier: v[0].parse::<i8>().unwrap(),
                sides: v[1].parse::<u8>().unwrap(),
            }
        } else {
            DieRollTerm::Modifier(drt.parse::<i8>().unwrap())
        }
    }


    fn calculate(v: (DieRollTerm, Vec<i8>)) -> i32 {
        match v.0 {
            DieRollTerm::Modifier(n) => n as i32,
            DieRollTerm::DieRoll { multiplier: m, .. } => {
                let mut sum: i32 = v.1.iter().fold(0i32, |sum, &val| sum + val as i32);
                if m < 0 {
                    sum = sum * -1;
                }
                sum
            }
        }
    }

    fn evaluate(self) -> (DieRollTerm, Vec<i8>) {
        match self {
            DieRollTerm::Modifier(n) => (self, vec![n]),
            DieRollTerm::DieRoll { multiplier: m, sides: s } => {
                (self, (0..m.abs()).map(|_| thread_rng().gen_range(1, s as i8 + 1)).collect())
            }
        }
    }
}

/// `roll_dice()` will evaluate the string input as a die roll expression (e.g. 3d6 + 4).
pub fn roll_dice<'a>(s: String) -> Result<Roll, &'a str> {
    let s: String = s.split_whitespace().collect();
    let terms: Vec<DieRollTerm> = parse_die_roll_terms(&s);

    if terms.len() == 0 {
        Err("Invalid die roll expression: no die roll terms found.")
    } else {

        let v: Vec<_> = terms.into_iter().map(|t| t.evaluate()).collect();
        let t = v.clone();

        Ok(Roll {
            drex: s.to_string(),
            values: v,
            total: t.into_iter().fold(0i32, |sum, val| sum + DieRollTerm::calculate(val)),
        })
    }
}

fn parse_die_roll_terms(drex: &str) -> Vec<DieRollTerm> {
    let mut terms = Vec::new();

    let re = Regex::new(r"([+-]?\s*\d+[dD]\d+|[+-]?\s*\d+)").unwrap();

    let matches = re.find_iter(drex);
    for m in matches {
        println!("{:?}", m);
        terms.push(DieRollTerm::parse(&drex[m.start()..m.end()]));
    }
    terms
}

/// `roll_range()` will generate a random number within the specified range and return that value.
pub fn roll_range<'a>(min: i32, max: i32) -> Result<i32, &'a str> {
    if min > max {
        Err("Invalid range: min must be less than or equal to max")
    } else {
        Ok(thread_rng().gen_range(min, max + 1))
    }
}

#[cfg(test)]
mod tests;
