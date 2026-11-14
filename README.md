# 🃏 Conquián Mexicano Online (2–6 Jugadores) 🇲🇽

A real-time multiplayer web adaptation of the classic Mexican rummy card game **Conquián**, built with **Rust** (`Axum`, `Tokio`, `WebSockets`) and a responsive bilingual frontend supporting **2 to 6 players** with **Baraja Española** or **Poker** suits.

---

## 🌟 Key Features

1. **Multiplayer Scaling (2–6 Players)**:
   - **Auto-Scale Mode (Default)**:
     - **2 players**: 1 Baraja Española (40 cards), 10 cards each, meld **11** to win.
     - **3 players**: 1 Baraja Española (40 cards), 8 cards each, meld **9** to win.
     - **4–6 players**: Double Deck (80 cards), full **10-card hands** for every player, meld **11** to win.
   - Host can also force **Single Deck** (4p plays 7 cards / meld 8) or **Double Deck** for any player count.

2. **Authentic Baraja Española Rules**:
   - Ranks `1 (As), 2, 3, 4, 5, 6, 7, 10 (Sota), 11 (Caballo), 12 (Rey)` — **`7` and `10` are consecutive** in runs (`6-7-10`).
   - Instant visual toggle between traditional **Baraja Española** (*Oros, Copas, Espadas, Bastos*) and **Poker** (`♦ ♥ ♠ ♣`).
   - **Strict Turn-Order Claim Ring**: Priority rotates clockwise around the table one player at a time before any card is burned to *La Basura*.
   - **Table Meld Rearrangement**: Borrow from your own 4-card melds to form new valid sets or runs.
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
