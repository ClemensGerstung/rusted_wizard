use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Player {
    name: String,
    points: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tips {
    tips: HashMap<String, u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoundState {
    Tipping,
    Retipping,
    Playing,
    Checking,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Round {
    round_nr: u32,
    state: RoundState,
    tips: Tips,
    matches: Tips,
    players: Vec<Player>,
    current_player_index: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WizardState {
    Init,
    NextRound,
    Playing,
    EndRound,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wizard {
    pub state: WizardState,
    round_count: usize,
    round_index: usize,
    pub player_count: usize,
    player_index: usize,
    players: Vec<Player>,
    rounds: Vec<Round>,
    current_round: Option<Round>,
}

impl Player {
    pub fn new(name: String) -> Self {
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
    pub fn new() -> Self {
        Self {
            tips: HashMap::new(),
        }
    }

    pub fn add_tip(&mut self, player: &Player, tip: u8) {
        self.tips.insert(player.name.to_string(), tip);
    }

    pub fn get_tip(&self, player: &Player) -> u8 {
        self.tips.get(&player.name).copied().unwrap_or(0)
    }

    pub fn sum(&self) -> u32 {
        self.tips.values().copied().map(u32::from).sum()
    }
}

impl Round {
    pub fn new(round_nr: u32, players: Vec<Player>) -> Self {
        Self {
            round_nr,
            state: RoundState::Tipping,
            tips: Tips::new(),
            matches: Tips::new(),
            players,
            current_player_index: 0,
        }
    }

    pub fn play(&mut self, input_callback: impl Fn(&Player, &RoundState) -> u8) {
        if self.state == RoundState::Tipping || self.state == RoundState::Retipping {
            let current_player = &self.players[self.current_player_index];
            self.tips
                .add_tip(current_player, input_callback(current_player, &self.state));

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
            self.matches
                .add_tip(current_player, input_callback(current_player, &self.state));

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
            for player in self.players.iter_mut() {
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
        for player in self.players.iter() {
            writeln!(f, "{}", player)?;
        }

        write!(f, "{:?}", self.state)?;

        Ok(())
    }
}

impl Wizard {
    pub fn new(player_count: usize) -> Self {
        Self {
            state: WizardState::Init,
            round_count: 60 / player_count,
            round_index: 0,
            player_count,
            player_index: 0,
            players: Vec::with_capacity(player_count),
            rounds: Vec::with_capacity(60 / player_count),
            current_round: None,
        }
    }

    pub fn play(
        &mut self,
        player_callback: impl Fn(usize) -> String,
        input_callback: impl Fn(&Player, &RoundState) -> u8,
    ) {
        if self.state == WizardState::Init {
            let player = Player::new(player_callback(self.player_index));
            self.players.insert(self.player_index, player);

            if self.player_index + 1 == self.player_count {
                self.state = WizardState::NextRound;
                self.player_index = 0;
            } else {
                self.player_index += 1;
            }
        } else if self.state == WizardState::Playing {
            let current_round = self.current_round.as_mut().unwrap();

            if current_round.state == RoundState::End {
                self.state = WizardState::EndRound;
            }

            current_round.play(input_callback); // TODO: event/callback

            if self.round_index == self.round_count && current_round.state == RoundState::End {
                self.state = WizardState::End;
            }
        } else if self.state == WizardState::NextRound {
            self.round_index += 1;
            self.current_round =
                Option::from(Round::new(self.round_index as u32, self.players.to_vec()));

            self.state = WizardState::Playing;
        } else if self.state == WizardState::EndRound {
            let current_round = self.current_round.as_mut().unwrap();
            self.rounds
                .insert(self.round_index - 1, current_round.to_owned());
            self.players = current_round.players.to_vec();
            self.players.rotate_left(1);

            self.state = WizardState::NextRound;
        }
    }
}

impl Display for Wizard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for player in &self.players {
            write!(f, "{} ", player)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assertor::*;

    fn player_input(index: usize) -> String {
        let names = vec!["Player 1", "Player 2", "Player 3"];

        return String::from(names[index]);
    }

    fn empty_round_input(_player: &Player, _state: &RoundState) -> u8 {
        return 0;
    }

    #[test]
    fn initialization_no_state_change_state_is_init() {
        // arrange
        // act
        let wizard = Wizard::new(3);

        // assert
        assert_that!(wizard.state).is_equal_to(WizardState::Init);
    }

    #[test]
    fn initialization_input_names_state_is_next_round() {
        // arrange
        let mut wizard = Wizard::new(3);

        // act
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);

        // assert
        assert_that!(wizard.state).is_equal_to(WizardState::NextRound);
        assert_that!(wizard.players[0].name).is_equal_to(String::from("Player 1"));
        assert_that!(wizard.players[1].name).is_equal_to(String::from("Player 2"));
        assert_that!(wizard.players[2].name).is_equal_to(String::from("Player 3"));
    }

    #[test]
    fn initialization_input_names_state_is_playing() {
        // arrange
        let mut wizard = Wizard::new(3);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);

        // act
        wizard.play(player_input, empty_round_input);

        // assert
        assert_that!(wizard.current_round).is_some();
        let round = wizard.current_round.unwrap();
        assert_that!(round.state).is_equal_to(RoundState::Tipping);

        assert_that!(wizard.state).is_equal_to(WizardState::Playing);
    }

    #[test]
    fn tip_first_round() {
        // arrange
        let mut wizard = Wizard::new(3);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);

        // act
        wizard.play(player_input, |_, _| {
            return 1;
        });
        wizard.play(player_input, |_, _| {
            return 0;
        });
        wizard.play(player_input, |_, _| {
            return 1;
        });

        // assert
        assert_that!(wizard.current_round).is_some();
        let round = wizard.current_round.unwrap();
        assert_that!(round.state).is_equal_to(RoundState::Playing);

        assert_that!(wizard.state).is_equal_to(WizardState::Playing);
    }

    #[test]
    fn play_first_round() {
        // arrange
        let mut wizard = Wizard::new(3);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);

        wizard.play(player_input, |_, _| {
            return 1;
        });
        wizard.play(player_input, |_, _| {
            return 0;
        });
        wizard.play(player_input, |_, _| {
            return 1;
        });

        // act
        wizard.play(player_input, |_, _| {
            return 1;
        });
        wizard.play(player_input, |_, _| {
            return 0;
        });
        wizard.play(player_input, |_, _| {
            return 0;
        });

        // assert
        assert_that!(wizard.current_round).is_some();
        let round = wizard.current_round.unwrap();
        assert_that!(round.state).is_equal_to(RoundState::Checking);

        assert_that!(wizard.state).is_equal_to(WizardState::Playing);
    }

    #[test]
    fn check_first_round() {
        // arrange
        let mut wizard = Wizard::new(3);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);
        wizard.play(player_input, empty_round_input);

        wizard.play(player_input, |_, _| {
            return 1;
        });
        wizard.play(player_input, |_, _| {
            return 0;
        });
        wizard.play(player_input, |_, _| {
            return 1;
        });

        wizard.play(player_input, |_, _| {
            return 1;
        });
        wizard.play(player_input, |_, _| {
            return 0;
        });
        wizard.play(player_input, |_, _| {
            return 0;
        });

        // act
        wizard.play(player_input, empty_round_input); // check round
        wizard.play(player_input, empty_round_input); // end round
        wizard.play(player_input, empty_round_input); // next round

        // assert
        assert_that!(wizard.current_round).is_some();
        let round = wizard.current_round.unwrap();
        assert_that!(round.state).is_equal_to(RoundState::End);

        assert_that!(wizard.state).is_equal_to(WizardState::NextRound);

        let mut players = wizard.players.to_vec();
        players.rotate_right(1); // we need to rotate back one because end round already rotates players
        assert_that!(players[0].points).is_equal_to(30);
        assert_that!(players[1].points).is_equal_to(20);
        assert_that!(players[2].points).is_equal_to(-10);
    }
}
