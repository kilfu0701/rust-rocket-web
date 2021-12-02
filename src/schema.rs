table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        description -> Nullable<Text>,
        authenticated_at -> Nullable<Datetime>,
        created_at -> Nullable<Datetime>,
        deleted_at -> Nullable<Datetime>,
    }
}
