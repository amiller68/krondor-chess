# Krondor-Chess

I thought that writing a little chess app would be a fun way to learn both HTMX and Shuttle-Rs.

Right now all the web app does is allow you to create a game, and view the pretty formatted FEN of said games, which is just starting position.


See [https://krondor-chess.shuttleapp.rs/](https://krondor-chess.shuttleapp.rs) for deployed version

## Requirements
- Rust & Cargo
- [Shuttle](https://docs.shuttle.rs/getting-started/installation)
- Docker

## Local development and usage

Run code checks:
```bash
make check
```

Run tests:
```bash
make test
```

Prepare SQLX queries:
```bash
./bin/queries.sh
```

Run locally:
```bash
./bin/run.sh
```

## Deployment

You'll need to get an Api key if you want to deploy to [Shuttle.rs](https://console.shuttle.rs/). It's easy and free to get one :) 

In order to deploy to Shuttle.rs run:
```bash
cargo shuttle deploy
```
Remember to ensure your repository isn't dirty, your code passes checks, and your migrations are properly version controlled!

### CI/CD

This repository comes with workflows for deploying to Shuttle.rs from GitHub. These will run on pushing to `main` if you set up the `SHUTTLE_API_KEY` secret for GitHub actions in your repository.
