diesel::table! {
    posts (_id_) {
        _id_ -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
