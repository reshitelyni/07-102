/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : record.rs
*   Last Modified : 2023-09-12 18:05
*   Describe      : Record struct type.
*
* ====================================================*/
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub struct SongRank{
    pub name: String,
    pub pack: String,
    pub level: String,
    pub constant: f32,
    pub best: f32
}

#[derive(Debug, Clone, PartialEq)]
pub struct User{
    pub b30: f32,
    pub r10: f32,
    pub ptt: f32
}

#[derive(Debug, Clone, PartialEq)]
pub struct Song{
    pub id: u32,
    pub name: String,
    pub pack: String,
    pub level: String,
    pub constant: f32
}

pub type SongDifficulty = (u32, bool);
pub type SongConstantRange = (f32, f32);

impl Eq for SongRank{}

impl Eq for User{}

impl Eq for Song{}

impl Display for SongRank{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>{
        write!(
            f,
            "{}/{} {} {} {}",
            self.name,
            self.pack,
            self.level,
            self.constant,
            self.best
        )
    }
}

impl Display for Song{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>{
        write!(
            f,
             "#{}: {}/{} {} {}",
             self.id,
             self.name,
             self.pack,
             self.level,
             self.constant
        )
    }
}
