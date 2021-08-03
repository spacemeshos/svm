BEGIN TRANSACTION;
DROP TABLE IF EXISTS "commits";
CREATE TABLE IF NOT EXISTS "commits" (
	"id"	INTEGER NOT NULL UNIQUE,
	"hash"	BLOB UNIQUE,
	PRIMARY KEY("id")
);
DROP TABLE IF EXISTS "values";
CREATE TABLE IF NOT EXISTS "values" (
	"id"	INTEGER NOT NULL UNIQUE,
	"commit_id"	INTEGER,
	"key_hash"	BLOB NOT NULL,
	"value"	BLOB NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("commit_id") REFERENCES "commits"("id")
);
DROP INDEX IF EXISTS "commits_by_hash";
CREATE INDEX IF NOT EXISTS "commits_by_hash" ON "commits" (
	"hash"	ASC
);
DROP INDEX IF EXISTS "values_by_commit_id_and_key_hash";
CREATE UNIQUE INDEX IF NOT EXISTS "values_by_commit_id_and_key_hash" ON "values" (
	"commit_id"	ASC,
	"key_hash"	ASC
);
COMMIT;
