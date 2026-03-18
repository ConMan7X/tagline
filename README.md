# Tagline

Tagline is a game about guessing movies just from the tagline or promotional line given to the movie.

## Docker

Running tagline in a docker container is the easiest method.

You will need to create a .env file with your TMDB API key:

```.env
API_KEY={Your API Key}
```

Then, to run in detached mode simply run:

```bash
docker compose up -d
```

## CLI

You can run the CLI version of Tagline by simply using the cli argument:

```bash
cargo run -- cli
```
or

```bash
cargo build --release &&
./target/release/tagline cli
```

## Frontend

The frontend is runnable by simply omitting the cli argument:

```bash
cargo run
```

or

```bash
cargo build --release &&
./target/release/tagline
```

Then the frontend will be accesible on http://localhost:3000/.
