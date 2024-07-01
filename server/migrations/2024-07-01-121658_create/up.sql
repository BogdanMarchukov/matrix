-- Your SQL goes here
CREATE TABLE "users"(
	"user_id" UUID NOT NULL PRIMARY KEY,
	"telegram_id" INT8 NOT NULL,
	"first_name" TEXT,
	"last_name" TEXT,
	"username" TEXT,
	"language_code" TEXT,
	"is_premium" BOOL,
	"photo_url" TEXT
);

