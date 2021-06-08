table! {
    accounts (id) {
        id -> Varchar,
        user_id -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    answer_frames (id) {
        id -> Varchar,
        text -> Varchar,
        question_id -> Varchar,
    }
}

table! {
    diagnoses (id) {
        id -> Varchar,
        title -> Varchar,
        organization_id -> Varchar,
    }
}

table! {
    organizations (id) {
        id -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    questions (id) {
        id -> Varchar,
        text -> Varchar,
        diagnosis_id -> Varchar,
        question_type -> Int4,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users_organizations (user_id, organization_id) {
        user_id -> Varchar,
        organization_id -> Varchar,
    }
}

joinable!(accounts -> users (user_id));
joinable!(answer_frames -> questions (question_id));
joinable!(diagnoses -> organizations (organization_id));
joinable!(questions -> diagnoses (diagnosis_id));
joinable!(users_organizations -> organizations (organization_id));
joinable!(users_organizations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    answer_frames,
    diagnoses,
    organizations,
    questions,
    todos,
    users,
    users_organizations,
);
