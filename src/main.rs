use rollr::{dices, throw::{flip_coin, throw_dices}};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.get(1).map(|s| s.to_ascii_lowercase()) {
    Some(s) if ["f", "flip", "flipcoin"].contains(&s.as_str()) => {
      let result = flip_coin();
      println!("🪙 {} !", if result { "Pile" } else { "Face" });
    }
    _ => {
      let roll: dices::DiceRoll = dices::parse_dice_arg(&args);
      let results: Vec<u16> = throw_dices(roll.count, roll.dice_type.value());
      println!("{}D{} : {:?}", roll.count, roll.dice_type.value(), results);
    }
  }
}
