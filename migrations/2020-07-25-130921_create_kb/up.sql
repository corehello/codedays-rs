BEGIN;

--
-- Create model Category
--
CREATE TABLE "category" (
    "id" serial NOT NULL PRIMARY KEY,
    "name" varchar(64) NOT NULL
);

--
-- Create model Problem
--
CREATE TABLE "problem" (
    "id" serial NOT NULL PRIMARY KEY,
    "title" varchar(128) NOT NULL,
    "content" text NOT NULL,
    "updated_time" timestamp with time zone NOT NULL,
    "difficulty" smallint NOT NULL CHECK ("difficulty" >= 0),
    "order" integer NOT NULL UNIQUE,
    "category_id" integer NOT NULL
);

--
-- Create model Tag
--
CREATE TABLE "tag" (
    "id" serial NOT NULL PRIMARY KEY,
    "key" varchar(64) NOT NULL,
    "name" varchar(64) NOT NULL
);

--
-- Create model Solution
--
CREATE TABLE "solution" (
    "id" serial NOT NULL PRIMARY KEY,
    "content" text NOT NULL,
    "level" smallint NOT NULL CHECK ("level" >= 0),
    "problem_id" integer NOT NULL
);

--
-- Add field tags to problem
--
CREATE TABLE "problem_tags" (
    "id" serial NOT NULL PRIMARY KEY,
    "problem_id" integer NOT NULL,
    "tag_id" integer NOT NULL
);

ALTER TABLE
    "problem"
ADD
    CONSTRAINT "problem_category_id_fk_category_id" FOREIGN KEY ("category_id") REFERENCES "category" ("id") DEFERRABLE INITIALLY DEFERRED;

CREATE INDEX "problem_category_id_index" ON "problem" ("category_id");

ALTER TABLE
    "solution"
ADD
    CONSTRAINT "solution_problem_id_fk_problem_id" FOREIGN KEY ("problem_id") REFERENCES "problem" ("id") DEFERRABLE INITIALLY DEFERRED;

CREATE INDEX "solution_problem_id_index" ON "solution" ("problem_id");

ALTER TABLE
    "problem_tags"
ADD
    CONSTRAINT "problem_tags_problem_id_fk_problem_id" FOREIGN KEY ("problem_id") REFERENCES "problem" ("id") DEFERRABLE INITIALLY DEFERRED;

ALTER TABLE
    "problem_tags"
ADD
    CONSTRAINT "problem_tags_tag_id_fk_tag_id" FOREIGN KEY ("tag_id") REFERENCES "tag" ("id") DEFERRABLE INITIALLY DEFERRED;

ALTER TABLE
    "problem_tags"
ADD
    CONSTRAINT "problem_tags_problem_id_tag_id_d6634356_uniq" UNIQUE ("problem_id", "tag_id");

CREATE INDEX "problem_tags_problem_id_index" ON "problem_tags" ("problem_id");

CREATE INDEX "problem_tags_tag_id_index" ON "problem_tags" ("tag_id");

COMMIT;