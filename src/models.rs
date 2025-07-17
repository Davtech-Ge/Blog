use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use crate::schema::{comments, users};
use crate::errors::AppError;
use crate::schema::posts;


type Result<T> = std::result::Result<T, AppError>;

//this one is for creating a post
#[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
#[diesel(belongs_to(User))]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub fn create_post(conn: &mut SqliteConnection, user: &User, title: String, body: String) -> Result<Post> {
    conn.transaction(|conn | {
        diesel::insert_into(posts::table)
            .values((
                posts::user_id.eq(user.id),
                posts::title.eq(title),
                posts::body.eq(body)
            ))
            .execute(conn)?;

        posts::table
            .order(posts::id.desc())
            .select(posts::all_columns)
            .first(conn)
            .map_err(AppError::from)
    })
}


// this details are creating a new user
#[derive(Debug, Queryable, Identifiable, Serialize, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
}

pub fn create_user(conn: &mut SqliteConnection, username: &str) -> Result<User> {
    //since diesel does not return the created record, we need to use a transaction to ensure we can retrieve it
    conn.transaction(|conn| {
        //this will insert the user and return the created record
        diesel::insert_into(users::table)
            .values((users::username.eq(username),))
            .execute(conn)?;
        //we can then retrieve the last inserted user
        users::table
            .order(users::id.desc())
            .select((users::id, users::username))
            .first::<User>(conn) // this will return the last inserted user
            .map_err(AppError::from)
    })
}

//we are goint to use this enum to allow us to search for users by either username or id 
// the 'a is used to indicate that this enum is generic over a lifetime and can be used for 
//as long as the lifetime of the string slice
#[derive(Debug)]
pub enum UserKey<'a> {
    Username(&'a str),
    ID(i32),
}

pub fn find_user<'a>(conn: &mut SqliteConnection, key: UserKey<'a>) -> Result<User> {
    // this ensures that even if we search for the user by id or username we get the user
    match key {
        UserKey::Username(name) => {
            use diesel::QueryDsl;
            users::table
            .filter(users::username.eq(name))
            .select((users::id, users::username))
            .first::<User>(conn)
            .map_err(AppError::from)},

            UserKey::ID(id) => users::table
                .find(id)
                .select((users::id, users::username))
                .first::<User>(conn)
                .map_err(AppError::from),
    }
}

//To publish a post 
pub fn publish_post(conn: &mut SqliteConnection, post_id: i32) -> Result<Post> {
    conn.transaction(|conn| {
        diesel::update(posts::table.filter(posts::id.eq(post_id)))
        .set(posts::published.eq(true))
        .execute(conn)?;

    posts::table
        .find(post_id)
        .select(posts::all_columns)
        .first::<Post>(conn)
        .map_err(AppError::from)
    })
}


pub fn all_posts(conn: &mut SqliteConnection) -> Result<Vec<(Post, User)>> {
    posts::table
        .order(posts::id.desc())
        .filter(posts::published.eq(true))
        .inner_join(users::table)
        .select((posts::all_columns, (users::id, users::username)))
        .load::<(Post, User)>(conn)
        .map_err(AppError::from)
}

pub fn user_posts(conn: &mut SqliteConnection, user_id: i32) -> Result<Vec<Post>> {
    posts::table
        .filter(posts::user_id.eq(user_id))
        .order(posts::id.desc())
        .select(posts::all_columns)
        .load::<Post>(conn)
        .map_err(AppError::from)
}

#[derive(Debug, Associations, Identifiable, Queryable, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Post))]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub body: String
}

pub fn create_comment(conn: &mut SqliteConnection, user_id: i32, post_id: i32, body: String) -> Result<Comment> {
    conn.transaction(|conn| {
        diesel::insert_into(comments::table)
            .values((
                comments::user_id.eq(user_id),
                comments::post_id.eq(post_id),
                comments::body.eq(body)
            ))
            .execute(conn)?;

        comments::table
            .order(comments::id.desc())
            .select(comments::all_columns)
            .first(conn)
            .map_err(AppError::from)
    })
}

pub fn post_comments(conn: &mut SqliteConnection, post_id: i32) -> Result<Vec<(Comment, User)>> {
    comments::table
        .filter(comments::post_id.eq(post_id))
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(conn)
        .map_err(AppError::from)
}

#[derive(Debug, Serialize, Queryable)]
pub struct PostWithComment{
    pub id: i32,
    pub title: String,
    pub published: bool,
}

pub fn user_comments(conn: &mut SqliteConnection, user_id: i32) -> Result<Vec<(Comment, PostWithComment)>> {
    comments::table
        .filter(comments::user_id.eq(user_id))
        .inner_join(posts::table)
        .select((
            comments::all_columns,
            (posts::id, posts::title, posts::published)
        ))
        .load::<(Comment, PostWithComment)>(conn)
        .map_err(AppError::from)
}