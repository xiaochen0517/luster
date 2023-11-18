// @generated automatically by Diesel CLI.

diesel::table! {
    todo_table (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        due_date -> Nullable<Date>,
        completed -> Bool,
        create_time -> Timestamp,
    }
}
