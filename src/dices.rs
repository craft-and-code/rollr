use regex::Regex;

/// Represents the different kinds of dice available for rolling.
///
/// This enum restricts rolls to standard RPG dice such as D6, D20, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeOfDice {
  D3 = 3,
  D4,
  D5,
  D6,
  D7,
  D8,
  D10 = 10,
  D12 = 12,
  D14 = 14,
  D16 = 16,
  D20 = 20,
  D24 = 24,
  D30 = 30,
  D50 = 50,
  D60 = 60,
  D100 = 100,
}

impl TypeOfDice {
  /// Attempts to convert a numeric value into a valid `TypeOfDice` variant.
  ///
  /// # Arguments
  ///
  /// * `n` - The numeric value representing the sides of the die.
  ///
  /// # Returns
  ///
  /// * `Some(TypeOfDice)` if the value is a supported dice type.
  /// * `None` if the value does not match any known dice types.
  pub fn from_u16(n: u16) -> Option<Self> {
    use TypeOfDice::*;
    match n {
      3 => Some(D3),
      4 => Some(D4),
      5 => Some(D5),
      6 => Some(D6),
      7 => Some(D7),
      8 => Some(D8),
      10 => Some(D10),
      12 => Some(D12),
      14 => Some(D14),
      16 => Some(D16),
      20 => Some(D20),
      24 => Some(D24),
      30 => Some(D30),
      50 => Some(D50),
      60 => Some(D60),
      100 => Some(D100),
      _ => None,
    }
  }

  pub fn value(&self) -> u16 {
    *self as u16
  }
}

/// Represents a dice roll request, including the number of dice and their type.
#[derive(Debug)]
pub struct DiceRoll {
  /// The number of dice to roll.
  pub count: u16,
  /// The type of dice to roll, e.g., D6, D20.
  pub dice_type: TypeOfDice,
}

/// Parses the command-line argument to determine how many dice to roll and of what type.
///
/// # Example
///
/// ```
/// use rollr::dices::parse_dice_arg;
///
/// let args = vec!["".to_string(), "2D20".to_string()];
/// let roll = parse_dice_arg(&args);
/// assert_eq!(roll.count, 2);
/// assert_eq!(roll.dice_type.value(), 20);
/// ```
pub fn parse_dice_arg(args: &Vec<String>) -> DiceRoll {
  match args.get(1) {
    Some(arg) => {
      let re = Regex::new(r"(?i)^(\d*)D(\d+)$").unwrap(); // (?i) = ignore case
      if let Some(caps) = re.captures(arg) {
        let count = caps.get(1).map_or("1", |m| m.as_str()).parse().unwrap_or(1);
        let sides = caps.get(2).unwrap().as_str().parse().unwrap_or(6);
        let dice_type = TypeOfDice::from_u16(sides).unwrap_or(TypeOfDice::D6);

        DiceRoll { count, dice_type }
      } else {
        DiceRoll {
          count: 1,
          dice_type: TypeOfDice::D6,
        }
      }
    }
    None => DiceRoll {
      count: 1,
      dice_type: TypeOfDice::D6,
    },
  }
}

#[test]
fn test_throw() {
  let dices: DiceRoll = parse_dice_arg(&vec!["".to_string(), "".to_string()]);
  assert_eq!(dices.count, 1);
  assert_eq!(dices.dice_type, TypeOfDice::D6);

  let dices: DiceRoll = parse_dice_arg(&vec!["".to_string(), "1".to_string()]);
  assert_eq!(dices.count, 1);
  assert_eq!(dices.dice_type, TypeOfDice::D6);

  let dices: DiceRoll = parse_dice_arg(&vec!["".to_string(), "d".to_string()]);
  assert_eq!(dices.count, 1);
  assert_eq!(dices.dice_type, TypeOfDice::D6);

  let dices: DiceRoll = parse_dice_arg(&vec!["".to_string(), "1D".to_string()]);
  assert_eq!(dices.count, 1);
  assert_eq!(dices.dice_type, TypeOfDice::D6);

  let dices: DiceRoll = parse_dice_arg(&vec!["".to_string(), "2D99".to_string()]);
  assert_eq!(dices.count, 2);
  assert_eq!(dices.dice_type, TypeOfDice::D6);

  let dices: DiceRoll = parse_dice_arg(&vec!["".to_string(), "fezfe".to_string()]);
  assert_eq!(dices.count, 1);
  assert_eq!(dices.dice_type, TypeOfDice::D6);

  let dices: DiceRoll = parse_dice_arg(&vec!["".to_string(), "D8".to_string()]);
  assert_eq!(dices.count, 1);
  assert_eq!(dices.dice_type, TypeOfDice::D8);

  let dices: DiceRoll = parse_dice_arg(&vec!["".to_string(), "2D6".to_string()]);
  assert_eq!(dices.count, 2);
  assert_eq!(dices.dice_type, TypeOfDice::D6);

  let dices: DiceRoll = parse_dice_arg(&vec!["".to_string(), "3D20".to_string()]);
  assert_eq!(dices.count, 3);
  assert_eq!(dices.dice_type, TypeOfDice::D20);
}
