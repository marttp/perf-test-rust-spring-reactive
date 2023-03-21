CREATE DATABASE perf_test;

\c perf_test;

CREATE TABLE IF NOT EXISTS employee (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

INSERT INTO employee(id, name) VALUES (1, 'Test1');
INSERT INTO employee(id, name) VALUES (2, 'Test2');
INSERT INTO employee(id, name) VALUES (3, 'Test3');
INSERT INTO employee(id, name) VALUES (4, 'Test4');
INSERT INTO employee(id, name) VALUES (5, 'Test5');
INSERT INTO employee(id, name) VALUES (6, 'Test6');
INSERT INTO employee(id, name) VALUES (7, 'Test7');
INSERT INTO employee(id, name) VALUES (8, 'Test8');
