// @generated automatically by Diesel CLI.

diesel::table! {
    round1_data (id) {
        id -> Integer,
        team1 -> Integer,
        team2 -> Integer,
        team3 -> Integer,
        team4 -> Integer,
        team5 -> Integer,
        team6 -> Integer,
    }
}

diesel::table! {
    round1_info (id) {
        id -> Nullable<Integer>,
        current_stage -> Nullable<Integer>,
    }
}

diesel::table! {
    round1_tokutendt (id) {
        id -> Integer,
        correct -> Integer,
        miss -> Integer,
        ask_throw -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    round1_data,
    round1_info,
    round1_tokutendt,
);
