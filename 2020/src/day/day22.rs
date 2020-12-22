/// Basically the card game of war.
/// The winner players card goes on top of the losing players card then placed
/// at the bottom of the winners deck.
///
/// Score is calculated from the bottom of the winner's deck when they have all the cards.
/// Bottom card value * 1 + next card value * 2 and so on.
///
/// # Example
/// ```
/// use advent_of_code_2020::day::day22::*;
///
/// let input = "Player 1:
///9
///2
///6
///3
///1
///
///Player 2:
///5
///8
///4
///7
///10".to_string();
/// let results = part1(&input);
/// assert_eq!(results, 306);
/// ```
pub fn part1(i: &String) -> usize {
  let mut players = parse(i);
  loop {
    if players[0].is_empty() || players[1].is_empty() {
      break;
    }
    play_round(&mut players);
  }
  let winner = players.iter().find(|p| !p.is_empty()).unwrap();
  score(winner)
}

fn parse(i: &String) -> Vec<Vec<usize>> {
  i.split("\n\n")
    .map(|player| {
      player
        .split("\n")
        .filter_map(|line| line.parse().ok())
        .collect()
    })
    .collect()
}

fn play_round(players: &mut Vec<Vec<usize>>) {
  let player_one_card = players[0].remove(0);
  let player_two_card = players[1].remove(0);
  if player_one_card > player_two_card {
    players[0].push(player_one_card);
    players[0].push(player_two_card);
  } else {
    players[1].push(player_two_card);
    players[1].push(player_one_card);
  }
}

fn score(deck: &Vec<usize>) -> usize {
  deck
    .iter()
    .rev()
    .enumerate()
    .fold(0, |t, (i, v)| t + ((i + 1) * v))
}

/// Now it gets recursive with some weird rules:
/// - Before either player deals a card, if there was a previous round in this game that had
///   exactly the same cards in the same order in the same players' decks, the game instantly ends in a win for player 1.
/// - If both players have at least as many cards remaining in their deck as the value of the card they just drew,
///   the winner of the round is determined by playing a new game of Recursive Combat
/// - Otherwise, at least one player must not have enough cards left in their deck to recurse;
///   the winner of the round is the player with the higher-value card.
///
/// # Example
/// ```
/// use advent_of_code_2020::day::day22::*;
///
/// let input = "Player 1:
///9
///2
///6
///3
///1
///
///Player 2:
///5
///8
///4
///7
///10".to_string();
/// let results = part2(&input);
/// assert_eq!(results, 291);
/// ```
pub fn part2(i: &String) -> usize {
  let mut players = parse(i);
  let winner = play_game(&mut players);
  score(&players[winner])
}

fn play_game(players: &mut Vec<Vec<usize>>) -> usize {
  let mut states: Vec<Vec<Vec<usize>>> = vec![vec![], vec![]];
  let mut winner = 0;
  loop {
    if states[0].contains(&players[0]) || states[1].contains(&players[1]) {
      break;
    }
    if players[0].is_empty() {
      winner = 1;
      break;
    }
    if players[1].is_empty() {
      break;
    }
    states[0].push(players[0].clone());
    states[1].push(players[1].clone());
    let player_one_card = players[0].remove(0);
    let player_two_card = players[1].remove(0);
    if players[0].len() >= player_one_card && players[1].len() >= player_two_card {
      let mut sub_game_players = vec![
        players[0][0..player_one_card].to_vec(),
        players[1][0..player_two_card].to_vec(),
      ];
      let winner = play_game(&mut sub_game_players);
      if winner == 0 {
        players[0].push(player_one_card);
        players[0].push(player_two_card);
      } else {
        players[1].push(player_two_card);
        players[1].push(player_one_card);
      }
    } else {
      if player_one_card > player_two_card {
        players[0].push(player_one_card);
        players[0].push(player_two_card);
      } else {
        players[1].push(player_two_card);
        players[1].push(player_one_card);
      }
    }
  }
  winner
}
