table! {
    accounts (id) {
        id -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

table! {
    todos (id) {
        id -> Varchar,
        title -> Varchar,
        done -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    todos,
);
