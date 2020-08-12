use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "category"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "tag"]
pub struct Tag {
    pub id: i32,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Identifiable)]
#[belongs_to(Category, foreign_key = "category")]
#[table_name = "problem"]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub updated_time: chrono::NaiveDateTime,
    pub difficulty: i8,
    pub category: i32,
    // pub tags: i32,
    pub order: i32,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[belongs_to(Problem, foreign_key = "problem")]
#[belongs_to(Tag, foreign_key = "tag")]
#[table_name = "problem_tag"]
pub struct ProblemTag {
    pub id: i32,
    pub problem: i32,
    pub tag: i32,
}

#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[belongs_to(Problem, foreign_key = "problem")]
#[table_name = "solution"]
pub struct Solution {
    pub id: i32,
    pub content: String,
    pub problem: i32,
    pub level: i8,
}
