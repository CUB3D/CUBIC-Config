CREATE TABLE Projects (
    id INT UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
    name VARCHAR(128) NOT NULL,
    projectUUID VARCHAR(64) NOT NULL UNIQUE
);

CREATE TABLE Layers (
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(128) NOT NULL,
    _condition VARCHAR(128) NOT NULL,
    projectID INT NOT NULL,
    FOREIGN KEY(projectID) REFERENCES Projects(id)
);

create table Property (
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(128) NOT NULL,
    type INT NOT NULL,
    value VARCHAR(256),
    layerID INT NOT NULL,
    FOREIGN KEY(layerID) REFERENCES Layers(id)
)