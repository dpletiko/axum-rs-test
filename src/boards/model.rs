use crate::{db, error_handler::AppError};
use crate::schema::boards;
use chrono::DateTime;
use chrono::offset::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = boards)]
pub struct Board {
    pub name: String,
    pub user_id: i32,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = boards)]
pub struct Boards {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Boards {
    pub fn all() -> Result<Vec<Self>, anyhow::Error> {
        let mut conn = db::connection()?;
        let boards = boards::table.load::<Boards>(&mut conn)?;
        Ok(boards)
    }

    pub fn find(id: i32) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let board = boards::table.filter(boards::id.eq(id)).first(&mut conn)?;
        Ok(board)
    }

    pub fn create(board: Board) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let board = Board::from(board);
        let board = diesel::insert_into(boards::table)
            .values(board)
            .get_result(&mut conn)?;
        Ok(board)
    }

    pub fn update(id: i32, board: Board) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let board = diesel::update(boards::table)
            .filter(boards::id.eq(id))
            .set(board)
            .get_result(&mut conn)?;
        Ok(board)
    }

    pub fn delete(id: i32) -> Result<usize, anyhow::Error> {
        let mut conn = db::connection()?;
        let res = diesel::delete(boards::table.filter(boards::id.eq(id))).execute(&mut conn)?;
        Ok(res)
    }
}

impl Board {
    fn from(board: Board) -> Board {
        Board {
            name: board.name,
            user_id: board.user_id,
            // created_at: board.created_at,
            // updated_at: board.updated_at
        }
    }
}
