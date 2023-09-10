/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : crud.rs
*   Last Modified : 2023-09-10 13:26
*   Describe      : CRUD execution.
*
* ====================================================*/

use crate::sql3drv;
use rusqlite::Result;
use crate::record;
use crate::util;

pub struct CrudSvr{
    conn: sql3drv::Sql3Connection
}

impl CrudSvr{
    pub fn new(conn: sql3drv::Sql3Connection) -> CrudSvr{
        CrudSvr{conn}
    }
}

//user
impl CrudSvr{
    pub fn query_b30(&mut self) -> Result<f32>{
        let _ = self.conn.start_transaction()?;
        let (b30, cached) :(f32, i32) = self.conn
                                            .prepare("select b30, cached from user")?
                                            .query_row([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))?;
        if cached == 1{
            return Ok(b30);
        }

        let mut b30 = self.conn
                      .prepare("select song.constant, score.sc from score inner join song on score.songid = song.id")?
                      .query_map([], |row| {
                          let (constant, score) :(f32, i32) = (row.get(0).unwrap(), row.get(1).unwrap());
                          Ok(util::eval_song_ptt(constant, score))
                      })?
                      .map(|x| x.expect("should always be ok"))
                      .collect::<Vec<f32>>();
        b30.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let b30 = b30.iter()
                     .take(30)
                     .sum::<f32>()
                     / 30.0;

        let ptt :f32 = self.conn
                      .prepare("select ptt from user")?
                      .query_row([], |row| {
                          row.get(0)
                      })?;
        let r10 = (ptt * 40.0 - b30 * 30.0) / 10.0;

        self.conn
            .execute(
                format!("update user set b30 = {b30}, r10 = {r10}, cached = 1 where ptt = {ptt}").as_str()
             )?;

        Ok(b30)
    }

    pub fn query_score(&mut self, limit: usize, reverse: bool) -> Result<Vec<record::SongRank>>{
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
            let order = a.as_ref().unwrap().best.partial_cmp(&b.as_ref().unwrap().best).unwrap();
            //default is ascent
            match reverse{
                true => order,
                false => order.reverse()
            }
        });

        Ok(all_score.iter_mut().take(limit).map(|x| x.take().unwrap()).collect())
    }

    pub fn query_user(&mut self) -> Result<record::User>{
        let b30 = self.query_b30()?;
        let (r10, ptt) = self.conn
                             .prepare("select r10, ptt from user")?
                             .query_row([], |row| {
                                 Ok((row.get(0).unwrap(), row.get(1).unwrap()))
                             })?;
        Ok(record::User{b30, r10, ptt})
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn fake_db() -> CrudSvr {
        let mut conn = sql3drv::Sql3Connection::open_memory().unwrap();
        conn.execute("insert into song values(0, \'s1\', \'p1\', \'past\', 4.0)").unwrap();
        conn.execute("insert into song values(1, \'s1\', \'p2\', \'past\', 3.0)").unwrap();
        conn.execute("insert into song values(2, \'s2\', \'p1\', \'past\', 5.0)").unwrap();
        CrudSvr::new(conn)
    }

    #[test]
    fn test_query() {
        let mut srv = fake_db();
        match srv.query_b30() {
            Err(_) => panic!("it should be valid"),
            Ok(b30) => assert_eq!(b30, 0.0)
        }

        assert_eq!(srv.conn.execute("insert into score(songid, sc) values(0, 9500000)").unwrap(), 1);
        assert_eq!(srv.conn.execute("update user set cached = 0").unwrap(), 1);

        match srv.query_b30() {
            Err(_) => panic!("it should be valid"),
            Ok(b30) => assert_eq!(b30, 4.0 / 30.0)
        }
    }

    #[test]
    fn test_query_best() {
        let mut srv = fake_db();
        //ptt 4.0
        assert_eq!(srv.conn.execute("insert into score(songid, sc) values(0, 9500000)").unwrap(), 1);
        //ptt 2.0
        assert_eq!(srv.conn.execute("insert into score(songid, sc) values(1, 9200000)").unwrap(), 1);
        //ptt 6.0
        assert_eq!(srv.conn.execute("insert into score(songid, sc) values(2, 9800000)").unwrap(), 1);

        assert_eq!(srv.query_score(1, false).unwrap(), Vec::from([record::SongRank{name: "s2".to_owned(), pack: "p1".to_owned(), level: "past".to_owned(), constant: 5.0, best: 6.0}]));
        assert_eq!(srv.query_score(1, true).unwrap(), Vec::from([record::SongRank{name: "s1".to_owned(), pack: "p2".to_owned(), level: "past".to_owned(), constant: 3.0, best: 2.0}]));
    }

    #[test]
    fn test_query_user() {
        let mut srv = fake_db();

        assert_eq!(srv.conn.execute("insert into score(songid, sc) values(0, 9500000)").unwrap(), 1);
        assert_eq!(srv.conn.execute("update user set cached = 0").unwrap(), 1);

        match srv.query_user() {
            Err(_) => panic!("it should be valid"),
            Ok(record::User{b30, r10, ptt}) => {
                assert_eq!(b30, 4.0 / 30.0);
                assert_eq!(ptt, 0.0);
                assert_eq!(r10, -0.4);
            }
        }
    }
}
