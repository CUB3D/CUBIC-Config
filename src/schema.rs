table! {
    Layers (id) {
        id -> Integer,
        name -> Varchar,
        _condition -> Varchar,
        projectID -> Nullable<Integer>,
    }
}

table! {
    Projects (id) {
        id -> Integer,
        name -> Varchar,
        projectUUID -> Varchar,
    }
}

table! {
    Property (id) {
        id -> Integer,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        value -> Nullable<Varchar>,
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
