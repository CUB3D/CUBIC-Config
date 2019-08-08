CREATE TABLE Projects (
    id INT UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
    name VARCHAR(128) NOT NULL,
    projectUUID VARCHAR(64) NOT NULL UNIQUE
);

CREATE TABLE Layers (
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(128) NOT NULL,
    _condition VARCHAR(128) NOT NULL,
    projectID INT,
    FOREIGN KEY(projectID) REFERENCES Projects(id)
);

create table Property (
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(128) NOT NULL,
    type INT,
    value VARCHAR(256),
    layerID INT,
    FOREIGN KEY(layerID) REFERENCES Layers(id)
)