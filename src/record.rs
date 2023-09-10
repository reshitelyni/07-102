/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : record.rs
*   Last Modified : 2023-09-10 13:17
*   Describe      : Record struct type.
*
* ====================================================*/
use std::cmp::Eq;
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
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

impl PartialEq for SongRank{
    fn eq(&self, other: &Self) -> bool{
        (&self.name, &self.pack, &self.level, self.constant, self.best)
            == (&other.name, &other.pack, &other.level, other.constant, other.best)
    }
}
impl Eq for SongRank{}

impl Eq for User{}
