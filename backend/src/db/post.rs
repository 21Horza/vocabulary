use super::{get_db_con, Result};
use crate::{error::Error::*, DBPool};
use common::*;
use mobc_postgres::tokio_postgres::Row;

pub const TABLE: &str = "post";

pub async fn get_all(db_pool: &DBPool) -> Result<Vec<Post>> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "SELECT * FROM {} ",
        TABLE
    );
    
    let rows = con
        .query(query.as_str(), &[])
        .await
        .map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_post(&r)).collect())
}

pub async fn get_one(db_pool: &DBPool, id: i32) -> Result<Post> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "SELECT * FROM {} WHERE id = $1 ",
        TABLE
    );
    
    let row = con
        .query_one(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;

    Ok(row_to_post(&row))
}

pub async fn create(db_pool: &DBPool, post: PostDto) -> Result<Post> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (word, pinyin, def, example) VALUES ($1, $2, $3, $4) RETURNING *",
        TABLE
    );
    let row = con
        .query_one(
            query.as_str(),
            &[&post.word, &post.pinyin, &post.def, &post.example],
        )
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_post(&row))

}

pub async fn delete(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_post(row: &Row) -> Post {
    let id: i32 = row.get(0);
    let word: String = row.get(1);
    let pinyin: Option<String> = row.get(2);
    let def: String = row.get(3);
    let example: Option<String> = row.get(4);
    Post {
        id,
        word,
        pinyin,
        def,
        example,
    }
}