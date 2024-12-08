#![allow(dead_code)]

use std::path::Path;
use std::{fs, isize};

#[derive(Debug, Clone)]
pub struct CharBlock {
    block: Vec<Vec<char>>,
}

impl CharBlock {
    /// Creates a new CharBlock from a 2D vector of characters.
    pub fn new(block: Vec<Vec<char>>) -> Self {
        Self { block }
    }

    /// Reads a character from the block at (i, j), where `i` is the column and `j` is the row.
    pub fn get(&self, i: usize, j: usize) -> Option<char> {
        self.block.get(j).and_then(|row| row.get(i).copied())
    }

    /// Writes a character to the block at (i, j), where `i` is the column and `j` is the row.
    pub fn set(&mut self, i: usize, j: usize, value: char) {
        if let Some(row) = self.block.get_mut(j) {
            if i < row.len() {
                row[i] = value;
            }
        }
    }

    /// Iterates over the characters row by row.
    pub fn iter_rows(&self) -> impl Iterator<Item = &Vec<char>> {
        self.block.iter()
    }

    /// Iterates over all characters in the block, row by row.
    pub fn iter_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.block.iter().flat_map(|row| row.iter().copied())
    }

    /// Reads a CharBlock from a string, splitting rows by newlines.
    pub fn from_string(input: &str) -> Self {
        let block = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self::new(block)
    }

    /// Reads a CharBlock from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Ok(Self::from_string(&content))
    }

    /// Converts the CharBlock back into a string representation.
    pub fn to_string(&self) -> String {
        self.block
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Gets the dimensions of the block as (columns, rows).
    pub fn dimensions(&self) -> (usize, usize) {
        let rows = self.block.len();
        let columns = self.block.get(0).map_or(0, |row| row.len());
        (columns, rows)
    }

    pub fn is_inside(&self, i: isize, j: isize) -> bool {
        let dim = self.dimensions();
        (i >= 0) && (i < dim.0 as isize) && (j >= 0) && (j < dim.1 as isize)
    }
}
