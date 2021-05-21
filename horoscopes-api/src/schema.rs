table! {
    accounts (id) {
        id -> Varchar,
        user_id -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    organizations (id) {
        id -> Varchar,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    todos (id) {
        id -> Varchar,
        title -> Varchar,
        done -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users_organizations (user_id, organization_id) {
        user_id -> Varchar,
        organization_id -> Varchar,
    }
}

joinable!(accounts -> users (user_id));
joinable!(users_organizations -> organizations (organization_id));
joinable!(users_organizations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    organizations,
    todos,
    users,
    users_organizations,
);
