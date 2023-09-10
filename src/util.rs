/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : util.rs
*   Last Modified : 2023-09-10 11:20
*   Describe      : Common Utilities.
*
* ====================================================*/
use std::cmp::Ordering;

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
