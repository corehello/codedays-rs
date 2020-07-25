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
        updated_time -> Timestamptz,
        difficulty -> Int2,
        order -> Int4,
        category_id -> Int4,
    }
}

table! {
    problem_tags (id) {
        id -> Int4,
        problem_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    solution (id) {
        id -> Int4,
        content -> Text,
        level -> Int2,
        problem_id -> Int4,
    }
}

table! {
    tag (id) {
        id -> Int4,
        key -> Varchar,
        name -> Varchar,
    }
}

joinable!(problem -> category (category_id));
joinable!(problem_tags -> problem (problem_id));
joinable!(problem_tags -> tag (tag_id));
joinable!(solution -> problem (problem_id));

allow_tables_to_appear_in_same_query!(
    category,
    problem,
    problem_tags,
    solution,
    tag,
);
