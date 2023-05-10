CREATE TABLE Ideas
(
  id                TEXT UNIQUE,
  title						  TEXT,
  description				TEXT,
  owner						  TEXT,
  status						TEXT,
  vote_num          INTEGER,
  vote_price        TEXT,
	create_at         INTEGER
);
CREATE INDEX idx_id ON Ideas (id);

CREATE TABLE IdeaVoter
(
  id                TEXT,
  user              TEXT UNIQUE,
  vote_num          INTEGER
);
