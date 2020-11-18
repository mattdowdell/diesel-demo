table! {
    objects (id) {
        id -> Uuid,
        created_by -> Uuid,
        deleted_by -> Nullable<Uuid>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(objects, users,);
