use barrel::{types, Migration};

/// Handle up migrations
fn up(migr: &mut Migration) {
    migr.create_table("category", |t| {
        t.add_column("id", types::primary());
        t.add_column("name", types::varchar(64));
    });

    migr.create_table("tag", |t| {
        t.add_column("id", types::primary());
        t.add_column("key", types::varchar(64));
        t.add_column("name", types::varchar(64));
    });

    migr.create_table("problem", |t| {
        t.add_column("id", types::primary());
        t.add_column("title", types::varchar(128));
        t.add_column("content", types::text());
        t.add_column("updated_time", types::date());
        t.add_column("difficulty", types::integer().default(1));
        t.add_column("order", types::integer().default(1).unique(true));
        t.add_column("category", types::foreign("category", "id").indexed(true));
    });

    migr.create_table("solution", |t| {
        t.add_column("id", types::primary());
        t.add_column("content", types::text());
        t.add_column("level", types::integer().default(0));
        t.add_column("problem", types::foreign("problem", "id").indexed(true));
    });
    // many to many
    migr.create_table("problem_tag", |t| {
        t.add_column("id", types::primary());
        t.add_column("problem", types::foreign("problem", "id"));
        t.add_column("tag", types::foreign("tag", "id"));

        t.add_index(
            "problem_tag_index",
            types::index(vec!["problem", "tag"])
                .unique(true)
                .nullable(false),
        );
    });
}

/// Handle down migrations
fn down(migr: &mut Migration) {
    migr.drop_table_if_exists("solution");
    migr.drop_table_if_exists("problem_tag");
    migr.drop_table_if_exists("problem");
    migr.drop_table_if_exists("tag");
    migr.drop_table_if_exists("category");
}
