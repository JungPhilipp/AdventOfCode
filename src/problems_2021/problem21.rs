use std::collections::HashMap;

use log::{debug, info};

pub fn solve() {
    info!(
        "Solutions Day 21:\nPart1{}\nPart2{}",
        solve_part1(),
        solve_part2()
    );
}

fn play(mut positions: Vec<u64>, die: &mut dyn FnMut() -> u64) -> (Vec<u64>, u64) {
    let mut round = 1;
    let mut scores = vec![0; positions.len()];
    let mut active = 0;
    loop {
        let new_pos = ((positions[active] + die() - 1) % 10) + 1;
        positions[active] = new_pos;
        scores[active] += new_pos;

        if scores[active] >= 1000 {
            debug!(
                "Player {} won with {} points on round {}",
                active, scores[active], round
            );
            break;
        }
        active = (active + 1) % positions.len();
        round += 1;
    }

    (scores, round)
}

struct DeterministicDie {
    value: u64,
}

impl DeterministicDie {
    fn roll(&mut self) -> u64 {
        let result = self.value;
        self.value += 1;
        self.value = (self.value - 1 % 100) + 1;
        result
    }
}

pub fn solve_part1() -> u64 {
    let mut die = DeterministicDie { value: 1 };
    let mut roll_die = || {
        let mut sum = 0;
        for _ in 0..3 {
            sum += die.roll();
        }
        sum
    };
    let (scores, rounds) = play(vec![5, 9], &mut roll_die);
    scores.iter().min().unwrap() * rounds * 3
}

fn compute_next_pos(pos: usize, step: usize) -> usize {
    (pos + step - 1) % 10 + 1
}

fn compute_scores(start_pos: usize, max_score: u64, max_rounds: usize) -> Vec<(usize, usize)> {
    let mut round_statistics = vec![(0, 0); max_rounds];
    let mut cache: Vec<Vec<usize>> = vec![vec![]; 11];

    let mut last_round = HashMap::<(usize, u64), usize>::new();
    last_round.insert((start_pos, 0), 1);

    for round in 0..max_rounds {
        let mut current_round = HashMap::<(usize, u64), usize>::new();
        for ((pos, score), times) in last_round {
            assert_ne!(times, 0);
            assert!((1..=10).contains(&pos));
            if score >= max_score {
                continue;
            }

            let new_positions = {
                if cache[pos].is_empty() {
                    let mut new_positions = vec![0; 11];
                    for first in 1..=3 {
                        for second in 1..=3 {
                            for third in 1..=3 {
                                let next_pos = compute_next_pos(pos, first + second + third);
                                new_positions[next_pos] += 1;
                            }
                        }
                    }
                    cache[pos] = new_positions;
                }
                &cache[pos]
            };
            for (new_pos, new_pos_times) in new_positions.iter().enumerate() {
                if *new_pos_times == 0 {
                    continue;
                }
                *current_round
                    .entry((new_pos, score + new_pos as u64))
                    .or_insert(0) += times * new_pos_times;
            }
        }
        let finished = current_round
            .iter()
            .filter_map(|((_, score), times)| if *score >= 21 { Some(times) } else { None })
            .sum::<usize>();
        let not_finished = current_round
            .iter()
            .filter_map(|((_, score), times)| if *score < 21 { Some(times) } else { None })
            .sum::<usize>();
        round_statistics[round] = (finished, not_finished);
        last_round = current_round;
    }

    round_statistics
}
pub fn solve_part2() -> usize {
    let player1_scores = compute_scores(5, 21, 11);
    dbg!(player1_scores.clone());
    let player2_scores = compute_scores(9, 21, 11);
    dbg!(player2_scores.clone());

    let mut player1_wins = 0;
    let mut player2_wins = 0;
    let mut player2_not_won_last = 0;
    for round in 0..player2_scores.len() {
        let (player1_finished, player1_not_finished) = player1_scores[round];
        let (player2_finished, player2_not_finished) = player2_scores[round];
        let player2_won = player2_finished * player1_not_finished;
        player2_wins += player2_won;
        let player1_won = player1_finished * player2_not_won_last;
        player1_wins += player1_won;
        let no_winner = player1_not_finished * player2_not_finished;

        debug!("{} {} {}", player1_won, player2_won, no_winner);
        player2_not_won_last = player2_not_finished;
    }

    dbg!(player1_wins, player2_wins);
    player1_wins.max(player2_wins)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(), 989352);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(), 430229563871565);
    }
}
