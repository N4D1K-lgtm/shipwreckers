use bevy::prelude::Component;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};

#[cfg(debug_assertions)]
use colored::Colorize;

/// Enum describing a Minesweeper tile
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    /// Is a bomb
    Bomb,
    /// Is a bomb neighbor
    BombNeighbor(u8),
    /// Empty tile
    Empty,
}

impl Tile {
    /// Is the tile a bomb?
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }

    #[cfg(debug_assertions)]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbor(v) => match v {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => v.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

// We want to be able to make coordinates sums..
impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        let x = ((self.x as i16) + x as i16) as u16;
        let y = ((self.y as i16) + y as i16) as u16;
        Self { x, y }
    }
}

// ..and subtractions
impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
