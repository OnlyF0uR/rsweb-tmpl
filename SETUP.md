# Setup

### Certificate
Either use a self-signed certificate or a certificate from a trusted CA. For the certificate both the key (cert.key) and the certificate (cert.pem) should be in the root directory.

### Database & Cache

```bash
docker run -d --name stack-redis -p 6379:6379 redis:latest
docker run -d --name stack-timescale -p 5432:5432 -e POSTGRES_PASSWORD=password timescale/timescaledb-ha:pg16
```

Migrations and seeding can be done using the following command (--sim for simulating):
```bash
cargo run --bin populate -- --sim
```

You may have to manually create the database and insert the SQL schemas when setting up the database for the first time.
So that there actually is a database to connect to and so that sqlx does not complain about missing tables.

### Environment Variables

Create a `.env` file in the root directory with the following content:

```env
DATABASE_URL=postgres://<username>:<password>@127.0.0.1:5432/<db_name>
REDIS_URL=redis://127.0.0.1:6379
GOOGLE_OAUTH_CLIENT_ID=<google_client_id>
GOOGLE_OAUTH_CLIENT_SECRET=<google_client_secret>
```

### Run

```bash
cargo run --bin stack
```
