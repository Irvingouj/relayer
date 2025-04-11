# Coolify http relayer

A minimal TCP reverse proxy written in Rust, designed for containerized deployment.

## 🔧 What It Does

**relayer** listens on a specified local port and forwards all incoming TCP traffic to a remote address, defined via environment variable. It's useful for exposing internal services (like media servers or internal APIs) through a controlled proxy layer — especially behind a platform like [Coolify](https://coolify.io) or your own Traefik/Nginx frontend.

## 🚀 Main Use Case

This project was created to:

- Deploy **a tiny, containerized TCP forwarder** for media heavy server that is not well supported by Cloudflare tunnel
- Be used as a reverse proxy layer inside **Coolify** or other self-hosted platforms.
- Relay traffic from a public-facing proxy (e.g., `public.example.com`) to **private/internal services** like:
  - `http://private.example.com:8096` (e.g., Maybe Jellyfin?)
  - `http://100.123.321.777:2283` (e.g., Maybe Immich?)
- **Avoid direct exposure** of services that violate Cloudflare/media hosting rules.
- Scale easily by spinning up one proxy container per domain or endpoint.

## 🌐 Environment Variables

| Variable                 | Description                             | Example                                |
|--------------------------|-----------------------------------------|----------------------------------------|
| `RELAYER_ENDPOINT`       | Full URL to forward traffic to          | `http://primary.example.com:8096`      |
| `RELAYER_LISTENING_PORT` | Port to listen on (default: `3000`)     | `3000`                                 |

## 🐳 Docker Usage
In Coolify, you can directly deploy by docker file like this
```bash
# Use your prebuilt image as the base image
FROM irvingouj/relayer:latest

# Expose the listening port (or expose in coolify)
EXPOSE 3000

```

## 📄 License

MIT

---


