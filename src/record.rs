/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : record.rs
*   Last Modified : 2023-09-10 15:38
*   Describe      : Record struct type.
*
* ====================================================*/
use std::cmp::Eq;
use std::cmp::PartialEq;

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

impl Eq for SongRank{}

impl Eq for User{}
