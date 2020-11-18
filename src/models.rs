use diesel::{
    ExpressionMethods, JoinOnDsl, NullableExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::{objects, users};

#[derive(Debug, Queryable)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    pub fn search(conn: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        users::table.load(conn)
    }
}

#[derive(Debug, Queryable)]
pub struct Object {
    pub id: Uuid,
    pub created_by: Uuid,
    pub deleted_by: Option<Uuid>,
}

impl Object {
    //pub fn search(conn: &PgConnection) -> Result<Vec<(Object, User)>, diesel::result::Error> {
    //pub fn search(conn: &PgConnection) -> Result<Vec<(Object, Option<User>)>, diesel::result::Error> {
    pub fn search(conn: &PgConnection) -> Result<Vec<(Object, User, Option<User>)>, diesel::result::Error> {
        objects::table
            .inner_join(users::table.on(objects::columns::created_by.eq(users::columns::id)))
            .left_join(users::table.on(objects::columns::deleted_by.eq(users::columns::id.nullable())))
            .load(conn)
    }
}
