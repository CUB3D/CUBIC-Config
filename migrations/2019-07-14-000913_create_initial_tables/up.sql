CREATE TABLE Projects (
    id INTEGER UNIQUE PRIMARY KEY,
    name VARCHAR NOT NULL,
    projectUUID VARCHAR NOT NULL UNIQUE
);

CREATE TABLE Layers (
    id INTEGER UNIQUE PRIMARY KEY,
    name VARCHAR NOT NULL,
    condition VARCHAR NOT NULL,
    projectID INTEGER,
    FOREIGN KEY(projectID) REFERENCES Projects(id)
);

create table Property (
    id INTEGER UNIQUE PRIMARY KEY,
    name VARCHAR NOT NULL,
    type INTEGER,
    value VARCHAR,
    layerID INTEGER,
    FOREIGN KEY(layerID) REFERENCES Layers(id)
)