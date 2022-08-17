use serde::{Deserialize, Serialize};

use crate::schema::tweets;

// A tweet is an id
// & its text
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Tweet {
    pub id: i32,
    pub message: String,
}

// Generates a new tweet
#[derive(Debug, Insertable)]
#[table_name = "tweets"]
pub struct NewTweet<'a> {
    pub message: &'a str,
}

// Holds tweet text
#[derive(Debug, Serialize, Deserialize)]
pub struct TweetPayload {
    pub message: String,
}
