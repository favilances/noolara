# noolara

Rust-based web server with MongoDB connectivity checks.

## Environment

All runtime settings are read from a single file:

- config/settings.toml

Default file content:

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[mongodb]
uri = "mongodb://127.0.0.1:27017"
db = "noolara"

[cors]
allow_origin = "*"

[security]
max_payload_bytes = 1048576
allowed_hosts = ["127.0.0.1", "localhost"]
expose_detailed_errors = false
enable_hsts = false
```

Run with default config file:

cargo run

If needed, you can set a custom config file path with:

CONFIG_PATH=config/settings.toml cargo run

## Security Defaults

- Security headers enabled (`X-Content-Type-Options`, `X-Frame-Options`, `Referrer-Policy`, `Permissions-Policy`)
- Host allowlist check enabled from `security.allowed_hosts`
- Request payload size limit from `security.max_payload_bytes`
- Detailed internal errors disabled by default (`security.expose_detailed_errors = false`)
- HSTS optional (`security.enable_hsts = true` only behind HTTPS/TLS)

## API Endpoints

- GET /api/ping
- GET /api/mongo-ping
- GET /ra

## Test Web UI

A lightweight test page is available under:

- /test-web

After running the server, open:

- http://127.0.0.1:8080/test-web

Use the buttons to send requests to ping endpoints and inspect response payloads.
