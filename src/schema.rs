table! {

    tweet (id) {
        id -> Int4,
        message -> Nullable<Varchar>,
    }
}

table! {
    tweets (id) {
        id -> Int4,
        message -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    tweet,
    tweets,
);
