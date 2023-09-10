/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : sql3drv.rs
*   Last Modified : 2023-09-10 11:08
*   Describe      : Database interface
*
* ====================================================*/
use rusqlite;
use rusqlite::Result;

/*
 * Table Structure
 * 1. User table(Metatable)
 *   id int
 *   b30 float
 *   r10 float
 *   ptt float
 *   cached float
 * 2. Song table
 *   id int(index)
 *   name varchar(origin)
 *   pack varchar
 *   level varchar(pst, prs, ftr, byd)
 *   constant float
 * 3. Score table
 *   id int(index)
 *   songid int
 *   sc int
 * 4. Alias table
 *   songid int
 *   alias varchar(index+)
 */

pub struct Sql3Connection{
    conn: rusqlite::Connection
}

impl Sql3Connection{
    pub fn open_memory() -> Result<Sql3Connection>{
        let conn = rusqlite::Connection::open_in_memory()?;
        Self::init_table(&conn)?;
        Ok(Sql3Connection{conn})
    }

    pub fn open_path(path: &str) -> Result<Sql3Connection>{
        let conn = rusqlite::Connection::open(path)?;
        Self::init_table(&conn)?;
        Ok(Sql3Connection{conn})
    }

    fn init_table(conn: &rusqlite::Connection) -> Result<()>{
        if let Ok(_) = conn.query_row("select id from user", [], |_| Ok(())){
            return Ok(())
        }
        conn.execute("PRAGMA foreign_keys=ON", ())?;
        conn.execute("create table user(id integer primary key autoincrement, b30 float(5, 5), r10 float(5, 5), ptt float(5, 5), cached int)", ())?;
        conn.execute("create table song(id integer primary key autoincrement, name varchar(100), pack varchar(100), level varchar(10), constant float(5, 5))", ())?;
        conn.execute("create table score(id integer primary key autoincrement, songid int, sc int, foreign key(songid) references song(id))", ())?;
        conn.execute("create table alias(songid int, alname varchar(100), primary key(songid, alname), foreign key(songid) references song(id))", ())?;
        conn.execute("insert into user(b30, r10, ptt, cached) values(0.0, 0.0, 0.0, 1)", ())?;
        Ok(())
    }

    pub fn start_transaction(&mut self) -> Result<rusqlite::Transaction<'_>>{
        let mut transaction = self.conn.transaction()?;
        transaction.set_drop_behavior(rusqlite::DropBehavior::Commit);
        Ok(transaction)
    }

    pub fn prepare(&self, sql: &str) -> Result<rusqlite::CachedStatement<'_>>{
        self.conn.prepare_cached(sql)
    }

    pub fn execute(&self, sql: &str) -> Result<usize>{
        self.conn.execute(sql, ())
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_init() -> Result<()>{
        assert_eq!(Sql3Connection::open_memory()?
                    .prepare("select * from user")?
                    .query_map([], |_| Ok(1))?
                    .count()
                   , 1);
        Ok(())
    }
}
