table! {
    authentication_pin_attempts (id) {
        id -> Int4,
        user_id -> Int4,
        pin -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    authentication_pins (id) {
        id -> Int4,
        user_id -> Int4,
        pin -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Nullable<Text>,
        phone_number -> Nullable<Text>,
    }
}

joinable!(authentication_pin_attempts -> users (user_id));
joinable!(authentication_pins -> users (user_id));

allow_tables_to_appear_in_same_query!(
    authentication_pin_attempts,
    authentication_pins,
    users,
);
