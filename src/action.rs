/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : action.rs
*   Last Modified : 2023-09-12 16:17
*   Describe      : Actions.
*
* ====================================================*/
use crate::flags;
use crate::crud;
use crate::sql3drv;

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
    fn run_song(&mut self, args: flags::SongArgs){
        match args.action{
            flags::SongArgsAction::Ls => {

            },
            flags::SongArgsAction::Add { name, pack, level, constant } => {

            },
            flags::SongArgsAction::Update { name, level, constant, pack, difficulty, plus } => {

            },
            flags::SongArgsAction::Delete { name, level, pack } => {

            },
            flags::SongArgsAction::Search { name, pack, level, constant, difficulty, plus } => {

            },
            flags::SongArgsAction::Alias { origin, alias, pack, delete } => {

            }
        }
    }

    fn run_score(&mut self, args: flags::ScoreArgs){
        match args.action{
            flags::ScoreArgsAction::Ls { limit, sort: _, reverse } => {
                self.srv.query_score(limit.unwrap_or(30) as usize, reverse)
                        .expect("query failed")
                        .iter()
                        .for_each(|x| {
                            println!("{}/{} {} {} {}",
                                x.name,
                                x.pack,
                                x.level,
                                x.constant,
                                x.best
                            );
                        });
            },
            flags::ScoreArgsAction::Add { name, pack, level, score } => {
                let song =
                    self.srv.query_song(
                    Some(name.clone()),
                    pack,
                    match level{
                        flags::SongLevel::Past => Some("past".to_owned()),
                        flags::SongLevel::Present => Some("present".to_owned()),
                        flags::SongLevel::Future => Some("future".to_owned()),
                        flags::SongLevel::Beyond => Some("beyond".to_owned())
                    },
                    None,
                    None);
                if let Ok(song) = song {
                    if song.len() >= 2 {
                        song.iter()
                            .for_each(|x| {
                                println!("{}/{} {}", x.name, x.pack, x.level);
                            });
                        panic!("ambiguous songs");
                    }
                    assert_eq!(
                        self.srv.add_score(
                            song.get(0).expect("it should be valid").id,
                            score
                        ).expect("add failed")
                        , 1
                    );
                } else {
                    //try alias
                    let id = self.srv.query_alias(name).expect("no such song");
                    if id.len() >= 2 {
                        id.iter()
                          .for_each(|x| {
                              println!("song #{x}");
                          });
                        panic!("ambiguous songs");
                    }
                    assert_eq!(
                        self.srv.add_score(
                            *id.get(0).expect("no such song"),
                            score
                        ).expect("add failed")
                        , 1
                    );
                }
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
                                    self.srv.delete_score(id).unwrap(),
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
