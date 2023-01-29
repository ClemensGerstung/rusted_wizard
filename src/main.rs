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
  NextRound,
  Playing,
  EndRound,
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
  current_round: Option<Round>,
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

  fn add_tip(&mut self, player: &Player, tip: u8) {
    self.tips.insert(player.name.to_string(), tip);
  }

  fn get_tip(&self, player: &Player) -> u8 {
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
      current_player_index: 0,
    }
  }

  fn play(&mut self, input_callback: fn(&Player, &RoundState) -> u8) {
    if self.state == RoundState::Tipping || self.state == RoundState::Retipping {
      let current_player = &self.players[self.current_player_index];
      self.tips.add_tip(current_player, input_callback(current_player, &self.state));

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
      self.matches.add_tip(current_player, input_callback(current_player, &self.state));

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
  fn new(player_count: usize) -> Self {
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

  fn play(&mut self, player_callback: fn(usize) -> String, input_callback: fn(&Player, &RoundState) -> u8) {
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
      self.current_round = Option::from(Round::new(self.round_index as u32, self.players.to_vec()));

      self.state = WizardState::Playing;
    } else if self.state == WizardState::EndRound {
      let current_round = self.current_round.as_mut().unwrap();
      self.rounds.insert(self.round_index - 1, current_round.to_owned());
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

fn input_player_names(index: usize) -> String {
  println!("Player {}", index);
  let player_names = vec!["qwer", "asdf", "zxcv"];

  return String::from(player_names[index]);
}

fn input_player_input(player: &Player, state: &RoundState) -> u8 {
  let ts = if *state == RoundState::Tipping {
    "Tipping"
  } else if *state == RoundState::Retipping {
    "Retipping"
  } else if *state == RoundState::Playing {
    "Matching"
  } else {
    ""
  };

  println!("> {} of {}: ", ts, player.name);

  let mut user_input = String::new();
  let stdin = std::io::stdin();
  let _ = stdin.read_line(&mut user_input);

  let parse_result = user_input.trim().parse::<u8>();
  return match parse_result {
    Ok(i) => i,
    Err(..) => 0
  };
}

fn main() {
  let mut game = Wizard::new(3);

  while game.state != WizardState::End {
    game.play(input_player_names, input_player_input);

    if game.state == WizardState::NextRound {
      println!("{}", game);
    }
  }
}
