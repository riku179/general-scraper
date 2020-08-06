table! {
    contents (id) {
        id -> Text,
        url -> Text,
        source_id -> Integer,
        title -> Text,
        body -> Text,
        created_at -> Timestamp,
    }
}

table! {
    sources (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        selectors -> Text,
        last_accessed -> Timestamp,
        last_accessed_urls -> Text,
        created_at -> Timestamp,
    }
}

joinable!(contents -> sources (source_id));

allow_tables_to_appear_in_same_query!(contents, sources,);
