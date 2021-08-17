BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS "commits" (
	"id" INTEGER NOT NULL UNIQUE,
	"fingerprint" BLOB NOT NULL CHECK(length("fingerprint") == 32),
	PRIMARY KEY("id")
);

CREATE TABLE IF NOT EXISTS "values" (
	"id" INTEGER NOT NULL UNIQUE,
	"commit_id"	INTEGER,
	"key_hash" BLOB NOT NULL CHECK(length("key_hash") == 32),
	"value"	BLOB,
	PRIMARY KEY("id"),
	FOREIGN KEY("commit_id") REFERENCES "commits"("id") ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "commits_by_fingerprint" ON "commits" (
	"fingerprint" ASC
);

CREATE UNIQUE INDEX IF NOT EXISTS "values_by_commit_id_and_key_hash" ON "values" (
	"commit_id" ASC,
	"key_hash" ASC
);

COMMIT;
