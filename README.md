# Tagline

Tagline is a game about guessing movies just from the tagline or promotional line given to the movie.

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

## Frontend [WIP]

The frontend is currently WIP, but will be runnable by simply omitting the cli argument:

```bash
cargo run
```

or

```bash
cargo build --release &&
./target/release/tagline
```

Then the frontend will be accesible on http://localhost:3000/.
