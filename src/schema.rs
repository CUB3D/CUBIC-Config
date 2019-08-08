table! {
    Layers (id) {
        id -> Nullable<Integer>,
        name -> Text,
        condition -> Text,
        projectID -> Nullable<Integer>,
    }
}

table! {
    Projects (id) {
        id -> Nullable<Integer>,
        name -> Text,
        projectUUID -> Text,
    }
}

table! {
    Property (id) {
        id -> Nullable<Integer>,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        value -> Nullable<Text>,
        layerID -> Nullable<Integer>,
    }
}

joinable!(Layers -> Projects (projectID));
joinable!(Property -> Layers (layerID));

allow_tables_to_appear_in_same_query!(
    Layers,
    Projects,
    Property,
);
