table! {
    state (name) {
        name -> Text,
        val -> Text,
    }
}

table! {
    temperature (timestamp) {
        timestamp -> Nullable<Timestamp>,
        value -> Double,
    }
}

allow_tables_to_appear_in_same_query!(
    state,
    temperature,
);
