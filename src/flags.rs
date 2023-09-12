/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : flags.rs
*   Last Modified : 2023-09-12 15:36
*   Describe      : Flags parser.
*
* ====================================================*/
use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser)]
#[command(author = "6607changchun", version = "0.0.1", about = "arccal", long_about = None)]
pub struct ArcArgs{
    #[command(subcommand)]
    pub field: FieldArgs,
    #[arg(short = 'p', long = "prefix", default_value_t = String::from("."))]
    pub prefix: String,
}

#[derive(Subcommand)]
pub enum FieldArgs{
    Song(SongArgs),
    Score(ScoreArgs),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SongLevel{
    Past,
    Present,
    Future,
    Beyond
}

#[derive(Args, Clone)]
pub struct SongArgs{
    #[command(subcommand)]
    pub action: SongArgsAction
}

#[derive(Subcommand, Clone)]
pub enum SongArgsAction{
    Ls,
    Add {name: String, pack: String, #[arg(value_enum)] level: SongLevel, constant: f32},
    Update {name: String, #[arg(value_enum)] level: SongLevel, constant: Option<f32>, pack: Option<String>, difficulty: Option<u32>, #[arg(short = 'p', long = "plus")] plus: bool},
    Delete {name: String, #[arg(value_enum)] level: SongLevel, pack: Option<String>},
    Search {name: Option<String>, pack: Option<String>, #[arg(value_enum)] level: Option<SongLevel>, constant: Option<f32>, difficulty: Option<u32>, #[arg(short = 'p', long = "plus")] plus: bool},
    Alias {origin: String, alias: String, pack: Option<String>, #[arg(short = 'd', long = "delete")] delete: bool}
}

#[derive(Args, Clone)]
pub struct ScoreArgs{
    #[command(subcommand)]
    pub action: ScoreArgsAction
}

#[derive(Subcommand, Clone)]
pub enum ScoreArgsAction{
    Ls {limit: Option<u32>, #[arg(short = 's', long = "sort")] sort: bool, #[arg(short = 'r', long = "reverse")] reverse: bool},
    Add {name: String, pack: Option<String>, #[arg(value_enum)] level: SongLevel, score: u32},
    Delete {song: u32, sc: u32, #[arg(short = 'a', long = "all")] clear: bool},
    Potential {potential: Option<f32>},
    B30,
    R10
}

impl ArcArgs{
    pub fn new() -> ArcArgs{
        ArcArgs::parse()
    }
}
