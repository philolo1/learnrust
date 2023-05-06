use std::{fs, str::FromStr};
use anyhow::{anyhow, Error, Result, Context};
use std::env;

#[derive(Debug, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum GameResult {
    Lose,
    Draw,
    Win,
}

#[derive(Debug)]
struct Game {
    player1: Play,
    player2: Play
}

#[derive(Debug)]
struct NewGame {
    player1: Play,
    player2: GameResult
}

impl FromStr for Play {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let player1 = match s {
            "A" | "X"  => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => return Err(anyhow!("Invalid play: {}", s)),
        };

        return Ok(player1)
    }

}

impl FromStr for GameResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let res = match s {
            "X"  => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => return Err(anyhow!("Invalid play: {}", s)),
        };

        return Ok(res)
    }
}

impl Play {
    fn score(&self) -> i32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors  => 3
        }
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s.split_once(' ').context("Needs space seperated")?;

        let player1 = str::parse(left)?;
        let player2 = str::parse(right)?;

        return Ok(Game{
            player1,
            player2
        })
    }
}

impl FromStr for NewGame {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s.split_once(' ').context("Needs space seperated")?;

        let player1 = str::parse(left)?;
        let player2 = str::parse(right)?;

        return Ok(NewGame{
            player1,
            player2
        })
    }
}

impl Game {
    fn score(&self) -> i32 {
        let mut my_score = match (self.player1, self.player2) {
            (Play::Rock, Play::Rock) => 3,
            (Play::Rock, Play::Paper) => 6,
            (Play::Rock, Play::Scissors) => 0,
            (Play::Paper, Play::Rock) => 0,
            (Play::Paper, Play::Paper) => 3,
            (Play::Paper, Play::Scissors) => 6,
            (Play::Scissors, Play::Rock) => 6,
            (Play::Scissors, Play::Paper) => 0,
            (Play::Scissors, Play::Scissors) => 3,
        };

        my_score += self.player2.score();

        return my_score;
    }
}

impl From<NewGame> for Game {
    fn from(value: NewGame) -> Self {
        let player1 = value.player1;
        let player2 = match (value.player1, value.player2) {
            (Play::Rock, GameResult::Lose) => Play::Scissors,
            (Play::Rock, GameResult::Draw) => Play::Rock,
            (Play::Rock, GameResult::Win) => Play::Paper,
            (Play::Paper, GameResult::Lose) => Play::Rock,
            (Play::Paper, GameResult::Draw) => Play::Paper,
            (Play::Paper, GameResult::Win) => Play::Scissors,
            (Play::Scissors, GameResult::Lose) => Play::Paper,
            (Play::Scissors, GameResult::Draw) => Play::Scissors,
            (Play::Scissors, GameResult::Win) => Play::Rock,
        };

        return Game { player1, player2 };
    }
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).context("one file is necessary")?;
    println!("file_name: {:?}", file_name);

    let content = fs::read_to_string(file_name)?;

    let games: Vec<Game> = content.lines()
        .flat_map(str::parse)
        .map(|x: NewGame| Game::from(x))
        .collect();

    let scores: Vec<i32> = games.iter().map(|x| x.score()).collect();

    println!("Scores : {:?}", scores);

    let sum: i32 = scores.iter().sum();
    println!("Final score : {:?}", sum);

    return Ok(());
}
