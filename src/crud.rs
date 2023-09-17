/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : crud.rs
*   Last Modified : 2023-09-12 18:13
*   Describe      : CRUD execution.
*
* ====================================================*/

use crate::record;
use crate::sql3drv;
use crate::util;
use rusqlite::Result;

pub struct CrudSvr {
    conn: sql3drv::Sql3Connection,
}

impl CrudSvr {
    pub fn new(conn: sql3drv::Sql3Connection) -> CrudSvr {
        CrudSvr { conn }
    }
}

//user
impl CrudSvr {
    pub fn query_b30(&mut self) -> Result<f32> {
        let _ = self.conn.start_transaction()?;
        let (b30, cached): (f32, i32) = self
            .conn
            .prepare("select b30, cached from user")?
            .query_row([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))?;
        if cached == 1 {
            return Ok(b30);
        }

        let mut b30 = self.conn
                      .prepare("select avg(song.constant), max(score.sc) from score inner join song on score.songid = song.id group by song.id")?
                      .query_map([], |row| {
                          let (constant, score) :(f32, i32) = (row.get(0).unwrap(), row.get(1).unwrap());
                          Ok(util::eval_song_ptt(constant, score))
                      })?
                      .map(|x| x.expect("should always be ok"))
                      .collect::<Vec<f32>>();
        b30.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let b30 = b30.iter().take(30).sum::<f32>() / 30.0;

        let ptt: f32 = self
            .conn
            .prepare("select ptt from user")?
            .query_row([], |row| row.get(0))?;
        let r10 = (ptt * 40.0 - b30 * 30.0) / 10.0;

        self.conn.execute(
            format!("update user set b30 = {b30}, r10 = {r10}, cached = 1 where ptt = {ptt}")
                .as_str(),
        )?;

        Ok(b30)
    }

    pub fn query_score(&mut self, limit: usize, reverse: bool) -> Result<Vec<record::SongRank>> {
        let _ = self.conn.start_transaction()?;
        let mut all_score = self.conn
                                .prepare("select avg(song.constant), song.name, song.pack, song.level, max(score.sc) from song inner join score on song.id = score.songid group by song.name, song.pack, song.level")?
                                .query_map([], |row|{
                                    Ok(record::SongRank{
                                            constant: row.get(0).unwrap(),
                                            name: row.get(1).unwrap(),
                                            pack: row.get(2).unwrap(),
                                            level: row.get(3).unwrap(),
                                            best: util::eval_song_ptt(row.get(0).unwrap(), row.get(4).unwrap())
                                    })
                                })?
                                .map(|x| Some(x.expect("it should be valid")))
                                .collect::<Vec<Option<record::SongRank>>>();
        all_score.sort_by(|a, b| {
            let order = a
                .as_ref()
                .unwrap()
                .best
                .partial_cmp(&b.as_ref().unwrap().best)
                .unwrap();
            //default is ascent
            match reverse {
                true => order,
                false => order.reverse(),
            }
        });

        Ok(all_score
            .iter_mut()
            .take(limit)
            .map(|x| x.take().unwrap())
            .collect())
    }

    pub fn query_user(&mut self) -> Result<record::User> {
        let b30 = self.query_b30()?;
        let (r10, ptt) = self
            .conn
            .prepare("select r10, ptt from user")?
            .query_row([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))?;
        Ok(record::User { b30, r10, ptt })
    }

    pub fn delete_score(&mut self, id: u32) -> Result<usize> {
        let _ = self.conn.start_transaction()?;
        let result = self
            .conn
            .execute(format!("delete from score where id = {id}").as_str());
        self.conn.execute("update user set cached = 0")?;
        result
    }

    pub fn clear_score(&mut self) -> Result<usize> {
        let _ = self.conn.start_transaction()?;
        let result = self.conn.execute("delete from score");
        self.conn.execute("update user set cached = 0")?;
        result
    }

    pub fn add_score(&mut self, songid: u32, score: u32) -> Result<usize> {
        let _ = self.conn.start_transaction()?;
        self.conn.execute("update user set cached = 0")?;
        self.conn
            .execute(format!("insert into score(songid, sc) values({songid}, {score})").as_str())
    }

    pub fn update_score(&mut self, id: u32, songid: u32, sc: u32) -> Result<usize> {
        let _ = self.conn.start_transaction()?;
        self.conn.execute("update user set cached = 0")?;
        self.conn.execute(
            format!("update score set songid = {songid}, sc = {sc} where id = {id}").as_str(),
        )
    }

    pub fn query_scoreid(&self, songid: u32, sc: u32) -> Result<Option<u32>> {
        Ok(self
            .conn
            .prepare(
                format!("select id from score where songid = {songid} and sc = {sc}").as_str(),
            )?
            .query_map([], |row| row.get(0))?
            .map(|x| x.expect("it should be valid"))
            .collect::<Vec<u32>>()
            .get(0)
            .take()
            .copied())
    }

    pub fn update_ptt(&mut self, ptt: f32) -> Result<usize> {
        let _ = self.conn.start_transaction()?;
        self.conn
            .execute(format!("update user set cached = 0, ptt = {ptt}").as_str())
    }
}

impl CrudSvr {
    //add new song do not interrupt current b30
    pub fn add_song(&self, name: &str, pack: &str, level: &str, constant: f32) -> Result<usize> {
        self.conn
            .execute(format!("insert into song(name, pack, level, constant) values(\'{name}\', \'{pack}\', \'{level}\', {constant})").as_str())
    }

    //query itself is safe to cache
    pub fn query_song(
        &self,
        name: Option<String>,
        pack: Option<String>,
        level: Option<String>,
        constant: Option<f32>,
        difficulty: Option<record::SongDifficulty>,
    ) -> Result<Vec<record::Song>> {
        Ok(
            self.conn
                .prepare(
                    format!("select id, name, pack, level, constant from song where name like \'{}\' and pack like \'{}\' {} {} {}",
                                match name {Some(name) => format!("%{name}%"), None => "%".to_owned()},
                                match pack {Some(pack) => format!("%{pack}%"), None => "%".to_owned()},
                                match level {Some(level) => format!("and level = \'{level}\'"), None => String::new()},
                                match constant {Some(constant) => format!("and constant = {constant}"), None => String::new()},
                                match difficulty {Some(difficulty) => format!("and constant <= {} and constant >= {}",
                                                                                util::diff_to_constant_range(&difficulty).1,
                                                                                util::diff_to_constant_range(&difficulty).0),
                                                  None => String::new()}
                            ).as_str()
                 )?
                .query_map([], |row|{
                    Ok(record::Song{
                        id: row.get(0).unwrap(),
                        name: row.get(1).unwrap(),
                        pack: row.get(2).unwrap(),
                        level: row.get(3).unwrap(),
                        constant: row.get(4).unwrap()
                    })
                 })?
                .map(|x| x.expect("it should be valid"))
                .collect()
        )
    }

    //deleting song is safe only if deletion is successful.
    pub fn delete_song(&self, id: u32) -> Result<usize> {
        self.conn
            .execute(format!("delete from song where id = {id}").as_str())
    }

    pub fn clear_song(&self) -> Result<usize> {
        self.conn.execute("delete from song")
    }

    //updating is likely to invalidate cache.
    pub fn update_song(
        &mut self,
        id: u32,
        name: Option<String>,
        pack: Option<String>,
        level: Option<String>,
        constant: Option<f32>,
    ) -> Result<usize> {
        let _ = self.conn.start_transaction();
        self.conn.execute("update user set cached = 0")?;
        self.conn.execute(
            format!(
                "update song set id={id}{}{}{}{} where id = {id}",
                match name {
                    Some(name) => format!(",name=\'{name}\'"),
                    None => String::new(),
                },
                match pack {
                    Some(pack) => format!(",pack=\'{pack}\'"),
                    None => String::new(),
                },
                match level {
                    Some(level) => format!(",level=\'{level}\'"),
                    None => String::new(),
                },
                match constant {
                    Some(constant) => format!(",constant={constant}"),
                    None => String::new(),
                }
            )
            .as_str(),
        )
    }
}

impl CrudSvr {
    pub fn insert_alias(&self, id: u32, alias: String) -> Result<usize> {
        self.conn
            .execute(format!("insert into alias values({id}, \'{alias}\')").as_str())
    }

    pub fn query_alias(&self, alias: String) -> Result<Vec<u32>> {
        Ok(self
            .conn
            .prepare(format!("select songid from alias where alname = \'{alias}\'").as_str())?
            .query_map([], |row| row.get(0))?
            .map(|x| x.expect("it should be valid"))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_db() -> CrudSvr {
        let mut srv = CrudSvr::new(sql3drv::Sql3Connection::open_memory().unwrap());
        assert_eq!(srv.add_song("s1", "p1", "past", 4.0).unwrap(), 1);
        assert_eq!(srv.add_song("s1", "p2", "past", 3.0).unwrap(), 1);
        assert_eq!(srv.add_song("s2", "p1", "past", 4.0).unwrap(), 1);
        assert_eq!(srv.update_song(3, None, None, None, Some(5.0)).unwrap(), 1);
        srv
    }

    #[test]
    fn test_query() {
        let mut srv = fake_db();
        assert_eq!(srv.query_b30().unwrap(), 0.0);

        assert_eq!(srv.add_score(1, 9500000).unwrap(), 1);

        assert_eq!(srv.query_b30().unwrap(), 4.0 / 30.0);
    }

    #[test]
    fn test_query_best() {
        let mut srv = fake_db();
        //ptt 4.0
        assert_eq!(srv.add_score(1, 9500000).unwrap(), 1);
        //ptt 2.0
        assert_eq!(srv.add_score(2, 9200000).unwrap(), 1);
        //ptt 6.0
        assert_eq!(srv.add_score(3, 9800000).unwrap(), 1);
        //multiple
        assert_eq!(srv.add_score(1, 9500000).unwrap(), 1);

        assert_eq!(
            srv.query_score(1, false).unwrap(),
            Vec::from([record::SongRank {
                name: "s2".to_owned(),
                pack: "p1".to_owned(),
                level: "past".to_owned(),
                constant: 5.0,
                best: 6.0
            }])
        );
        assert_eq!(
            srv.query_score(1, true).unwrap(),
            Vec::from([record::SongRank {
                name: "s1".to_owned(),
                pack: "p2".to_owned(),
                level: "past".to_owned(),
                constant: 3.0,
                best: 2.0
            }])
        );
    }

    #[test]
    fn test_query_user() {
        let mut srv = fake_db();

        assert_eq!(srv.add_score(1, 9500000).unwrap(), 1);

        let record::User { b30, r10, ptt } = srv.query_user().unwrap();
        assert_eq!(b30, 4.0 / 30.0);
        assert_eq!(ptt, 0.0);
        assert_eq!(r10, -0.4);
    }

    #[test]
    fn test_delete_score() {
        let mut srv = fake_db();

        assert_eq!(srv.add_score(1, 9200000).unwrap(), 1);
        assert_eq!(srv.update_score(1, 1, 9500000).unwrap(), 1);

        assert!(srv.query_user().is_ok());

        assert_eq!(srv.clear_score().unwrap(), 1);

        let record::User { b30, r10, ptt } = srv.query_user().unwrap();

        assert_eq!(b30, 0.0);
        assert_eq!(r10, 0.0);
        assert_eq!(ptt, 0.0);

        let mut srv = fake_db();

        assert_eq!(srv.add_score(1, 9500000).unwrap(), 1);

        assert!(srv.query_user().is_ok());
        assert_eq!(srv.delete_score(1).unwrap(), 1);
        let record::User { b30, r10, ptt } = srv.query_user().unwrap();

        assert_eq!(b30, 0.0);
        assert_eq!(r10, 0.0);
        assert_eq!(ptt, 0.0);
    }

    #[test]
    fn test_query_score_id() {
        let mut srv = fake_db();
        assert_eq!(srv.add_score(1, 9200000).unwrap(), 1);
        assert_eq!(srv.query_scoreid(1, 9200000).unwrap().unwrap(), 1);
    }

    #[test]
    fn test_query_song() {
        let mut srv = fake_db();
        assert_eq!(
            srv.query_song(
                Some(String::from("s1")),
                None,
                Some(String::from("past")),
                None,
                None
            )
            .unwrap()
            .len(),
            2
        );
    }
}
