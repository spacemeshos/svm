BEGIN TRANSACTION;

DROP TABLE IF EXISTS "commits";
CREATE TABLE IF NOT EXISTS "commits" (
	"id" INTEGER NOT NULL UNIQUE,
	"fingerprint" BLOB NOT NULL CHECK(length("fingerprint") == 32),
	PRIMARY KEY("id")
);

DROP TABLE IF EXISTS "values";
CREATE TABLE IF NOT EXISTS "values" (
	"id" INTEGER NOT NULL UNIQUE,
	"commit_id"	INTEGER,
	"key_hash" BLOB NOT NULL CHECK(length("key_hash") == 32),
	"value"	BLOB,
	PRIMARY KEY("id"),
	FOREIGN KEY("commit_id") REFERENCES "commits"("id")
);

DROP INDEX IF EXISTS "commits_by_fingerprint";
CREATE INDEX IF NOT EXISTS "commits_by_fingerprint" ON "commits" (
	"fingerprint" ASC
);

DROP INDEX IF EXISTS "values_by_commit_id_and_key_hash";
CREATE UNIQUE INDEX IF NOT EXISTS "values_by_commit_id_and_key_hash" ON "values" (
	"commit_id" ASC,
	"key_hash" ASC
);

COMMIT;
