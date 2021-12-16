// Oliver Kovacs 2021 MIT

use clap::{Arg, App};
use rand::Rng;
use serde::Serialize;
use serde_json;

#[derive(Serialize, Debug)]
struct Roll {
    amount: i32,
    faces: i32,
    modifier: i32,
    operator: char,
    rolls: Vec<i32>,
    sum: i32,
    result: i32,
}

fn main() {
    let matches = App::new("roll")
        .version("0.1")
        .author("Oliver Kovacs <oliver.kovacs.dev@gmail.com>")
        .about("Roll a dice.")
        .arg(Arg::with_name("DICE")
            .help("The dice to roll")
            .required(true)
            .index(1)
            .takes_value(true)
            .min_values(1)
        )
        .arg(Arg::with_name("JSON")
            .help("Output in JSON")
            .long("json")
            .short("j")
            .takes_value(false)
        )
        .arg(Arg::with_name("sum")
            .help("Output sum only")
            .long("sum")
            .short("s")
            .takes_value(false)
        )
        .arg(Arg::with_name("amount")
            .help("The default amount of the dice to be rolled.")
            .long("amount")
            .default_value("1")
        )
        .arg(Arg::with_name("faces")
            .help("The default faces of the dice to be rolled.")
            .long("type")
            .default_value("6")
        )
        .arg(Arg::with_name("modifier")
            .help("The default modifier to use.")
            .long("modifier")
            .default_value("0")
        )
        .get_matches();

    let default: [i32; 3] = [ 1, 6, 0 ];
    let arguments: [&str; 3] = [ "amount", "faces", "modifier" ];
    let default = (0..3)
        .map(|i| matches
            .value_of(arguments[i])
            .unwrap()
            .parse::<i32>()
            .unwrap_or(default[i]))
        .collect::<Vec<i32>>();

    let rolls: Vec<Roll> = matches
        .values_of("DICE")
        .unwrap()
        .into_iter()
        .map(|die| {
            let mut strings: [String; 3] = Default::default();
            let mut index = 0;
            let mut operator = '+';
            for character in die.chars() {
                match character {
                    '+' | '-' | '*' | '/' => {
                        index += 1;
                        operator = character;
                    },
                    'd' => index += 1,
                    _ => strings[index].push(character),
                }
            }

            let numbers = strings
                .into_iter()
                .enumerate()
                .map(|(i, e) | e.parse::<i32>().unwrap_or(default[i]))
                .collect::<Vec<i32>>();

            let (amount, faces, modifier) = (numbers[0], numbers[1], numbers[2]);

            let rolls = vec![0; amount as usize]
                .into_iter()
                .map(|_| rand::thread_rng().gen_range(1..faces + 1) as i32)
                .collect::<Vec<i32>>();

            let sum = rolls.iter().sum();
            let result = match operator {
                '-' => sum - modifier,
                '*' => sum * modifier,
                '/' => (sum + 1) / modifier,
                '+' | _ => sum + modifier,
            };

            Roll { amount, faces, modifier, operator, rolls, sum, result }
        })
        .collect::<Vec<Roll>>();
    
    let sum = rolls
        .iter()
        .map(|roll| roll.result)
        .sum::<i32>();
        
    if matches.is_present("JSON") {
        return println!("{}", serde_json::to_string_pretty(&rolls).unwrap());
    }
    if matches.is_present("sum") {
        return println!("{}", sum);
    }

    let list = rolls
        .iter()
        .map(|roll| {
            let string = match (roll.modifier, roll.operator) {
                (0, '+' | '-') | (1, '*' | '/') => String::from(""),
                _ => format!(
                    "({}{}{}) ",
                    roll.sum,
                    roll.operator,
                    roll.modifier
                ),
            };
            let roll_string = roll.rolls
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" ");

            format!("{{ {} {}[{}] }}", roll.result, string, roll_string)
        })
        .collect::<Vec<String>>()
        .join(" ");
    println!("{} {}", sum, format!("{}", list));
}
