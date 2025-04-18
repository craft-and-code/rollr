use rand::Rng;

/// Simulates rolling a given number of dice with a specified number of sides.
///
/// # Arguments
///
/// * `count` - The number of dice to roll.
/// * `dice_sides` - The number of sides on each die.
///
/// # Returns
///
/// A vector of integers representing the result of each individual die roll.
pub fn throw_dices(count: u16, dice_sides: u16) -> Vec<u16> {
  let mut rng = rand::rng();
  (0..count).map(|_| rng.random_range(1..=dice_sides)).collect()
}

/// Simulates flipping a coin.
///
/// # Returns
///
/// * `true` for heads (Pile).
/// * `false` for tails (Face).
pub fn flip_coin() -> bool {
  rand::random()
}

#[test]
fn test_throw_dices() {
  let results = throw_dices(2, 6);
  assert_eq!(results.len(), 2);
  for result in results {
    assert!(result >= 1 && result <= 6);
  }
}

#[test]
fn test_flip_coin() {
  let result = flip_coin();
  assert!(result == true || result == false);
}
