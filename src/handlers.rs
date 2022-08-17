use super::DbPool;
use crate::models::{NewTweet, Tweet, TweetPayload};
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::PgConnection;

// its an super err??
// i really need to study rust
// just and std error that can handle the async stuff
// and concurrency
type DbError = Box<dyn std::error::Error + Send + Sync>;

// finds all the tweets
// search querys concur in the db
// its fast?
// probably
// will discover later
#[get("/tweets")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let tweets = web::block(move || {
        let conn = pool.get()?;
        find_all(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(tweets))
}

// creates a tweet
// its a post request with the message
// concur & async
#[post("/tweets")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<TweetPayload>,
) -> Result<HttpResponse, Error> {
    let tweet = web::block(move || {
        let conn = pool.get()?;
        add_tweet(&payload.message, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweet))
}

// show a tweet by its id
#[get("/tweets/{id}")]
async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let tweet = web::block(move || {
        let conn = pool.get()?;
        find_id(id.into_inner(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(tweet))
}

// changes message of a tweet
// searches by id
// and replaces message
#[put("/tweets/{id}")]
async fn update(
    id: web::Path<i32>,
    payload: web::Json<TweetPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let tweet = web::block(move || {
        let conn = pool.get()?;
        update_tweet(id.into_inner(), &conn, &payload.message.clone())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweet))
}

// deletes a tweet
// searcher by id
#[delete("/tweets/{id}")]
async fn delete(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        let conn = pool.get()?;
        delete_tweet(id.into_inner(), &conn)
    })
    .await?
    .map(|tweet| HttpResponse::Ok().json(tweet))
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(res)
}

// makes a vec of tweets in db
// just load all,maybe could make it better
// with real sql querys
fn find_all(conn: &PgConnection) -> Result<Vec<Tweet>, DbError> {
    use crate::schema::tweets::dsl::*;

    let items = tweets.load::<Tweet>(conn)?;
    Ok(items)
}

// uses diesel to add a tweet
// makes a tweet struct with the message you passed
// pass it to diesel,and it goes to the db
fn add_tweet(_message: &str, conn: &PgConnection) -> Result<Tweet, DbError> {
    use crate::schema::tweets::dsl::*;

    let new_tweet = NewTweet { message: _message };

    let res = diesel::insert_into(tweets)
        .values(&new_tweet)
        .get_result(conn)?;
    Ok(res)
}

// finds an id with a select & where clause
// its all diesel
// i even fell kind of bad
// will have to look into diesel
fn find_id(tweet_id: i32, conn: &PgConnection) -> Result<Option<Tweet>, DbError> {
    use crate::schema::tweets::dsl::*;

    let tweet = tweets
        .filter(id.eq(tweet_id))
        .first::<Tweet>(conn)
        .optional()?;

    Ok(tweet)
}

// diesel updates db,finds by id & change the message
fn update_tweet(tweet_id: i32, conn: &PgConnection, _message: &str) -> Result<Tweet, DbError> {
    use crate::schema::tweets::dsl::*;

    let tweet = diesel::update(tweets.find(tweet_id))
        .set(message.eq(_message))
        .get_result::<Tweet>(conn)?;
    Ok(tweet)
}

// diesel deletes tweet based by id
fn delete_tweet(tweet_id: i32, conn: &PgConnection) -> Result<usize, DbError> {
    use crate::schema::tweets::dsl::*;

    let count = diesel::delete(tweets.find(tweet_id)).execute(conn)?;
    Ok(count)
}
