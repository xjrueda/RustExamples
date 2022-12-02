/* This file is generated and managed by dsync */

use diesel::prelude::*;
use crate::schema::*;
use diesel::{insert_into, QueryResult, RunQueryDsl};
use diesel::{
    r2d2::{ConnectionManager, Pool as TPool},
    PgConnection
};
use serde::{Deserialize, Serialize};


type ConnPool = TPool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = posts, primary_key(_id_))]
pub struct Post {
    pub _id_: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = posts)]
pub struct PostForm {
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Serialize)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}

impl Post {
    pub fn create(pool: &ConnPool, item: PostForm) -> QueryResult<Post> {
        use crate::schema::posts::dsl::*;
        let db = &mut pool.get().unwrap();
        insert_into(posts).values(item).get_result::<Post>(db)
    }

    pub fn read(pool: &ConnPool, param_id: i32) -> QueryResult<Post> {
        use crate::schema::posts::dsl::*;
        let db = &mut pool.get().unwrap();
        posts.filter(_id_.eq(param_id)).first::<Post>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(pool: &ConnPool, page: i64, page_size: i64) -> QueryResult<PaginationResult<Post>> {
        use crate::schema::posts::dsl::*;
        let db = &mut pool.get().unwrap();
        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = posts.count().get_result(db)?;
        let items = posts.limit(page_size).offset(page * page_size).load::<Post>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(pool: &ConnPool, param_id: i32, item: &PostForm) -> QueryResult<Post> {
        use crate::schema::posts::dsl::*;
        let db = &mut pool.get().unwrap();
        diesel::update(posts.filter(_id_.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(pool: &ConnPool, param_id: i32) -> QueryResult<usize> {
        use crate::schema::posts::dsl::*;
        let db = &mut pool.get().unwrap();
        diesel::delete(posts.filter(_id_.eq(param_id))).execute(db)
    }
}

