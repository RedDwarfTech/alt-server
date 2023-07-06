table! {
    alt_app (id) {
        id -> Int8,
        name -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
    }
}

table! {
    alt_tag (id) {
        id -> Int8,
        name -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    alt_app,
    alt_tag,
);
