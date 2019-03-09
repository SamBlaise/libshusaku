#![feature(custom_attribute)]
#![feature(specialization)]

extern crate pyo3;

use pyo3::prelude::*;
use goban::rules::game::Game;
use goban::rules::game::GobanSizes;
use goban::rules::game::Move;
use goban::rules::EndGame;
use goban::rules::Rule;
use goban::rules::Player;
use goban::pieces::util::coord::{Coord, Order};
use goban::pieces::goban::Goban;

#[pymodule]
pub fn libshusaku(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IGoban>()?;
    m.add_class::<IGame>()?;
    Ok(())
}

#[pyclass(name = Goban)]
pub struct IGoban {
    goban: Goban
}

#[pymethods]
impl IGoban {
    #[new]
    pub fn __new__(obj: &PyRawObject, arr: Vec<u8>) {
        obj.init({
            IGoban { goban: Goban::from_array(&arr, Order::RowMajor) }
        });
    }

    pub fn raw(&self) -> PyResult<Vec<u8>> { Ok(self.goban.tab().clone()) }

    pub fn raw_split(&self) -> PyResult<(Vec<bool>, Vec<bool>)>
    {
        Ok((self.goban.b_stones().clone(), self.goban.w_stones().clone()))
    }

    pub fn pretty_string(&self) -> PyResult<String> {
        Ok(self.goban.pretty_string())
    }
}

#[pyclass]
pub struct IGame {
    game: Game,
}

#[pymethods]
impl IGame {
    #[new]
    ///
    /// By default the rule are chinese
    ///
    pub fn __new__(obj: &PyRawObject, size: usize) {
        obj.init({
            IGame { game: Game::new(GobanSizes::Custom(size), Rule::Chinese) }
        });
    }

    pub fn put_handicap(&mut self, coords: Vec<Coord>) -> PyResult<()> {
        Ok(self.game.put_handicap(&coords))
    }

    ///
    /// Get all the plays
    /// each element represents an vector.
    ///
    pub fn plays(&self) -> PyResult<Vec<Vec<u8>>> {
        Ok(self.game.plays()
            .into_iter()
            .map(|goban| goban.tab().clone()).collect())
    }

    ///
    /// Return the goban
    ///
    pub fn goban(&self) -> PyResult<IGoban> {
        Ok(IGoban { goban: self.game.goban().clone() })
    }

    ///
    /// Get the goban in a Vec<u8>
    ///
    pub fn raw_goban(&self) -> PyResult<Vec<u8>>
    {
        Ok(self.game.goban().tab().clone())
    }

    pub fn raw_goban_split(&self) -> PyResult<(Vec<bool>, Vec<bool>)> {
        Ok(
            (self.game.goban().b_stones().clone(), self.game.goban().w_stones().clone())
        )
    }

    ///
    /// Resume the game after to passes
    ///
    pub fn resume(&mut self) -> PyResult<()> {
        Ok(self.game.resume())
    }

    ///
    /// Get prisoners of the game.
    /// (black prisoners, white prisoners)
    ///
    pub fn prisoners(&self) -> PyResult<(u32, u32)> {
        Ok(self.game.prisoners())
    }

    ///
    /// Return the komi of the game
    ///
    pub fn komi(&self) -> PyResult<f32> {
        Ok
            (self.game.komi())
    }

    ///
    /// Set the komi
    ///
    pub fn set_komi(&mut self, komi: f32) -> PyResult<()> {
        self.game.set_komi(komi);
        Ok(())
    }
    ///
    /// Return true if the game is over
    ///
    pub fn over(&self) -> PyResult<bool> {
        Ok(self.game.over())
    }

    ///
    /// Returns the score
    /// (black score, white score)
    /// returns -1 if resign
    /// ex:
    /// (-1,0) Black resigned so white won
    /// (0,-1) White resigned so black won
    ///
    pub fn outcome(&self) -> PyResult<Option<(f32, f32)>> {
        Ok(match self.game.outcome() {
            None => None,
            Some(endgame) => match endgame {
                EndGame::Score(x, y) => Some((x, y)),
                EndGame::WinnerByResign(res) => match res {
                    // White win
                    Player::White => Some((-1., 0.)),
                    // Black win
                    Player::Black => Some((0., -1.)),
                }
            }
        })
    }

    /// Get the current turn
    pub fn turn(&self) -> bool {
        match self.game.turn() {
            Player::White => true,
            Player::Black => false
        }
    }

    ///
    /// Don't check if the play is legal.
    ///
    pub fn play(&mut self, play: Option<Coord>) -> PyResult<()> {
        Ok(match play {
            Some(mov) => self.game.play(&Move::Play(mov.0, mov.1)),
            None => self.game.play(&Move::Pass),
        })
    }

    /// Resign
    pub fn resign(&mut self) -> PyResult<()> {
        Ok(self.game.play(&Move::Resign))
    }

    /// All the legals
    pub fn legals(&self) -> PyResult<Vec<Coord>> {
        Ok(self.game.legals().collect())
    }

    pub fn pop(&mut self) -> PyResult<()> {
        self.game.pop();
        Ok(())
    }

    pub fn calculate_territories(&self) -> PyResult<(f32, f32)> {
        Ok(self.game.calculate_territories())
    }

    pub fn display(&self) -> PyResult<()> {
        self.game.display();
        Ok(())
    }
}
