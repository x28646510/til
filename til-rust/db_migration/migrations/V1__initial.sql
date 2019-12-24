CREATE TABLE users
(
    id    varchar(255) NOT NULL,
    name  varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    CONSTRAINT user_pk PRIMARY KEY (id),
    CONSTRAINT user_email_unique UNIQUE (email)
);

CREATE TABLE products
(
    id      varchar(255)   NOT NULL,
    user_id varchar(255)   NOT NULL,
    name    varchar(255)   NOT NULL,
    price   decimal(10, 0) NOT NULL,
    CONSTRAINT product_pk PRIMARY KEY (id),
    CONSTRAINT product_fk0 FOREIGN KEY (user_id) REFERENCES users (id)
);
