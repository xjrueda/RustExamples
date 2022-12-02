diesel::table! {
    posts (_id_) {
        _id_ -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    person (id) {
        id -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        created_at -> Timestamp,
        valid_until -> Date,
    }
}