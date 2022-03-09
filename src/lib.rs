use std::fmt::format;

const MEGAS: [&str; 12] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
];
const UNITS: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const DECINES: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

#[derive(std::fmt::Debug)]
pub enum InputError {
    NegativeInteger(u64),
    TooBigInteger(u64),
}

trait TripletTrait {
    fn to_triplet(&self) -> Vec<u64>;
}

impl TripletTrait for u64 {
    fn to_triplet(&self) -> Vec<u64> {
        let mut triplets = vec![];
        let mut number = *self;

        while number > 0 {
            triplets.push(number % 1000);
            number /= 1000;
        }

        triplets
    }
}

pub fn to_eng(input: u64) -> Result<String, InputError> {
    let triplets = input.to_triplet();

    if triplets.is_empty() {
        return Ok(String::from("zero"));
    }

    let mut world = vec![];

    let mut last_t = 1_u64;
    for (i, triplet) in triplets.iter().rev().enumerate() {
        // three digits
        let hundreds = triplet / 100 % 10;
        let dec = triplet / 10 % 10;
        let units = triplet % 10;

        if i > 0 && last_t > 0 {
            world.push(String::from(MEGAS[triplets.len() - i as usize]));
        }

        if hundreds > 0 {
            world.push(format!("{} hundred", UNITS[hundreds as usize]));
        }

        match dec {
            0 => {
                if units > 0 {
                    world.push(String::from(UNITS[units as usize]))
                }
            }
            dec => {
                if dec > 1 {
                    if units > 0 {
                        world.push(format!(
                            "{}-{}",
                            DECINES[dec as usize], UNITS[units as usize],
                        ));
                    } else {
                        world.push(DECINES[dec as usize].to_string());
                    }
                } else if dec == 1 {
                    world.push(String::from(TEENS[units as usize]));
                } else if dec == 0 && units > 0 {
                    world.push(String::from(UNITS[units as usize]));
                }
            }
        }

        //Save last triplet for test for MILES
        last_t = *triplet;
    }

    Ok(world.join(" "))
}

pub fn encode(n: u64) -> String {
    to_eng(n).expect("invalid number")
}
