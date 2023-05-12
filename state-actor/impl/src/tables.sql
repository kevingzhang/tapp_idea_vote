CREATE TABLE Ideas
(
  id                      TEXT UNIQUE,
  title						        TEXT,
  description				      TEXT,
  owner						        TEXT,
	create_at               INTEGER,
  total_contribution      TEXT
);
CREATE INDEX idx_id ON Ideas (id);
