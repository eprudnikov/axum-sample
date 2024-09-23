CREATE TABLE orders
(
    id         UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE order_items
(
    id       UUID PRIMARY KEY,
    order_id UUID
        REFERENCES orders ON DELETE CASCADE
);