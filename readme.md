
# Face Recognition Wallet Server (Rust)

This project sets up a wallet corresponding to the face features sent from a Python server, saving the data in a PostgreSQL database. The server then returns the wallet address to the user when they request their wallet using facial recognition.

## Setup Your PostgreSQL Database Server

You have two options for setting up your PostgreSQL database server:

1. **Use your own database server.**
2. **Use an online PostgreSQL database server (recommended).**

## Set Environment Variables in the .env File

Create a `.env` file and set the following environment variables with your database and JWT configuration:

```env
DATABASE_URL='postgres://username:password@host:port/dbname' # Your database account info
JWT_SECRET=secret                                            
JWT_EXPIRATION_TIME=3600                                    
JWT_NOT_BEFORE=30
```

## Create Table for Saving Wallet Info

You need to create a table to save wallet information. You can choose one of the following methods:

### Using Diesel

Follow the [Getting Started Guide for Diesel](https://diesel.rs/guides/getting-started.html).

Run the following commands:

```sh
diesel setup
diesel migration generate --diff-schema create_account
diesel migration run
```

After running these commands, you will see a folder named `xxxx-xx-xx-xxxxxx_create_account` with `up.sql` and `down.sql` files in the `migrations` folder. You need to modify one line in `up.sql` as follows:

```sql
"id" BIGSERIAL NOT NULL PRIMARY KEY,
```

### Using SQL Query Directly

You can run the following SQL query directly on your PostgreSQL server using an SQL command prompt:

```sql
CREATE TABLE "account" (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "uid" INT8 NOT NULL,
    "mnemonic" VARCHAR(256),
    "address" VARCHAR(256),
    "token" VARCHAR(256)
);
```

## Run the Project

To run the project, use the following command:

```sh
cargo run
```

## Note

When clearing your table data in the Rust server, ensure that you also clear the database of the Python server, as the Python server uses its own SQLite database. This synchronization is crucial to maintain consistency between the two databases.