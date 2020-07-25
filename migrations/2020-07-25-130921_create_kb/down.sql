-- This file should undo anything in "up.sql"
BEGIN;

DROP TABLE "problem_tags";

DROP TABLE "solution";

DROP TABLE "tag";

DROP TABLE "problem";

DROP TABLE "category";

COMMIT;