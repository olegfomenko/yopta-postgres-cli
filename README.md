# Yopta SQL cli for PostgreSQL

Run: `yopta-postgres-cli <host> <user> <pass> <database>` 

## Language Syntax
```
ВЫСТЕЛИТЬ ПО - ORDER BY
ПЛАВЕЦ - FLOAT
ДЛИННЫЙСЕРИАЛ - BIGSERIAL
СЕМКА - INTEGER
ЕБНУТЬ - DELETE
ССЫЛОЧКА - REFERENCES
ЗАХУЯРИТЬ - CREATE
ИЛИ - OR
ЧЕТКО - TRUE
СХУЯРИТЬ СПРАВА - RIGHT JOIN
ХУЙНУТЬ - SET
НИХУЯ - NOT
СХУЯРИТЬ ПОСЕРЕДКЕ - INNER JOIN
КЛИЧКА - AS
МАЛЯВА - TEXT
ОБЪЕБАТЬ - UPDATE
НЕТРУШНАЯМАЛЯВА - CHAR
ХУЙ - BIG
ВЫСРАТЬ ПО - GROUP BY
СХРУЯРИТЬ ВСЕ - FULL JOIN
ЗДЕСЬ - FROM
НЕЧЕТКО - FALSE
СЕРИАЛ - SERIAL
В - INTO
МОЙНОМЕР - UUID
ЗАЛУПАБ - JSONB
ОЧКО - NULL
ХУЙНЯ - VALUES
ЕСТЬЧЕ? - SELECT
АЕСЛИНАЙДУ? - WHERE
КОРОТКОСТВОЛ - SMALLINT
ПОСЕРЕДКЕ - BETWEEN
СХУЯРИТЬ СЛЕВА - LEFT JOIN
ТАБЛО - TABLE
МЕНТЯРА - INDEX
МАМОЙКЛЯНУСЬ - HAVING
И - AND
ТРУШНАЯМАЛЯВА - VARCHAR
СХУЯРИТЬ - JOIN
ПАЦАН - BOOL
ЗАЛУПА - JSON
ПАХАН - PRIMARY KEY
СУНУТЬ - INSERT
ЕСЛИНЕХУЙ - DEFAULT
```

## Example queries

```sql
CREATE TABLE customers
(
    customer_id bigserial primary key,
    name        varchar(255),
    age         smallint
);

захуярить
табло customers
(
    customer_id длинныйсериал пахан,
    name        трушнаямалява(255),
    age         короткоствол

);
```

```sql
SELECT name
FROM customers
         INNER JOIN orders
                    ON customers.customer_id = orders.customer_id;

естьче? name
здесь customers
    схуярить посередке orders
        on customers.customer_id = orders.customer_id;
```

```sql
CREATE
INDEX idx_name
ON customers (name);

захуярить
ментяра idx_name
on customers (name);
```

```sql
UPDATE customers
SET age = 56
WHERE name = 'Bob';

объебать
customers
хуйнуть age = 56
аеслинайду? name = 'bob';
```

```sql
DELETE FROM customers WHERE name = 'Bob';

ебнуть здесь customers аеслинайду? name = 'bob';
```