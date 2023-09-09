/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : crud.rs
*   Last Modified : 2023-09-09 16:46
*   Describe      : CRUD execution.
*
* ====================================================*/

use crate::sql3drv;
use rusqlite::Result;

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
                          if score >= 9800000 {
                              return Ok(constant + 1.0 + (score - 9800000) as f32 /200000.0);
                          }
                          Ok(constant + (score - 9500000) as f32 / 300000.0)
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
}

#[cfg(test)]
mod tests{
    use super::*;

    fn fake_db() -> CrudSvr {
        let mut conn = sql3drv::Sql3Connection::open_memory().unwrap();
        conn.execute("insert into song values(0, \'s1\', \'p1\', \'past\', 4.0)").unwrap();
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
}
