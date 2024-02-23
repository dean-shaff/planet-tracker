## Planet Tracker Server

A simple Flask app for tracking the positions of planets in the sky. This is
the server code. Head [here](https://github.com/dean-shaff/planet-tracker_client)
for the Rust/leptos client.

### Running locally

The client is a git submodule, so we need to initialize it and fetch it when we first clone the server.

```
git submodule init
git submodule update
```

(Note there's no need for the `--recursive` flag as we're just initializing one submodule)

With poetry installed, we can install and run the server:

```
poetry install
poetry run litestar run
```

Now we can point our browser to `localhost:8000` to get the position of planets at our current location!
