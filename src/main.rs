use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Eq, Hash)]
struct Player {
    name:String,
    points:i16
}

struct Tips {
    tips:HashMap<String, i8>
}

#[derive(PartialEq)]
#[derive(Debug)]
enum RoundState {
    TIPPING,
    RETIPPING,
    PLAYING,
    CHECKING,
    END,
}

struct Round {
    round_nr:i8,
    state:RoundState,
    tips:Tips,
    matches:Tips,
    players:Vec<Rc<RefCell<Player>>>,
    current_player_index:i8
}

enum WizardState {
    INIT,
    PLAYING,
    END
}

struct Wizard {
    state:WizardState,
    round_count:i8,
    round_index:i8,
    player_count:i8,
    player_index:i8,
    players:Vec<Player>,
    rounds:Vec<Round>
}

impl Player {
    fn new(name: String) -> Player {
        Player { name, points: 0 }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.points)
    }
}

impl Tips {
    fn new() -> Tips {
        Tips { tips: HashMap::new() }
    }

    fn add_tip(&mut self, player:&Player, tip:i8) {
        self.tips.insert(player.name.to_string(), tip);
    }

    fn get_tip(&self, player:&Player) -> i8 {
        match self.tips.get(&player.name) {
            Some(tip) => *tip,
            None => 0
        }
    }
}

impl Round {
    fn new(round_nr:i8, players:Vec<Rc<RefCell<Player>>>) -> Round {
        Round {round_nr, state:RoundState::TIPPING, tips:Tips::new(), matches:Tips::new(), players, current_player_index:0}
    }

    fn play(&mut self, input_callback: fn(&RefCell<Player>) -> i8) {
        let current_player_index = self.current_player_index as usize;
        if self.state == RoundState::TIPPING || self.state == RoundState::RETIPPING {
            let current_player:&RefCell<Player> = &self.players[current_player_index];
            self.tips.add_tip(&current_player.borrow(), input_callback(current_player));

            if self.current_player_index == (self.players.len() - 1) as i8 {
                let sum_of_tips:i8 = self.tips.tips.values().sum(); // TODO: is there an easier way?
                self.state = if sum_of_tips == self.round_nr {
                     RoundState::RETIPPING
                } else {
                     RoundState::PLAYING
                };

                self.current_player_index = 0;
            } else {
                self.current_player_index += 1;
            }
        } else if self.state == RoundState::PLAYING {
            let current_player:&RefCell<Player> = &self.players[current_player_index];
            self.matches.add_tip(&current_player.borrow(), input_callback(current_player));

            if self.current_player_index == (self.players.len() - 1) as i8 {
                let sum_of_matches:i8 = self.matches.tips.values().sum(); // TODO: is there an easier way?
                if sum_of_matches == self.round_nr {
                    self.state = RoundState::CHECKING;
                }

                self.current_player_index = 0;
            } else {
                self.current_player_index += 1;
            }
        } else if self.state == RoundState::CHECKING {
            for player in &mut self.players {
                let p: &RefCell<Player> = player.borrow_mut();

                let tip = self.tips.get_tip(&p.borrow());
                let matched = self.matches.get_tip(&p.borrow());
                let diff = (tip - matched).abs();

                if diff == 0 {
                    p.borrow_mut().points += (20 + tip * 10) as i16;
                } else {
                    p.borrow_mut().points += (diff * -10) as i16;
                };

            }

            self.state = RoundState::END;
        }
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for player in &self.players {
            let p:&RefCell<Player> = player;
            write!(f, "{}\n", p.borrow()).expect("AHHHHHAHAHAHAHAHA");
        }

        write!(f, "{:?}", self.state)
    }
}

fn main() {
    let player1 = Rc::new(RefCell::new(Player::new("Player 1".to_string())));
    let player2 = Rc::new(RefCell::new(Player::new("Player 2".to_string())));
    let players = vec![player1, player2];

    let mut round = Round::new(1, players.to_vec());
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
