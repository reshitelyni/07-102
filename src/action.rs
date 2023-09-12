/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : action.rs
*   Last Modified : 2023-09-12 18:09
*   Describe      : Actions.
*
* ====================================================*/
use crate::flags;
use crate::crud;
use crate::sql3drv;
use crate::util;

pub struct Actor{
    srv: crud::CrudSvr,
    args: flags::FieldArgs
}

impl Actor{
    pub fn new(args: flags::ArcArgs) -> Actor{
        let conn = sql3drv::Sql3Connection::open_path(format!("{}/arccal.sql3", args.prefix).as_str()).expect("open database failed");
        let srv = crud::CrudSvr::new(conn);
        let args = args.field;
        Actor{srv, args}
    }

    pub fn run(&mut self){
        match &self.args{
            flags::FieldArgs::Song(args) => self.run_song(args.clone()),
            flags::FieldArgs::Score(args) => self.run_score(args.clone())
        }
    }
}

impl Actor{
    fn match_song(&mut self, name: String, pack: Option<String>, level: flags::SongLevel) -> u32{
        let song = self.srv.query_song(
            Some(name.clone()),
            pack,
            Some(util::level_to_str(&level).to_owned()),
            None,
            None
        );
        match song {
            Ok(song) => {
                if song.len() >= 2 {
                    song.iter()
                        .for_each(|x| println!("{x}"));
                    panic!("ambiguous songs");
                } else if song.len() == 0 {
                    let song = self.srv.query_alias(name).expect("no such song");
                    if song.len() >= 2{
                        song.iter()
                            .for_each(|x| println!("song #{x}"));
                        panic!("ambiguous songs");
                    }
                    *song.get(0).expect("no such song")
                } else {
                    song.get(0).expect("it shoulb be valid").id
                }
            },
            Err(_) => panic!("no such song")
        }
    }
}

impl Actor{
    fn run_song(&mut self, args: flags::SongArgs){
        match args.action{
            flags::SongArgsAction::Ls => {
                let songs = self.srv.query_song(None, None, None, None, None).expect("query song failed");
                songs.iter()
                     .for_each(|x| println!("{x}"));
            },
            flags::SongArgsAction::Add { name, pack, level, constant } => {
                assert_eq!(
                    self.srv.add_song(
                        name.as_str(),
                        pack.as_str(),
                        util::level_to_str(&level),
                        constant
                    ).expect("add song failed"),
                    1
                )
            },
            flags::SongArgsAction::Update { name, level, constant, pack, difficulty: _, plus: _} => {
                let id = self.match_song(name, pack, level);
                assert_eq!(
                    self.srv.update_song(id, None, None, None, constant).expect("update failed"),
                    1
                );
            },
            flags::SongArgsAction::Delete { name, level, pack } => {
                let id = self.match_song(name, pack, level);
                assert_eq!(
                    self.srv.delete_song(id).expect("delete song failed"),
                    1
                );
            },
            flags::SongArgsAction::Search { name, pack, level, constant, difficulty, plus } => {
                self.srv.query_song(
                    name,
                    pack,
                    level.map(|level| {
                        util::level_to_str(&level).to_owned()
                    }),
                    constant,
                    difficulty.map(|difficulty| {
                        (difficulty, plus)
                    })
                ).expect("no songs")
                 .iter()
                 .for_each(|x| println!("{x}"));
            },
            flags::SongArgsAction::Alias { origin, alias, pack, delete : _} => {
                let id = self.srv.query_song(Some(origin), pack, None, None, None).expect("no such song");
                if id.len() >= 2{
                    panic!("ambiguous songs");
                }
                let id = id.get(0).expect("no such song").id;
                assert_eq!(self.srv.insert_alias(id, alias).expect("add alias failed"), 1);
            }
        }
    }

    fn run_score(&mut self, args: flags::ScoreArgs){
        match args.action{
            flags::ScoreArgsAction::Ls { limit, sort: _, reverse } => {
                self.srv.query_score(limit.unwrap_or(30) as usize, reverse)
                        .expect("query failed")
                        .iter()
                        .for_each(|x| println!("{x}"));
            },
            flags::ScoreArgsAction::Add { name, pack, level, score } => {
                let id = self.match_song(name, pack, level);
                assert_eq!(
                    self.srv.add_score(id, score).expect("it should be valid"),
                    1
                );
            },
            flags::ScoreArgsAction::Delete { song, sc, clear } => {
                if clear{
                    self.srv.clear_score().expect("score clear failed");
                } else{
                    let id = self.srv.query_scoreid(song, sc);
                    match id {
                        Err(_) => {},
                        Ok(id) => {
                            match id {
                                None => {},
                                Some(id) => assert_eq!(
                                    self.srv.delete_score(id).expect("delete score failed"),
                                    1
                                )
                            }
                        }
                    }
                }
            },
            flags::ScoreArgsAction::Potential { potential } => {
                match potential{
                    Some(ptt) => assert_eq!(self.srv.update_ptt(ptt).expect("update failed"), 1),
                    None => {
                        let user = self.srv.query_user().expect("query failed");
                        println!("user ptt: {}", user.ptt);
                    }
                }
            },
            flags::ScoreArgsAction::B30 => {
                let user = self.srv.query_user().expect("user not found");
                println!("user b30: {}", user.b30);
            },
            flags::ScoreArgsAction::R10 => {
                let user = self.srv.query_user().expect("user not found");
                println!("user r10(estimated): {}", user.r10);
            }
        }
    }
}
