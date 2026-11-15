# 🃏 Conquián Mexicano Online (2–6 Jugadores) 🇲🇽

[![CI/CD](https://github.com/pinguxx/conquian/actions/workflows/ci.yml/badge.svg)](https://github.com/pinguxx/conquian/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?logo=docker)](https://www.docker.com/)
[![Render](https://img.shields.io/badge/Deploy-Render-46E3B7?logo=render)](https://render.com)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A real-time multiplayer web adaptation of the classic Mexican rummy card game **Conquián**, built with **Rust** (`Axum`, `Tokio`, `WebSockets`) and a responsive bilingual frontend supporting **2 to 6 players** with **Baraja Española** or **Poker** suits.

---

## 🌟 Key Features

1. **Multiplayer Scaling (2–6 Players)**:
   - **Auto-Scale Classic 9-Card Conquián (Default)**:
     - **2–3 players**: 1 Baraja Española (40 cards), **8 cards each**, meld **9** to win (e.g. 3 *tercias* = 9 cards).
     - **4–6 players**: Double Deck (80 cards), **8 cards each**, meld **9** to win.
   - Host can also force **Single Deck** (4p plays 7 cards / meld 8), **Double Deck** (8 cards / meld 9), or **Extended 10-Card Conquián** (10 cards / meld 11 to win).

2. **Authentic Baraja Española Rules**:
   - Ranks `1 (As), 2, 3, 4, 5, 6, 7, 10 (Sota), 11 (Caballo), 12 (Rey)` — **`7` and `10` are consecutive** in runs (`6-7-10`).
   - Instant visual toggle between traditional **Baraja Española** (*Oros, Copas, Espadas, Bastos*) and **Poker** (`♦ ♥ ♠ ♣`).
   - **Strict Turn-Order Claim Ring**: Priority rotates clockwise around the table one player at a time before any card is burned to *La Basura*.
   - **Hand Melds & Table Meld Mixing**: Anytime on your active turn (before flipping, while deciding on the offered card, or before discarding), lay sets/runs from your hand and mix cards across your table melds without needing to take the offered card.
   - **⚡ ¡Obligado! (Forcing)**: Force an opponent to take an offered card that extends one of their table melds, forcing them to immediately discard!
   - **🤝 Tablas & El Pozo Rollover**: When *El Mazo* runs out, the hand ends in *Tablas* and *El Pozo* chips roll over into the next deal.

3. **Cantina AI Bots & Party Features**:
   - Fill empty seats up to 6 players with AI bots (*Don Ramón Bot*, *La Catrina Bot*, *Cantinflas Bot*, *El Chavo Bot*, *Frida Bot*, *El Santo Bot*).
   - Private 5-letter room codes, shareable `?room=CODE` links, scannable **📱 QR Code**, bilingual UI (`Español` / `English`), 3 themes, and synthesized Web Audio sound effects.

---

## 🚀 How to Run

```powershell
# Run unit tests
cargo test

# Start server (default port 3000)
cargo run --release
```

Open **[http://localhost:3000](http://localhost:3000)** in your browser (or connect phones on the same Wi-Fi network).

---

## 🐳 Docker & Render.com Deployment

This repository includes an optimized multi-stage [Dockerfile](Dockerfile), a [.dockerignore](.dockerignore), and a [Render Blueprint (`render.yaml`)](render.yaml) configured with `healthCheckPath: /health` and dynamic `PORT` binding.

### Deploy to Render.com (Recommended)

1. Push or connect the `pinguxx/conquian` repository in your [Render Dashboard](https://dashboard.render.com).
2. Create a new **Blueprint** (or **Web Service** with **Docker** runtime) — Render automatically detects `render.yaml`, builds the Docker image, and checks `/health`.

### Build & Run Locally with Docker

```bash
docker build -t conquian:latest .
docker run -p 3000:3000 conquian:latest
```

---

## 📜 License

MIT License — see [LICENSE](LICENSE).
