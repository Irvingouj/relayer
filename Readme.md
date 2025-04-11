# relayer

A minimal TCP reverse proxy written in Rust, designed for containerized deployment.

## 🔧 What It Does

**relayer** listens on a specified local port and forwards all incoming TCP traffic to a remote address, defined via environment variable. It's useful for exposing internal services (like media servers or internal APIs) through a controlled proxy layer — especially behind a platform like [Coolify](https://coolify.io) or your own Traefik/Nginx frontend.

## 🚀 Main Use Case

This project was created to:

- Deploy **a tiny, containerized TCP forwarder**.
- Be used as a reverse proxy layer inside **Coolify** or other self-hosted platforms.
- Relay traffic from a public-facing proxy (e.g., `proxy.example.com`) to **private/internal services** like:
  - `http://primary.example.com:8096` (e.g., Jellyfin)
  - `http://primary.example.com:2283` (e.g., Immich)
- **Avoid direct exposure** of services that violate Cloudflare/media hosting rules.
- Scale easily by spinning up one proxy container per domain or endpoint.

## 🌐 Environment Variables

| Variable                 | Description                             | Example                                |
|--------------------------|-----------------------------------------|----------------------------------------|
| `RELAYER_ENDPOINT`       | Full URL to forward traffic to          | `http://primary.example.com:8096`     |
| `RELAYER_LISTENING_PORT` | Port to listen on (default: `3000`)     | `3000`                                  |

## 🐳 Docker Usage

```bash
docker build -t relayer .

docker run --rm \
  -e RELAYER_ENDPOINT=http://primary.example.com:8096 \
  -e RELAYER_LISTENING_PORT=3000 \
  -p 3000:3000 \
  relayer
```

## 📆 Coolify Deployment

1. Create a Dockerfile app.
2. Set environment variables in Coolify:
   - `RELAYER_ENDPOINT=http://primary.example.com:8096`
   - (Optional) `RELAYER_LISTENING_PORT=3000`
3. Set the exposed port to match `RELAYER_LISTENING_PORT`.

Coolify or Traefik will handle routing TLS and domains — `relayer` simply forwards the raw TCP traffic.

## 💦 Tech Stack

- Language: [Rust](https://www.rust-lang.org)
- Runtime: [Tokio](https://tokio.rs)
- Dependencies:
  - `tokio` (async TCP handling)
  - `url` (for parsing endpoint)
  - `dotenvy` (optional `.env` support)

## 📄 License

MIT

---

Made with ❤️ to keep internal services safely reachable through smart relaying.

