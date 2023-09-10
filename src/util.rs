/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : util.rs
*   Last Modified : 2023-09-10 15:52
*   Describe      : Common Utilities.
*
* ====================================================*/
use std::cmp::Ordering;
use crate::record::SongDifficulty;
use crate::record::SongConstantRange;

pub fn eval_song_ptt(constant: f32, score: i32) -> f32{
    match score.cmp(&9800000){
        Ordering::Equal | Ordering::Greater => {
            constant + 1.0 + (score - 9800000) as f32 / 200000.0
        },
        Ordering::Less => {
            constant + (score - 9500000) as f32 / 300000.0
        }
    }
}

pub fn diff_to_constant_range(diff: &SongDifficulty) -> SongConstantRange{
    let (major, plus) = diff.clone();
    if major <= 8 || major >= 11{
        return (major as f32, major as f32 + 0.9);
    }else if plus{
        return (major as f32 + 0.7, major as f32 + 0.9);
    }else {
        return (major as f32, major as f32 + 0.6);
    }
}

pub fn constant_to_diff(constant: f32) -> SongDifficulty{
    if constant < 9.0 || constant >= 11.0 {
        return (constant as u32, false);
    } else if constant - (constant as u32) as f32 > 0.6{
        return (constant as u32, true);
    } else{
        return (constant as u32, false);
    }
}
