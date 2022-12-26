CREATE TABLE groups (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX groups_index ON groups(name);

CREATE TABLE participants (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    group_id INTEGER,
    FOREIGN KEY(group_id) REFERENCES groups(id)
);


CREATE TABLE fortunes (
  id INTEGER PRIMARY KEY NOT NULL,
  text TEXT NOT NULL
);

INSERT INTO
    fortunes(text)
VALUES
    ("hello there")
;
