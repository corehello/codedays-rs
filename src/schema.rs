table! {
    category (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    problem (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        updated_time -> Date,
        difficulty -> Int4,
        order -> Int4,
        category -> Int4,
    }
}

table! {
    problem_tag (id) {
        id -> Int4,
        problem -> Int4,
        tag -> Int4,
    }
}

table! {
    solution (id) {
        id -> Int4,
        content -> Text,
        level -> Int4,
        problem -> Int4,
    }
}

table! {
    tag (id) {
        id -> Int4,
        key -> Varchar,
        name -> Varchar,
    }
}

joinable!(problem -> category (category));
joinable!(problem_tag -> problem (problem));
joinable!(problem_tag -> tag (tag));
joinable!(solution -> problem (problem));

allow_tables_to_appear_in_same_query!(
    category,
    problem,
    problem_tag,
    solution,
    tag,
);
