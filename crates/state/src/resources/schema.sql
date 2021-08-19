BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS "layers" (
	"id" INTEGER NOT NULL UNIQUE,
	"fingerprint" BLOB NOT NULL CHECK(length("fingerprint") == 32),
	"ready" INTEGER NOT NULL,
	PRIMARY KEY("id")
);

CREATE TABLE IF NOT EXISTS "values" (
	"id" INTEGER NOT NULL UNIQUE,
	"key_hash" BLOB NOT NULL CHECK(length("key_hash") == 32),
	"value"	BLOB,
	"layer_id" INTEGER,
	PRIMARY KEY("id"),
	FOREIGN KEY("layer_id") REFERENCES "layers"("id") ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "values_key_hash" ON "values" (
	"key_hash" ASC
);

CREATE INDEX IF NOT EXISTS "values_by_layer_id_and_key_hash" ON "values" (
	"layer_id" ASC,
	"key_hash" ASC
);

COMMIT;
