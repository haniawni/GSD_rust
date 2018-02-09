table! {
    ctl (id) {
        id -> Int4,
        name -> Varchar,
        complete_date -> Nullable<Timestamptz>,
        discrete -> Bool,
    }
}
