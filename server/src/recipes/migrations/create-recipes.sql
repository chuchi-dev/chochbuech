CREATE TABLE recipes (
	id CHAR(14) PRIMARY KEY,
	creator CHAR(14) NOT NULL,
	title TEXT NOT NULL,
	created_on TIMESTAMP NOT NULL,
	updated_on TIMESTAMP NOT NULL
);

CREATE INDEX idx_recipes_creator ON recipes (creator);

CREATE TABLE chapters (
	id CHAR(14) PRIMARY KEY,
	recipe CHAR(14) NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
	order INTEGER NOT NULL,
	title TEXT NOT NULL
);

CREATE INDEX idx_chapters_recipe ON chapters (recipe);

CREATE TABLE ingredients (
	id CHAR(14) PRIMARY KEY,
	chapter CHAR(14) NOT NULL REFERENCES chapters (id) ON DELETE CASCADE,
	order INTEGER NOT NULL,
	title TEXT NOT NULL,
	amount DOUBLE NOT NULL,
	unit TEXT NOT NULL
);

CREATE INDEX idx_ingredients_chapter ON ingredients (chapter);

CREATE TABLE blocks (
	id CHAR(14) PRIMARY KEY,
	chapter CHAR(14) NOT NULL REFERENCES chapters (id) ON DELETE CASCADE,
	order INTEGER NOT NULL,
	text TEXT
);

CREATE INDEX idx_blocks_chapter ON blocks (chapter);
