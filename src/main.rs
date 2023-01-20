
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Player {
    name: String,
    points: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tips {
    tips: HashMap<String, u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RoundState {
    Tipping,
    Retipping,
    Playing,
    Checking,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Round {
    round_nr: u32,
    state: RoundState,
    tips: Tips,
    matches: Tips,
    players: Vec<Player>,
    current_player_index: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum WizardState {
    Init,
    Playing,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Wizard {
    state: WizardState,
    round_count: usize,
    round_index: usize,
    player_count: usize,
    player_index: usize,
    players: Vec<Player>,
    rounds: Vec<Round>,
}

impl Player {
    fn new(name: String) -> Self {
        Self { name, points: 0 }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.points)?;

        Ok(())
    }
}

impl Tips {
    fn new() -> Self {
        Self { tips: HashMap::new() }
    }

    fn add_tip(&mut self, player:&Player, tip: u8) {
        self.tips.insert(player.name.to_string(), tip);
    }

    fn get_tip(&self, player:&Player) -> u8 {
        self.tips.get(&player.name)
            .copied()
            .unwrap_or(0)
    }

    fn sum(&self) -> u32 {
        self.tips.values().copied().map(u32::from).sum()
    }
}

impl Round {
    fn new(round_nr: u32, players: Vec<Player>) -> Self {
        Self {
            round_nr,
            state: RoundState::Tipping,
            tips: Tips::new(),
            matches: Tips::new(),
            players,
            current_player_index: 0
        }
    }

    fn play(&mut self, input_callback: fn(&Player) -> u8) {
        if self.state == RoundState::Tipping || self.state == RoundState::Retipping {
            let current_player = &self.players[self.current_player_index];
            self.tips.add_tip(current_player, input_callback(current_player));

            if self.current_player_index + 1 == self.players.len() {
                let sum_of_tips = self.tips.sum();
                self.state = if sum_of_tips == self.round_nr {
                     RoundState::Retipping
                } else {
                     RoundState::Playing
                };

                self.current_player_index = 0;
            } else {
                self.current_player_index += 1;
            }
        } else if self.state == RoundState::Playing {
            let current_player = &self.players[self.current_player_index];
            self.matches.add_tip(current_player, input_callback(current_player));

            if self.current_player_index == self.players.len() - 1 {
                let sum_of_matches = self.matches.sum();
                if sum_of_matches == self.round_nr {
                    self.state = RoundState::Checking;
                }

                self.current_player_index = 0;
            } else {
                self.current_player_index += 1;
            }
        } else if self.state == RoundState::Checking {
            for player in &mut self.players {
                let tip = self.tips.get_tip(player);
                let matched = self.matches.get_tip(player);
                let diff = u8::abs_diff(tip, matched);

                if diff == 0 {
                    player.points += 20 + i16::from(tip) * 10
                } else {
                    player.points -= i16::from(diff) * 10;
                };

            }

            self.state = RoundState::End;
        }
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for player in &self.players {
            writeln!(f, "{}", player)?;
        }

        write!(f, "{:?}", self.state)?;

        Ok(())
    }
}

fn main() {
    let player1 = Player::new(String::from("Player 1"));
    let player2 = Player::new(String::from("Player 2"));

    let players = vec![player1, player2];

    let mut round = Round::new(1, players);
    println!("{}", round);
    round.play(|_| { 1 }); // player 1 tips

    println!("{}", round);
    round.play(|_| { 1 }); // player 2 tips

    println!("{}", round);
    round.play(|_| { 0 }); // player 1 matches

    println!("{}", round);
    round.play(|_| { 1 }); // player 2 matches

    println!("{}", round);
    round.play(|_| { 0 }); // evaluate round

    println!("{}", round);
}
