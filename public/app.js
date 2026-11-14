// ¡Conquián Mexicano Online! — Client Application
(function () {
  'use strict';

  const RANK_ORDER_MAP = { 1: 0, 2: 1, 3: 2, 4: 3, 5: 4, 6: 5, 7: 6, 10: 7, 11: 8, 12: 9 };

  const I18N = {
    en: {
      appSubtitle: 'Mexican Rummy • 2–6 Players',
      leaveRoom: 'Leave',
      resumeTitle: 'Game in Progress',
      roomTag: 'ROOM',
      btnResume: '▶️ Resume',
      welcomeTitle: 'Play Conquián at a Multiplayer Table!',
      welcomeDesc: '2 to 6 players with Baraja Española, sets & runs, ¡Obligado!, and rollover Pozo.',
      tabCreate: 'Create Room',
      tabJoin: 'Join Room',
      labelName: 'Your Name or Nickname',
      placeholderHost: 'e.g. Don Pancho',
      placeholderGuest: 'e.g. Lupita',
      labelDeckMode: 'Deck & Hand Scaling (2–6 Players)',
      deckAuto: '✨ Auto-Scale (1 deck 2–3p, 2 decks 4–6p)',
      deckSingle: '🃏 Single 40-Card Deck (2p:10, 3p:8, 4p:7 cards)',
      deckDouble: '🎴 Double 80-Card Deck (Always 10 cards, meld 11)',
      labelChips: 'Betting Chips / El Pozo',
      chipsOff: '🚫 Free Play (No Chips)',
      chips5: '💰 5 Fichas Buy-in',
      chips10: '💰 10 Fichas Buy-in',
      chips20: '💰 20 Fichas Buy-in',
      labelBotSpeed: 'Bot Turn Pace',
      speedFast: '⚡ Fast',
      speedNormal: '⏱️ Normal',
      speedRelaxed: '☕ Relaxed',
      labelObligado: '⚡ Enable "¡Obligado!" (force opponents to meld matching cards)',
      btnCreate: 'Create Table & Play!',
      labelRoomCode: 'Room Code (5 letters)',
      btnJoin: 'Sit at Table!',
      lobbyTitle: 'Cantina Table Lobby',
      codeLabel: 'Code:',
      btnCopy: 'Copy',
      qrTitle: '📱 Scan to Join Table',
      qrSubtitle: 'Point your phone camera to sit at this table immediately',
      playersInRoom: 'Seated Players',
      btnAddBot: '🤖 + Add Bot',
      btnRemoveBot: '🤖 - Remove Bot',
      btnStartGame: '🃏 DEAL CARDS!',
      waitingHost: 'Waiting for host to deal the cards...',
      tableRulesTitle: 'Table Rules & Deck Preview',
      potLabel: 'Pozo',
      stockLabel: 'El Mazo',
      offeredLabel: 'Table Offer',
      noOfferedCard: 'Flip from stock',
      wasteLabel: 'La Basura',
      burnedSub: 'Burned',
      myMeldsTitle: 'My Table Melds',
      btnRearrange: '🔀 Rearrange / Custom Meld',
      noMeldsYet: 'No melds on the table yet. Build sets or runs of 3+!',
      myHandTitle: 'My Hand',
      btnSortSuit: '🎨 Suit',
      btnSortRank: '🔢 Rank',
      btnClearSel: '✖ Clear',
      rearrangeTitle: '🔀 Rearrange Melds / Meld Builder',
      rearrangeGroupsLabel: 'Proposed Table Melds (click a group to select it, then tap cards below):',
      btnNewGroup: '➕ New Group',
      rearrangePoolLabel: 'Available Cards (unassigned):',
      btnConfirmRearrange: '✅ Confirm Melds',
      btnCancel: 'Cancel',
      btnPlayAgain: '🃏 Next Round!',
      btnExitRoom: '🚪 Leave Table',
      waitingNextRound: 'Waiting for host to deal the next hand...',
      rulesHeader: '📖 Conquián Multiplayer Rules',
      btnGotIt: 'Got It!',
      chipsLabel: 'chips',
      yourTurnFlip: '🎴 Your turn! Flip the top card from El Mazo.',
      waitFlip: '⏳ Waiting for {name} to flip from El Mazo...',
      yourPriority: '⭐ Your decision on {card}! Meld it, force ¡Obligado!, or Pass.',
      waitPriority: '⏳ {name} is deciding on {card}...',
      yourDiscard: '🗑️ Select 1 card from your hand to discard (or lay another meld).',
      waitDiscard: '⏳ {name} must discard 1 card...',
      btnFlipStock: '🎴 Flip Stock Card',
      btnMeldSelected: '✨ Meld Selected ({n})',
      btnExtendOwn: '➕ Add to My Meld',
      btnObligadoTo: '⚡ ¡Obligado! → {name}',
      btnPass: '⏭️ Pass',
      btnDiscardSel: '🗑️ Discard Selected',
      btnLayExtra: '🧩 Lay Extra Meld ({n})',
      fromStock: 'From El Mazo',
      fromDiscard: 'Discarded by {name}',
      tablasTitle: '🤝 ¡TABLAS! (TIE HAND)',
      tablasMsg: 'El Mazo ran out! El Pozo ({pot} chips) rolls over into the next deal.',
      winnerTitle: '¡¡CONQUIÁN!!',
      winnerMsg: '🏆 {name} melded {target} cards and took the hand!'
    },
    es: {
      appSubtitle: 'Rummy Mexicano • 2–6 Jugadores',
      leaveRoom: 'Salir',
      resumeTitle: 'Partida en Curso',
      roomTag: 'SALA',
      btnResume: '▶️ Volver',
      welcomeTitle: '¡Juega Conquián en Mesa Multijugador!',
      welcomeDesc: 'De 2 a 6 jugadores con Baraja Española, tercias, corridas, ¡Obligado! y Pozo acumulado.',
      tabCreate: 'Crear Mesa',
      tabJoin: 'Unirse a Mesa',
      labelName: 'Tu Nombre o Apodo',
      placeholderHost: 'ej. Don Pancho',
      placeholderGuest: 'ej. Lupita',
      labelDeckMode: 'Barajas y Cartas por Jugador (2–6)',
      deckAuto: '✨ Automático (1 baraja 2–3j, 2 barajas 4–6j)',
      deckSingle: '🃏 1 Baraja de 40 Cartas (2j:10, 3j:8, 4j:7)',
      deckDouble: '🎴 2 Barajas de 80 Cartas (Siempre 10 cartas, meta 11)',
      labelChips: 'Fichas de Apuesta / El Pozo',
      chipsOff: '🚫 Sin Apuestas (Juego Libre)',
      chips5: '💰 Entrada de 5 Fichas',
      chips10: '💰 Entrada de 10 Fichas',
      chips20: '💰 Entrada de 20 Fichas',
      labelBotSpeed: 'Velocidad de Bots',
      speedFast: '⚡ Rápido',
      speedNormal: '⏱️ Normal',
      speedRelaxed: '☕ Relajado',
      labelObligado: '⚡ Activar "¡Obligado!" (obligar a rivales a bajar carta)',
      btnCreate: '¡Crear Mesa y Jugar!',
      labelRoomCode: 'Código de Sala (5 letras)',
      btnJoin: '¡Sentarse a la Mesa!',
      lobbyTitle: 'Sala de Espera en la Cantina',
      codeLabel: 'Código:',
      btnCopy: 'Copiar',
      qrTitle: '📱 Escanea para Unirte',
      qrSubtitle: 'Apunta la cámara del celular para sentarte en esta mesa al instante',
      playersInRoom: 'Jugadores Sentados',
      btnAddBot: '🤖 + Agregar Bot',
      btnRemoveBot: '🤖 - Quitar Bot',
      btnStartGame: '🃏 ¡REPARTIR CARTAS!',
      waitingHost: 'Esperando a que el anfitrión reparta las cartas...',
      tableRulesTitle: 'Reglas de la Mesa y Baraja',
      potLabel: 'Pozo',
      stockLabel: 'El Mazo',
      offeredLabel: 'Carta en Mesa',
      noOfferedCard: 'Voltea del mazo',
      wasteLabel: 'La Basura',
      burnedSub: 'Quemadas',
      myMeldsTitle: 'Mis Juegos en Mesa',
      btnRearrange: '🔀 Acomodar Juegos',
      noMeldsYet: 'Aún no bajas juegos. ¡Forma tercias o corridas de 3+!',
      myHandTitle: 'Mi Mano',
      btnSortSuit: '🎨 Palo',
      btnSortRank: '🔢 Número',
      btnClearSel: '✖ Limpiar',
      rearrangeTitle: '🔀 Acomodar Juegos / Constructor',
      rearrangeGroupsLabel: 'Juegos Propuestos (toca un grupo para seleccionarlo y toca cartas abajo):',
      btnNewGroup: '➕ Nuevo Juego',
      rearrangePoolLabel: 'Cartas Disponibles (sin asignar):',
      btnConfirmRearrange: '✅ Confirmar Juegos',
      btnCancel: 'Cancelar',
      btnPlayAgain: '🃏 ¡Siguiente Mano!',
      btnExitRoom: '🚪 Salir al Menú',
      waitingNextRound: 'Esperando a que el anfitrión reparta otra mano...',
      rulesHeader: '📖 Reglas de Conquián Multijugador',
      btnGotIt: '¡Entendido!',
      chipsLabel: 'fichas',
      yourTurnFlip: '🎴 ¡Tu turno! Voltea la carta de arriba de El Mazo.',
      waitFlip: '⏳ Esperando a que {name} voltee de El Mazo...',
      yourPriority: '⭐ ¡Te toca decidir por {card}! Bájala, aplica ¡Obligado! o Pasa.',
      waitPriority: '⏳ {name} está decidiendo sobre {card}...',
      yourDiscard: '🗑️ Selecciona 1 carta de tu mano para descartar (o baja otro juego).',
      waitDiscard: '⏳ {name} debe descartar 1 carta...',
      btnFlipStock: '🎴 Voltear de El Mazo',
      btnMeldSelected: '✨ Bajar Juego ({n})',
      btnExtendOwn: '➕ Agregar a Mi Juego',
      btnObligadoTo: '⚡ ¡Obligado! → {name}',
      btnPass: '⏭️ Pasar',
      btnDiscardSel: '🗑️ Descartar Seleccionada',
      btnLayExtra: '🧩 Bajar Otro Juego ({n})',
      fromStock: 'De El Mazo',
      fromDiscard: 'Descarte de {name}',
      tablasTitle: '🤝 ¡JUEGO TABLAS!',
      tablasMsg: '¡Se acabó El Mazo! El Pozo ({pot} fichas) se acumula para la siguiente mano.',
      winnerTitle: '¡¡CONQUIÁN!!',
      winnerMsg: '🏆 ¡{name} bajó {target} cartas y ganó la mano!'
    }
  };

  // Client State
  let ws = null;
  let lang = localStorage.getItem('conquian_lang') || 'es';
  let deckStyle = localStorage.getItem('conquian_deck_style') || 'baraja'; // 'baraja' | 'poker'
  let themeIndex = Number(localStorage.getItem('conquian_theme_idx') || 0);
  const THEMES = ['theme-cantina', 'theme-muertos', 'theme-vintage'];

  let roomCode = null;
  let playerId = null;
  let playerName = '';
  let gameState = null;
  let selectedHandIds = new Set();
  let handSortMode = 'suit';
  let lastNoticeSignature = '';
  let lastActorSignature = '';
  let lastStateKind = '';
  let deferredInstallPrompt = null;

  // Rearrange modal state
  let rearrangeGroups = []; // Array of Array<card>
  let rearrangePool = [];   // Array<card>
  let rearrangeActiveGroupIdx = 0;

  // DOM helpers
  const $ = (id) => document.getElementById(id);

  function t(key, vars = {}) {
    let text = (I18N[lang] && I18N[lang][key]) || I18N.en[key] || key;
    for (const [k, v] of Object.entries(vars)) {
      text = text.replace(`{${k}}`, v);
    }
    return text;
  }

  function applyTranslations() {
    document.querySelectorAll('[data-i18n]').forEach((el) => {
      const key = el.getAttribute('data-i18n');
      el.textContent = t(key);
    });
    document.querySelectorAll('[data-i18n-ph]').forEach((el) => {
      const key = el.getAttribute('data-i18n-ph');
      el.setAttribute('placeholder', t(key));
    });
    $('btn-toggle-lang').textContent = lang === 'es' ? '🌐 English' : '🌐 Español';
    $('btn-toggle-deck-style').textContent = deckStyle === 'baraja' ? '🎴 Baraja' : '♠️ Poker';
    renderRulesBody();
    if (gameState) {
      if (gameState.state === 'lobby') renderLobby();
      else renderGameTable();
    }
  }

  function showToast(msg, type = 'info') {
    const container = $('toast-container');
    const div = document.createElement('div');
    div.className = `toast ${type === 'error' ? 'toast-error' : type === 'success' ? 'toast-success' : ''}`;
    div.textContent = msg;
    container.appendChild(div);
    setTimeout(() => div.remove(), 3500);
  }

  function switchScreen(name) {
    document.querySelectorAll('.screen').forEach((s) => s.classList.remove('active'));
    $(`screen-${name}`).classList.add('active');
    $('btn-header-leave').style.display = name === 'welcome' ? 'none' : 'inline-flex';
  }

  // Suit & Rank formatting
  function suitVisual(suit) {
    if (deckStyle === 'poker') {
      switch (suit) {
        case 'oros': return { symbol: '♦', label: 'Diamonds', cls: 'suit-oros' };
        case 'copas': return { symbol: '♥', label: 'Hearts', cls: 'suit-copas' };
        case 'espadas': return { symbol: '♠', label: 'Spades', cls: 'suit-espadas' };
        case 'bastos': return { symbol: '♣', label: 'Clubs', cls: 'suit-bastos' };
      }
    }
    switch (suit) {
      case 'oros': return { symbol: '🪙', label: 'Oros', cls: 'suit-oros' };
      case 'copas': return { symbol: '🏆', label: 'Copas', cls: 'suit-copas' };
      case 'espadas': return { symbol: '⚔️', label: 'Espadas', cls: 'suit-espadas' };
      case 'bastos': return { symbol: '🪵', label: 'Bastos', cls: 'suit-bastos' };
    }
    return { symbol: '?', label: '', cls: '' };
  }

  function rankDisplay(rank) {
    if (deckStyle === 'poker') {
      if (rank === 1) return { short: 'A', badge: '' };
      if (rank === 10) return { short: 'J', badge: 'Jack' };
      if (rank === 11) return { short: 'Q', badge: 'Queen' };
      if (rank === 12) return { short: 'K', badge: 'King' };
      return { short: String(rank), badge: '' };
    }
    if (rank === 1) return { short: '1', badge: 'As' };
    if (rank === 10) return { short: '10', badge: 'Sota' };
    if (rank === 11) return { short: '11', badge: 'Cab.' };
    if (rank === 12) return { short: '12', badge: 'Rey' };
    return { short: String(rank), badge: '' };
  }

  function cardLabel(card) {
    const r = rankDisplay(card.rank);
    const s = suitVisual(card.suit);
    return `${r.short}${s.symbol}`;
  }

  function renderCardElement(card, size = '', extraOpts = {}) {
    const sv = suitVisual(card.suit);
    const rd = rankDisplay(card.rank);
    const el = document.createElement('div');
    el.className = `playing-card ${size} ${sv.cls}`.trim();
    if (extraOpts.selected) el.classList.add('selected');
    if (extraOpts.synergy) el.classList.add('synergy-hint');
    el.dataset.cardId = String(card.id);

    el.innerHTML = `
      <div class="card-corner">
        <span class="card-rank">${rd.short}</span>
        <span class="card-suit-mini">${sv.symbol}</span>
      </div>
      ${rd.badge ? `<span class="card-court-badge">${rd.badge}</span>` : ''}
      <div class="card-center-art">${sv.symbol}</div>
      <div class="card-corner card-corner-bottom">
        <span class="card-rank">${rd.short}</span>
        <span class="card-suit-mini">${sv.symbol}</span>
      </div>
    `;
    return el;
  }

  // Client-side Conquián meld validator (matches Rust backend logic)
  function isValidMeld(cards) {
    if (!cards || cards.length < 3) return false;
    const ids = new Set(cards.map(c => c.id));
    if (ids.size !== cards.length) return false;

    // Set check
    if (cards.length <= 4) {
      const rank = cards[0].rank;
      if (cards.every(c => c.rank === rank)) {
        const suits = new Set(cards.map(c => c.suit));
        if (suits.size === cards.length) return true;
      }
    }

    // Run check
    if (cards.length <= 10) {
      const suit = cards[0].suit;
      if (cards.every(c => c.suit === suit)) {
        const orders = cards.map(c => RANK_ORDER_MAP[c.rank]).sort((a, b) => a - b);
        let ok = true;
        for (let i = 1; i < orders.length; i++) {
          if (orders[i] !== orders[i - 1] + 1) {
            ok = false;
            break;
          }
        }
        if (ok) return true;
      }
    }

    return false;
  }

  function computeSynergyCardIds(hand, offeredCard) {
    const synergetic = new Set();
    if (!offeredCard || !hand) return synergetic;
    for (let i = 0; i < hand.length; i++) {
      for (let j = i + 1; j < hand.length; j++) {
        if (isValidMeld([offeredCard, hand[i], hand[j]])) {
          synergetic.add(hand[i].id);
          synergetic.add(hand[j].id);
        }
      }
    }
    return synergetic;
  }

  function resolveScalingClient(count, mode) {
    if (count >= 5) return { isDouble: true, handSize: 10, target: 11 };
    if (mode === 'double') return { isDouble: true, handSize: 10, target: 11 };
    if (mode === 'single') {
      if (count <= 2) return { isDouble: false, handSize: 10, target: 11 };
      if (count === 3) return { isDouble: false, handSize: 8, target: 9 };
      return { isDouble: false, handSize: 7, target: 8 };
    }
    if (count <= 2) return { isDouble: false, handSize: 10, target: 11 };
    if (count === 3) return { isDouble: false, handSize: 8, target: 9 };
    return { isDouble: true, handSize: 10, target: 11 };
  }

  // WebSocket Management
  function connectWS(onOpenCallback) {
    if (ws && ws.readyState === WebSocket.OPEN) {
      if (onOpenCallback) onOpenCallback();
      return;
    }

    const proto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    ws = new WebSocket(`${proto}//${window.location.host}/ws`);

    ws.onopen = () => {
      if (onOpenCallback) onOpenCallback();
    };

    ws.onmessage = (evt) => {
      try {
        const msg = JSON.parse(evt.data);
        handleServerEvent(msg.event, msg.data);
      } catch (e) {
        console.error('WS parse error:', e);
      }
    };

    ws.onclose = () => {
      // Do not wipe session; user can click Resume if dropped
    };
  }

  function sendWS(type, payload = {}) {
    if (!ws || ws.readyState !== WebSocket.OPEN) {
      connectWS(() => {
        ws.send(JSON.stringify({ type, ...payload }));
      });
      return;
    }
    ws.send(JSON.stringify({ type, ...payload }));
  }

  function saveSession() {
    if (roomCode && playerId) {
      localStorage.setItem('conquian_session', JSON.stringify({ roomCode, playerId, playerName }));
    }
  }

  function clearSession() {
    localStorage.removeItem('conquian_session');
    $('resume-banner').style.display = 'none';
  }

  function handleServerEvent(event, data) {
    switch (event) {
      case 'room_created':
      case 'joined_room':
        roomCode = data.roomCode;
        playerId = data.playerId;
        saveSession();
        switchScreen('lobby');
        break;

      case 'session_restored':
        roomCode = data.roomCode;
        playerId = data.myId;
        gameState = data;
        saveSession();
        if (data.state === 'lobby') {
          switchScreen('lobby');
          renderLobby();
        } else {
          switchScreen('game');
          renderGameTable();
        }
        break;

      case 'reconnect_failed':
        clearSession();
        showToast(data.message || 'Session expired', 'error');
        break;

      case 'join_error':
      case 'action_error':
        showToast(data.message || 'Error', 'error');
        break;

      case 'left_room':
        clearSession();
        roomCode = null;
        playerId = null;
        gameState = null;
        selectedHandIds.clear();
        $('winner-modal').classList.remove('open');
        switchScreen('welcome');
        break;

      case 'game_started':
        selectedHandIds.clear();
        $('winner-modal').classList.remove('open');
        switchScreen('game');
        break;

      case 'new_reaction':
        showToast(`${data.sender}: ${data.emoji}`);
        break;

      case 'room_state':
        handleRoomStateUpdate(data);
        break;
    }
  }

  function handleRoomStateUpdate(data) {
    const prevState = gameState ? gameState.state : '';
    gameState = data;
    playerId = data.myId;
    roomCode = data.roomCode;

    if (data.state === 'lobby') {
      $('winner-modal').classList.remove('open');
      switchScreen('lobby');
      renderLobby();
    } else if (data.state === 'playing') {
      $('winner-modal').classList.remove('open');
      switchScreen('game');
      renderGameTable();
      triggerAudioFeedback(data, prevState);
    } else if (data.state === 'ended') {
      switchScreen('game');
      renderGameTable();
      renderWinnerModal();
      if (lastStateKind !== 'ended') {
        if (data.isTie) {
          window.conquianAudio.playTie();
        } else {
          window.conquianAudio.playFanfare();
          if (typeof window.confetti === 'function') {
            window.confetti({ particleCount: 110, spread: 80, origin: { y: 0.6 } });
          }
        }
      }
    }
    lastStateKind = data.state;
  }

  function triggerAudioFeedback(data, prevState) {
    if (data.lastNotice) {
      const sig = `${data.lastNotice.kind}:${data.lastNotice.messageEs}`;
      if (sig !== lastNoticeSignature) {
        lastNoticeSignature = sig;
        switch (data.lastNotice.kind) {
          case 'flip': window.conquianAudio.playFlip(); break;
          case 'meld': window.conquianAudio.playMeld(); break;
          case 'obligado': window.conquianAudio.playObligado(); break;
          case 'discard': window.conquianAudio.playDiscard(); break;
          case 'pass':
          case 'burn': window.conquianAudio.playPass(); break;
        }
        const msg = lang === 'es' ? data.lastNotice.messageEs : data.lastNotice.messageEn;
        if (data.lastNotice.kind === 'obligado' || data.lastNotice.kind === 'start') {
          window.conquianAudio.speak(msg, lang);
        }
      }
    }

    const actorSig = `${data.phase}:${data.activeActorId}:${data.offeredCard ? data.offeredCard.id : 'none'}`;
    if (actorSig !== lastActorSignature) {
      lastActorSignature = actorSig;
      if (data.activeActorId === playerId && prevState === 'playing') {
        window.conquianAudio.playYourTurn();
      }
    }
  }

  // Render Lobby
  function renderLobby() {
    if (!gameState) return;
    $('lobby-room-code').textContent = gameState.roomCode;
    const players = gameState.players || [];
    const isHost = gameState.hostId === playerId;
    const settings = gameState.settings || {};

    $('lobby-player-count').textContent = players.length;

    const scaling = resolveScalingClient(Math.max(2, players.length), settings.deckMode);
    const deckLabel = scaling.isDouble
      ? (lang === 'es' ? '2 Barajas (80)' : '2 Decks (80)')
      : (lang === 'es' ? '1 Baraja (40)' : '1 Deck (40)');
    $('lobby-scaling-badge').textContent = `${deckLabel} • ${scaling.handSize}→${scaling.target}`;

    // Render players
    const listEl = $('lobby-players-list');
    listEl.innerHTML = '';
    players.forEach((p, idx) => {
      const row = document.createElement('div');
      row.className = 'player-row';
      row.innerHTML = `
        <div>
          <strong>#${idx + 1} ${escapeHtml(p.name)}</strong>
          ${p.isHost ? '<span class="rule-badge" style="margin-left:6px;">HOST</span>' : ''}
          ${p.id === playerId ? `<span class="rule-badge" style="margin-left:6px;background:rgba(34,197,94,0.2);border-color:#4ade80;color:#bbf7d0;">${lang === 'es' ? 'TÚ' : 'YOU'}</span>` : ''}
        </div>
        <div style="font-size:0.82rem;color:#fde047;font-weight:700;">
          💰 ${p.chips} • 🏆 ${p.wins}
        </div>
      `;
      listEl.appendChild(row);
    });

    // Rules summary
    const obligadoStr = settings.allowObligado
      ? (lang === 'es' ? '✅ ¡Obligado! Activado' : '✅ ¡Obligado! Enabled')
      : (lang === 'es' ? '🚫 ¡Obligado! Desactivado' : '🚫 ¡Obligado! Disabled');
    const chipsStr = settings.enableChips && settings.buyIn > 0
      ? (lang === 'es' ? `💰 Pozo: ${settings.buyIn} fichas/ronda (acumulable en Tablas)` : `💰 Pot: ${settings.buyIn} chips/round (rollover on Tie)`)
      : (lang === 'es' ? '🆓 Juego libre sin apuestas' : '🆓 Free Play (no chips)');

    $('lobby-rules-summary').innerHTML = lang === 'es'
      ? `<div>🃏 <strong>${deckLabel}</strong> (${players.length} jugadores)</div>
         <div>🖐️ Se reparten <strong>${scaling.handSize} cartas</strong> — gana quien baje <strong>${scaling.target} cartas</strong></div>
         <div>${obligadoStr}</div>
         <div>${chipsStr}</div>
         ${gameState.pot > 0 ? `<div style="color:#fde047;margin-top:6px;">🔥 <strong>Pozo acumulado actual: ${gameState.pot} fichas</strong></div>` : ''}`
      : `<div>🃏 <strong>${deckLabel}</strong> (${players.length} players)</div>
         <div>🖐️ Deal <strong>${scaling.handSize} cards</strong> each — win with <strong>${scaling.target} melded</strong></div>
         <div>${obligadoStr}</div>
         <div>${chipsStr}</div>
         ${gameState.pot > 0 ? `<div style="color:#fde047;margin-top:6px;">🔥 <strong>Current rollover pot: ${gameState.pot} chips</strong></div>` : ''}`;

    $('lobby-bot-controls').style.display = isHost ? 'flex' : 'none';
    $('lobby-host-controls').style.display = isHost ? 'block' : 'none';
    $('lobby-host-settings').style.display = isHost ? 'block' : 'none';
    $('lobby-guest-msg').style.display = isHost ? 'none' : 'block';
    $('btn-start-game').disabled = players.length < 2;

    if (isHost) {
      $('lobby-deck-mode').value = settings.deckMode || 'auto';
      $('lobby-chips').value = settings.enableChips ? String(settings.buyIn) : '0';
      $('lobby-obligado').checked = !!settings.allowObligado;
    }

    renderQrCode();
  }

  function renderQrCode() {
    const qrPanel = $('lobby-qr-card');
    if (qrPanel.style.display === 'none' || !roomCode) return;
    const qrWrap = $('lobby-qrcode');
    qrWrap.innerHTML = '';
    const url = `${window.location.origin}/?room=${roomCode}`;
    if (typeof window.QRCode === 'function') {
      new window.QRCode(qrWrap, {
        text: url,
        width: 156,
        height: 156,
        colorDark: '#0f172a',
        colorLight: '#ffffff'
      });
    }
  }

  // Render Live Game Table
  function renderGameTable() {
    if (!gameState) return;
    const players = gameState.players || [];
    const myPlayer = players.find(p => p.id === playerId);
    const myMelds = myPlayer ? myPlayer.melds : [];
    const hand = sortCardsList(gameState.myHand || []);

    // Clean stale selections
    const handIdsSet = new Set(hand.map(c => c.id));
    for (const id of Array.from(selectedHandIds)) {
      if (!handIdsSet.has(id)) selectedHandIds.delete(id);
    }

    $('game-room-code-tag').textContent = gameState.roomCode;
    $('game-round-badge').textContent = `${lang === 'es' ? 'Ronda' : 'Round'} #${gameState.roundNumber}`;
    $('game-target-badge').textContent = `${lang === 'es' ? 'Meta' : 'Target'}: ${gameState.targetMeldCount}`;

    if (gameState.settings && gameState.settings.enableChips) {
      $('game-pot-badge').style.display = 'inline-flex';
      $('game-pot-val').textContent = gameState.pot;
      $('game-rollover-tag').style.display = gameState.potRolledOver ? 'inline-flex' : 'none';
    } else {
      $('game-pot-badge').style.display = 'none';
      $('game-rollover-tag').style.display = 'none';
    }

    // Notice banner
    if (gameState.lastNotice) {
      $('game-notice-banner').textContent =
        lang === 'es' ? gameState.lastNotice.messageEs : gameState.lastNotice.messageEn;
    }

    renderOpponentsRing(players);
    renderCenterTable(players);
    renderActionBar(players, hand, myMelds);
    renderMyMelds(myMelds, myPlayer ? myPlayer.meldedCount : 0);
    renderMyHand(hand);
  }

  function renderOpponentsRing(players) {
    const ring = $('opponents-ring');
    ring.innerHTML = '';

    const myIdx = players.findIndex(p => p.id === playerId);
    const orderedOpponents = [];
    if (myIdx >= 0) {
      for (let offset = 1; offset < players.length; offset++) {
        orderedOpponents.push(players[(myIdx + offset) % players.length]);
      }
    } else {
      orderedOpponents.push(...players);
    }

    const isMyPriority =
      gameState.phase === 'claiming_offer' && gameState.claimTurnPlayerId === playerId;
    const obligadoMap = new Map();
    if (isMyPriority && gameState.obligadoTargets) {
      gameState.obligadoTargets.forEach((ot) => {
        obligadoMap.set(`${ot.targetPlayerId}:${ot.meldId}`, ot);
      });
    }

    orderedOpponents.forEach((opp) => {
      const isActive = gameState.activeActorId === opp.id;
      const seat = document.createElement('div');
      seat.className = `opponent-seat ${isActive ? 'is-active' : ''} ${!opp.connected ? 'is-disconnected' : ''}`;

      let statusTag = '';
      if (isActive) {
        if (gameState.phase === 'awaiting_flip') statusTag = lang === 'es' ? '🎴 Voltea' : '🎴 Flips';
        else if (gameState.phase === 'claiming_offer') statusTag = lang === 'es' ? '⭐ Decide' : '⭐ Deciding';
        else if (gameState.phase === 'must_discard') statusTag = lang === 'es' ? '🗑️ Descarta' : '🗑️ Discards';
      }

      const head = document.createElement('div');
      head.className = 'opponent-head';
      head.innerHTML = `
        <span class="opponent-name">${escapeHtml(opp.name)}</span>
        ${statusTag ? `<span class="seat-status-tag">${statusTag}</span>` : ''}
      `;
      seat.appendChild(head);

      const stats = document.createElement('div');
      stats.className = 'opponent-stats';
      stats.innerHTML = `
        <span>🖐️ ${opp.handCount}</span>
        <span>🃏 ${opp.meldedCount}/${gameState.targetMeldCount}</span>
        <span>💰 ${opp.chips}</span>
        <span>🏆 ${opp.wins}</span>
      `;
      seat.appendChild(stats);

      const meldsWrap = document.createElement('div');
      meldsWrap.className = 'opponent-melds';
      (opp.melds || []).forEach((meld) => {
        const groupEl = document.createElement('div');
        const obKey = `${opp.id}:${meld.id}`;
        const isObligadoTarget = obligadoMap.has(obKey);
        groupEl.className = `table-meld-group ${isObligadoTarget ? 'obligado-target' : ''}`;
        if (isObligadoTarget) {
          groupEl.title = lang === 'es' ? '¡Click para aplicar ¡Obligado!!' : 'Click to force ¡Obligado!';
          groupEl.addEventListener('click', () => {
            sendWS('force_obligado', {
              roomCode,
              targetPlayerId: opp.id,
              targetMeldId: meld.id
            });
          });
        }
        meld.cards.forEach((c) => {
          groupEl.appendChild(renderCardElement(c, 'xs'));
        });
        meldsWrap.appendChild(groupEl);
      });
      seat.appendChild(meldsWrap);

      ring.appendChild(seat);
    });
  }

  function renderCenterTable(players) {
    $('stock-count-val').textContent = gameState.stockCount;
    $('waste-count-val').textContent = gameState.wasteCount;

    const canFlip =
      gameState.state === 'playing' &&
      gameState.phase === 'awaiting_flip' &&
      gameState.activeActorId === playerId;

    const stockBtn = $('stock-pile-card');
    stockBtn.classList.toggle('clickable', canFlip);
    const turnPlayer = players.find(p => p.id === gameState.currentTurnPlayerId);
    $('stock-turn-hint').textContent = canFlip
      ? (lang === 'es' ? '¡Toca para voltear!' : 'Click to flip!')
      : (turnPlayer ? turnPlayer.name : '---');

    // Offered card slot
    const offerSlot = $('offered-card-slot');
    offerSlot.innerHTML = '';
    const sourceLabel = $('offered-source-label');

    if (gameState.offeredCard) {
      offerSlot.classList.remove('empty');
      offerSlot.classList.add('has-card');
      offerSlot.appendChild(renderCardElement(gameState.offeredCard, 'lg'));
      sourceLabel.textContent =
        gameState.offeredFrom === 'discard'
          ? t('fromDiscard', { name: gameState.offeredByName || '?' })
          : t('fromStock');
    } else {
      offerSlot.classList.add('empty');
      offerSlot.classList.remove('has-card');
      const span = document.createElement('span');
      span.className = 'empty-offer-text';
      span.textContent = t('noOfferedCard');
      offerSlot.appendChild(span);
      sourceLabel.textContent = t('offeredLabel');
    }

    // Priority ring strip
    const ringStrip = $('claim-ring-strip');
    ringStrip.innerHTML = '';
    if (gameState.phase === 'claiming_offer' && gameState.offeredCard) {
      const passedSet = new Set(gameState.passedPlayerIds || []);
      players.forEach((p) => {
        if (gameState.offeredFrom === 'discard' && gameState.offeredBy === p.id) {
          return; // Discarder does not claim their own discard
        }
        const chip = document.createElement('span');
        const isCurrent = gameState.claimTurnPlayerId === p.id;
        const hasPassed = passedSet.has(p.id);
        chip.className = `ring-chip ${isCurrent ? 'active' : ''} ${hasPassed ? 'passed' : ''}`;
        chip.textContent = isCurrent ? `👉 ${p.name}` : hasPassed ? `✓ ${p.name}` : p.name;
        ringStrip.appendChild(chip);
      });
    }
  }

  function renderActionBar(players, hand, myMelds) {
    const promptEl = $('action-prompt');
    const btnsEl = $('action-buttons');
    btnsEl.innerHTML = '';

    if (gameState.state !== 'playing') {
      promptEl.textContent = lang === 'es' ? 'Mano terminada.' : 'Hand complete.';
      return;
    }

    const activePlayer = players.find(p => p.id === gameState.activeActorId);
    const activeName = activePlayer ? activePlayer.name : '...';
    const selectedCards = hand.filter(c => selectedHandIds.has(c.id));

    if (gameState.phase === 'awaiting_flip') {
      if (gameState.activeActorId === playerId) {
        promptEl.textContent = t('yourTurnFlip');
        const btn = document.createElement('button');
        btn.className = 'btn-action-emerald';
        btn.textContent = t('btnFlipStock');
        btn.addEventListener('click', () => sendWS('flip_stock', { roomCode }));
        btnsEl.appendChild(btn);
      } else {
        promptEl.textContent = t('waitFlip', { name: activeName });
      }
      return;
    }

    if (gameState.phase === 'claiming_offer') {
      const cardStr = gameState.offeredCard ? cardLabel(gameState.offeredCard) : '';
      if (gameState.claimTurnPlayerId === playerId) {
        promptEl.textContent = t('yourPriority', { card: cardStr });

        // 1. Meld with selected hand cards + offeredCard
        const meldBtn = document.createElement('button');
        meldBtn.className = 'btn-action-emerald';
        meldBtn.textContent = t('btnMeldSelected', { n: selectedCards.length });
        const candidateCards = gameState.offeredCard ? [gameState.offeredCard, ...selectedCards] : [];
        const canDirectMeld = selectedCards.length >= 2 && isValidMeld(candidateCards);
        meldBtn.disabled = !canDirectMeld;
        meldBtn.addEventListener('click', () => {
          const proposedMelds = myMelds.map(m => m.cards.map(c => c.id));
          proposedMelds.push(candidateCards.map(c => c.id));
          sendWS('meld_card', {
            roomCode,
            usedHandIds: selectedCards.map(c => c.id),
            proposedMelds
          });
          selectedHandIds.clear();
        });
        btnsEl.appendChild(meldBtn);

        // 2. 1-Click extend own melds
        (gameState.canExtendMyMelds || []).forEach((meldId) => {
          const extBtn = document.createElement('button');
          extBtn.className = 'btn-action-emerald';
          extBtn.textContent = t('btnExtendOwn');
          extBtn.addEventListener('click', () => {
            sendWS('meld_card', { roomCode, extendMeldId: meldId });
            selectedHandIds.clear();
          });
          btnsEl.appendChild(extBtn);
        });

        // 3. ¡Obligado! buttons
        (gameState.obligadoTargets || []).forEach((ot) => {
          const obBtn = document.createElement('button');
          obBtn.className = 'btn-action-obligado';
          obBtn.textContent = t('btnObligadoTo', { name: ot.targetPlayerName });
          obBtn.addEventListener('click', () => {
            sendWS('force_obligado', {
              roomCode,
              targetPlayerId: ot.targetPlayerId,
              targetMeldId: ot.meldId
            });
            selectedHandIds.clear();
          });
          btnsEl.appendChild(obBtn);
        });

        // 4. Pass button
        const passBtn = document.createElement('button');
        passBtn.className = 'btn-action-pass';
        passBtn.textContent = t('btnPass');
        passBtn.addEventListener('click', () => {
          selectedHandIds.clear();
          sendWS('pass_card', { roomCode });
        });
        btnsEl.appendChild(passBtn);
      } else {
        promptEl.textContent = t('waitPriority', { name: activeName, card: cardStr });
      }
      return;
    }

    if (gameState.phase === 'must_discard') {
      if (gameState.mustDiscardPlayerId === playerId) {
        promptEl.textContent = t('yourDiscard');

        const discBtn = document.createElement('button');
        discBtn.className = 'btn-primary';
        discBtn.style.width = 'auto';
        discBtn.style.padding = '10px 16px';
        discBtn.textContent = t('btnDiscardSel');
        discBtn.disabled = selectedCards.length !== 1;
        discBtn.addEventListener('click', () => {
          if (selectedCards.length === 1) {
            sendWS('discard_card', { roomCode, cardId: selectedCards[0].id });
            selectedHandIds.clear();
          }
        });
        btnsEl.appendChild(discBtn);

        // Allow laying down an extra pure-hand meld before discarding
        if (selectedCards.length >= 3 && isValidMeld(selectedCards)) {
          const extraBtn = document.createElement('button');
          extraBtn.className = 'btn-action-emerald';
          extraBtn.textContent = t('btnLayExtra', { n: selectedCards.length });
          extraBtn.addEventListener('click', () => {
            const proposedMelds = myMelds.map(m => m.cards.map(c => c.id));
            proposedMelds.push(selectedCards.map(c => c.id));
            sendWS('rearrange_melds', {
              roomCode,
              usedHandIds: selectedCards.map(c => c.id),
              proposedMelds
            });
            selectedHandIds.clear();
          });
          btnsEl.appendChild(extraBtn);
        }
      } else {
        promptEl.textContent = t('waitDiscard', { name: activeName });
      }
    }
  }

  function renderMyMelds(melds, meldedCount) {
    $('my-melds-count').textContent = meldedCount;
    $('my-target-count').textContent = gameState.targetMeldCount;

    const canRearrange =
      gameState.state === 'playing' &&
      ((gameState.phase === 'claiming_offer' && gameState.claimTurnPlayerId === playerId) ||
        (gameState.phase === 'must_discard' && gameState.mustDiscardPlayerId === playerId));
    $('btn-open-rearrange').style.display = canRearrange ? 'inline-flex' : 'none';

    const listEl = $('my-melds-list');
    listEl.innerHTML = '';

    if (!melds || melds.length === 0) {
      const msg = document.createElement('div');
      msg.className = 'empty-melds-msg';
      msg.textContent = t('noMeldsYet');
      listEl.appendChild(msg);
      return;
    }

    const canExtendSet = new Set(
      gameState.phase === 'claiming_offer' && gameState.claimTurnPlayerId === playerId
        ? gameState.canExtendMyMelds || []
        : []
    );

    melds.forEach((meld) => {
      const group = document.createElement('div');
      const isExt = canExtendSet.has(meld.id);
      group.className = `table-meld-group ${isExt ? 'extendable-own' : ''}`;
      if (isExt) {
        group.title = lang === 'es' ? 'Click para agregar carta en mesa' : 'Click to add offered card';
        group.addEventListener('click', () => {
          sendWS('meld_card', { roomCode, extendMeldId: meld.id });
          selectedHandIds.clear();
        });
      }
      meld.cards.forEach(c => group.appendChild(renderCardElement(c, 'sm')));
      listEl.appendChild(group);
    });
  }

  function sortCardsList(cards) {
    const copy = [...cards];
    if (handSortMode === 'rank') {
      copy.sort((a, b) => (RANK_ORDER_MAP[a.rank] - RANK_ORDER_MAP[b.rank]) || a.suit.localeCompare(b.suit));
    } else {
      const suitOrder = { oros: 0, copas: 1, espadas: 2, bastos: 3 };
      copy.sort((a, b) => (suitOrder[a.suit] - suitOrder[b.suit]) || (RANK_ORDER_MAP[a.rank] - RANK_ORDER_MAP[b.rank]));
    }
    return copy;
  }

  function renderMyHand(hand) {
    $('my-hand-count').textContent = hand.length;
    $('btn-clear-selection').style.display = selectedHandIds.size > 0 ? 'inline-flex' : 'none';

    const tray = $('my-hand-cards');
    tray.innerHTML = '';

    const showSynergy =
      gameState.phase === 'claiming_offer' &&
      gameState.claimTurnPlayerId === playerId &&
      gameState.offeredCard;
    const synergyIds = showSynergy
      ? computeSynergyCardIds(hand, gameState.offeredCard)
      : new Set();

    hand.forEach((card) => {
      const el = renderCardElement(card, '', {
        selected: selectedHandIds.has(card.id),
        synergy: synergyIds.has(card.id)
      });

      el.addEventListener('click', () => {
        if (selectedHandIds.has(card.id)) {
          selectedHandIds.delete(card.id);
        } else {
          selectedHandIds.add(card.id);
        }
        renderGameTable();
      });

      // Quick double-click to discard during MustDiscard
      el.addEventListener('dblclick', () => {
        if (
          gameState.state === 'playing' &&
          gameState.phase === 'must_discard' &&
          gameState.mustDiscardPlayerId === playerId
        ) {
          selectedHandIds.clear();
          sendWS('discard_card', { roomCode, cardId: card.id });
        }
      });

      tray.appendChild(el);
    });
  }

  // Interactive Rearrange / Custom Meld Builder Modal
  function openRearrangeModal() {
    if (!gameState) return;
    const players = gameState.players || [];
    const me = players.find(p => p.id === playerId);
    const myMelds = me ? me.melds : [];
    const hand = gameState.myHand || [];

    rearrangeGroups = myMelds.map(m => [...m.cards]);
    rearrangePool = [...hand];
    if (gameState.phase === 'claiming_offer' && gameState.offeredCard) {
      rearrangePool.push(gameState.offeredCard);
    }
    rearrangeGroups.push([]);
    rearrangeActiveGroupIdx = rearrangeGroups.length - 1;

    $('rearrange-subtitle').textContent =
      gameState.phase === 'claiming_offer'
        ? (lang === 'es'
            ? 'Puedes mover cartas de tus juegos de 4+, incluir la carta en mesa y cartas de tu mano.'
            : 'Rearrange your table melds, include the offered card, and add cards from your hand.')
        : (lang === 'es'
            ? 'Acomoda tus juegos en mesa o baja juegos adicionales de tu mano antes de descartar.'
            : 'Rearrange your table melds or lay extra melds from your hand before discarding.');

    renderRearrangeModal();
    $('rearrange-modal').classList.add('open');
  }

  function renderRearrangeModal() {
    const groupsEl = $('rearrange-groups-list');
    const poolEl = $('rearrange-pool-cards');
    const msgEl = $('rearrange-validation-msg');
    groupsEl.innerHTML = '';
    poolEl.innerHTML = '';

    const me = (gameState.players || []).find(p => p.id === playerId);
    const existingIds = new Set();
    (me ? me.melds : []).forEach(m => m.cards.forEach(c => existingIds.add(c.id)));
    const offeredId =
      gameState.phase === 'claiming_offer' && gameState.offeredCard
        ? gameState.offeredCard.id
        : null;

    rearrangeGroups.forEach((grp, gIdx) => {
      const box = document.createElement('div');
      const valid = grp.length >= 3 && isValidMeld(grp);
      box.className = `rearrange-group-box ${gIdx === rearrangeActiveGroupIdx ? 'active-group' : ''} ${valid ? 'valid-group' : ''}`;
      box.addEventListener('click', () => {
        rearrangeActiveGroupIdx = gIdx;
        renderRearrangeModal();
      });

      if (grp.length === 0) {
        box.innerHTML = `<span style="font-size:0.78rem;color:#94a3b8;">${lang === 'es' ? `Grupo #${gIdx + 1} (toca cartas abajo para añadir)` : `Group #${gIdx + 1} (tap cards below to add)`}</span>`;
      } else {
        grp.forEach((card, cIdx) => {
          const cardEl = renderCardElement(card, 'sm');
          cardEl.addEventListener('click', (e) => {
            e.stopPropagation();
            grp.splice(cIdx, 1);
            rearrangePool.push(card);
            renderRearrangeModal();
          });
          box.appendChild(cardEl);
        });
      }
      groupsEl.appendChild(box);
    });

    sortCardsList(rearrangePool).forEach((card) => {
      const cardEl = renderCardElement(card, 'sm', {
        selected: offeredId === card.id
      });
      cardEl.addEventListener('click', () => {
        const idx = rearrangePool.findIndex(c => c.id === card.id);
        if (idx >= 0) {
          rearrangePool.splice(idx, 1);
          if (!rearrangeGroups[rearrangeActiveGroupIdx]) {
            rearrangeGroups.push([]);
            rearrangeActiveGroupIdx = rearrangeGroups.length - 1;
          }
          rearrangeGroups[rearrangeActiveGroupIdx].push(card);
          renderRearrangeModal();
        }
      });
      poolEl.appendChild(cardEl);
    });

    // Validate full submission
    const nonEmptyGroups = rearrangeGroups.filter(g => g.length > 0);
    const assignedIds = new Set();
    nonEmptyGroups.forEach(g => g.forEach(c => assignedIds.add(c.id)));

    const missingExisting = Array.from(existingIds).some(id => !assignedIds.has(id));
    const missingOffered = offeredId !== null && !assignedIds.has(offeredId);
    const allGroupsValid = nonEmptyGroups.length > 0 && nonEmptyGroups.every(g => isValidMeld(g));

    let errorText = '';
    if (missingExisting) {
      errorText = lang === 'es' ? '⚠️ Todas las cartas ya bajadas en mesa deben permanecer en juegos válidos.' : '⚠️ All existing table meld cards must remain in valid melds.';
    } else if (missingOffered) {
      errorText = lang === 'es' ? '⚠️ Debes incluir la carta ofrecida en la mesa.' : '⚠️ You must include the offered card in a meld.';
    } else if (!allGroupsValid) {
      errorText = lang === 'es' ? '⚠️ Cada grupo debe ser una tercia/cuarteta o corrida válida de 3+.' : '⚠️ Every group must be a valid set or run of 3+ cards.';
    }

    if (errorText) {
      msgEl.style.color = '#fca5a5';
      msgEl.textContent = errorText;
      $('btn-confirm-rearrange').disabled = true;
    } else {
      msgEl.style.color = '#86efac';
      msgEl.textContent = lang === 'es' ? '✅ ¡Combinación válida!' : '✅ Valid meld arrangement!';
      $('btn-confirm-rearrange').disabled = false;
    }
  }

  function submitRearrangeModal() {
    const nonEmptyGroups = rearrangeGroups.filter(g => g.length > 0);
    const handIdsSet = new Set((gameState.myHand || []).map(c => c.id));
    const usedHandIds = [];
    nonEmptyGroups.forEach(g => g.forEach(c => {
      if (handIdsSet.has(c.id)) usedHandIds.push(c.id);
    }));
    const proposedMelds = nonEmptyGroups.map(g => g.map(c => c.id));

    if (gameState.phase === 'claiming_offer') {
      sendWS('meld_card', { roomCode, usedHandIds, proposedMelds });
    } else {
      sendWS('rearrange_melds', { roomCode, usedHandIds, proposedMelds });
    }
    $('rearrange-modal').classList.remove('open');
    selectedHandIds.clear();
  }

  // Winner / Tablas Modal
  function renderWinnerModal() {
    if (!gameState) return;
    const isHost = gameState.hostId === playerId;
    const meldsPreview = $('winner-melds-preview');
    meldsPreview.innerHTML = '';

    if (gameState.isTie) {
      $('winner-icons').textContent = '🤝🃏🔥';
      $('winner-title').textContent = t('tablasTitle');
      $('winner-message').textContent = t('tablasMsg', { pot: gameState.pot });
      $('winner-pot-banner').style.display = gameState.pot > 0 ? 'inline-block' : 'none';
      $('winner-pot-label').textContent = lang === 'es' ? 'Pozo Acumulado:' : 'Rollover Pot:';
      $('winner-pot-amount').textContent = gameState.pot;
    } else if (gameState.winner) {
      const w = gameState.winner;
      $('winner-icons').textContent = '🏆🎺🎊';
      $('winner-title').textContent = t('winnerTitle');
      $('winner-message').textContent = t('winnerMsg', { name: w.name, target: gameState.targetMeldCount });
      $('winner-pot-banner').style.display = w.potWon > 0 ? 'inline-block' : 'none';
      $('winner-pot-label').textContent = lang === 'es' ? 'Pozo Ganado:' : 'Pot Won:';
      $('winner-pot-amount').textContent = w.potWon;

      (w.melds || []).forEach((m) => {
        const grp = document.createElement('div');
        grp.className = 'table-meld-group';
        m.cards.forEach(c => grp.appendChild(renderCardElement(c, 'sm')));
        meldsPreview.appendChild(grp);
      });
    }

    const sb = $('winner-scoreboard');
    sb.innerHTML = '';
    const sorted = [...(gameState.players || [])].sort((a, b) => b.wins - a.wins || b.chips - a.chips);
    sorted.forEach((p, idx) => {
      const row = document.createElement('div');
      row.className = 'score-row';
      row.innerHTML = `
        <span>#${idx + 1} <strong>${escapeHtml(p.name)}</strong></span>
        <span>🏆 ${p.wins} • 💰 ${p.chips}</span>
      `;
      sb.appendChild(row);
    });

    $('winner-host-actions').style.display = isHost ? 'grid' : 'none';
    $('winner-guest-msg').style.display = isHost ? 'none' : 'block';
    $('winner-guest-actions').style.display = isHost ? 'none' : 'block';
    $('winner-modal').classList.add('open');
  }

  function renderRulesBody() {
    $('rules-body').innerHTML = lang === 'es'
      ? `<p><strong>1. Baraja de 40 o 80 Cartas:</strong> No existen el 8 ni el 9 (en poker equivalen a J=10, Q=11, K=12). Por eso el <strong>7 y el 10 (Sota)</strong> van seguidos en las corridas (ej. 6-7-10).</p>
         <p><strong>2. Escalado Multijugador (2–6 jugadores):</strong> En modo automático, con 2 jugadores se reparten 10 cartas (meta 11); con 3 jugadores en baraja simple se reparten 8 (meta 9); y de 4 a 6 jugadores se usan 2 barajas (80 cartas) para jugar con 10 cartas cada quien (meta 11).</p>
         <p><strong>3. Turno por Anillo Estricto:</strong> En tu turno volteas una carta de <em>El Mazo</em>. No entra a tu mano: debes bajarla en un juego (tercia/cuarteta o corrida de 3+), aplicarla con <strong>¡Obligado!</strong> o pasar. Si pasas, la prioridad gira alrededor de la mesa uno por uno.</p>
         <p><strong>4. ¡Obligado!:</strong> Si la carta en mesa cabe en un juego ya bajado de un rival (y le queda al menos 1 carta en mano), ¡puedes obligarlo a tomarla y forzarlo a descartar!</p>
         <p><strong>5. Tablas:</strong> Si se acaba El Mazo y nadie bajó todas sus cartas, hay empate y <strong>El Pozo se acumula</strong> para la siguiente mano.</p>`
      : `<p><strong>1. 40 or 80-Card Spanish Deck:</strong> Ranks run 1 (Ace), 2, 3, 4, 5, 6, 7, 10 (Sota/J), 11 (Caballo/Q), 12 (Rey/K). Notice <strong>7 and 10 are consecutive</strong> in runs (e.g. 6-7-10).</p>
         <p><strong>2. Multiplayer Scaling (2–6 Players):</strong> Auto mode deals 10 cards (meld 11 to win) for 2 players; 8 cards (meld 9) for 3 players; and automatically upgrades to a Double Deck (80 cards) for 4–6 players so everyone plays full 10-card hands.</p>
         <p><strong>3. Strict Turn-Order Claim Ring:</strong> On your turn, flip from <em>El Mazo</em>. Cards never go straight into your hand — you must meld them immediately, force them via <strong>¡Obligado!</strong>, or Pass. Priority moves one seat at a time clockwise.</p>
         <p><strong>4. ¡Obligado! (Forcing):</strong> If the offered card extends an opponent's table meld (and they have ≥1 hand card), you can force them to take it and immediately discard!</p>
         <p><strong>5. Tablas (Tie Rollover):</strong> If El Mazo runs out with no winner, the hand is a tie and <strong>El Pozo rolls over</strong> into the next round!</p>`;
  }

  function escapeHtml(str) {
    return String(str)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;');
  }

  // Ambient floating decorations
  function spawnAmbientParticles() {
    const layer = $('ambient-theme-layer');
    if (!layer) return;
    layer.innerHTML = '';
    const icons = themeIndex === 1
      ? ['💀', '🌼', '🕯️', '🃏']
      : themeIndex === 2
        ? ['🪙', '🎭', '🃏', '✨']
        : ['🪙', '🏆', '⚔️', '🌶️', '🃏'];
    for (let i = 0; i < 14; i++) {
      const span = document.createElement('span');
      span.className = 'ambient-particle';
      span.textContent = icons[i % icons.length];
      span.style.left = `${(i * 7.3) % 96}%`;
      span.style.fontSize = `${14 + (i % 4) * 5}px`;
      span.style.animationDuration = `${12 + (i % 5) * 4}s`;
      span.style.animationDelay = `${-(i * 1.7)}s`;
      span.style.setProperty('--drift-x', `${(i % 2 === 0 ? 1 : -1) * (15 + (i % 3) * 12)}px`);
      span.style.setProperty('--rot', `${(i % 2 === 0 ? 1 : -1) * 45}deg`);
      layer.appendChild(span);
    }
  }

  function applyTheme() {
    document.body.classList.remove(...THEMES);
    document.body.classList.add(THEMES[themeIndex % THEMES.length]);
    spawnAmbientParticles();
  }

  // Event Listeners
  function bindEvents() {
    $('tab-create').addEventListener('click', () => {
      $('tab-create').classList.add('active');
      $('tab-join').classList.remove('active');
      $('form-create').style.display = 'block';
      $('form-join').style.display = 'none';
    });

    $('tab-join').addEventListener('click', () => {
      $('tab-join').classList.add('active');
      $('tab-create').classList.remove('active');
      $('form-join').style.display = 'block';
      $('form-create').style.display = 'none';
    });

    $('form-create').addEventListener('submit', (e) => {
      e.preventDefault();
      playerName = $('create-name').value.trim() || 'Anfitrión';
      const chipsVal = Number($('create-chips').value);
      const settings = {
        deckMode: $('create-deck-mode').value,
        allowObligado: $('create-obligado').checked,
        enableChips: chipsVal > 0,
        buyIn: chipsVal,
        turnSpeedMs: Number($('create-speed').value)
      };
      sendWS('create_room', { playerName, settings });
    });

    $('form-join').addEventListener('submit', (e) => {
      e.preventDefault();
      playerName = $('join-name').value.trim() || 'Jugador';
      const code = $('join-code').value.trim().toUpperCase();
      if (code.length < 4) return;
      sendWS('join_room', { roomCode: code, playerName });
    });

    $('btn-copy-code').addEventListener('click', () => {
      if (!roomCode) return;
      const url = `${window.location.origin}/?room=${roomCode}`;
      navigator.clipboard?.writeText(url).then(() => {
        showToast(lang === 'es' ? `¡Enlace copiado! (${roomCode})` : `Invite link copied! (${roomCode})`, 'success');
      });
    });

    $('btn-toggle-qr').addEventListener('click', () => {
      const card = $('lobby-qr-card');
      card.style.display = card.style.display === 'none' ? 'block' : 'none';
      renderQrCode();
    });

    $('btn-add-bot').addEventListener('click', () => sendWS('add_bot', { roomCode }));
    $('btn-remove-bot').addEventListener('click', () => sendWS('remove_bot', { roomCode }));
    $('btn-start-game').addEventListener('click', () => sendWS('start_game', { roomCode }));

    const sendLobbySettings = () => {
      if (!gameState || gameState.hostId !== playerId) return;
      const chipsVal = Number($('lobby-chips').value);
      sendWS('update_settings', {
        roomCode,
        settings: {
          deckMode: $('lobby-deck-mode').value,
          allowObligado: $('lobby-obligado').checked,
          enableChips: chipsVal > 0,
          buyIn: chipsVal,
          turnSpeedMs: (gameState.settings && gameState.settings.turnSpeedMs) || 1200
        }
      });
    };
    $('lobby-deck-mode').addEventListener('change', sendLobbySettings);
    $('lobby-chips').addEventListener('change', sendLobbySettings);
    $('lobby-obligado').addEventListener('change', sendLobbySettings);

    $('stock-pile-card').addEventListener('click', () => {
      if (
        gameState &&
        gameState.state === 'playing' &&
        gameState.phase === 'awaiting_flip' &&
        gameState.activeActorId === playerId
      ) {
        sendWS('flip_stock', { roomCode });
      }
    });

    $('btn-sort-suit').addEventListener('click', () => {
      handSortMode = 'suit';
      renderGameTable();
    });
    $('btn-sort-rank').addEventListener('click', () => {
      handSortMode = 'rank';
      renderGameTable();
    });
    $('btn-clear-selection').addEventListener('click', () => {
      selectedHandIds.clear();
      renderGameTable();
    });

    $('btn-open-rearrange').addEventListener('click', openRearrangeModal);
    $('btn-add-group').addEventListener('click', () => {
      rearrangeGroups.push([]);
      rearrangeActiveGroupIdx = rearrangeGroups.length - 1;
      renderRearrangeModal();
    });
    $('btn-confirm-rearrange').addEventListener('click', submitRearrangeModal);
    $('btn-cancel-rearrange').addEventListener('click', () => {
      $('rearrange-modal').classList.remove('open');
    });

    $('btn-restart-round').addEventListener('click', () => sendWS('restart_round', { roomCode }));
    const exitRoom = () => sendWS('leave_room', { roomCode });
    $('btn-header-leave').addEventListener('click', exitRoom);
    $('btn-winner-exit-host').addEventListener('click', exitRoom);
    $('btn-winner-exit-guest').addEventListener('click', exitRoom);

    document.querySelectorAll('.reaction-btn').forEach((btn) => {
      btn.addEventListener('click', () => {
        sendWS('send_reaction', {
          roomCode,
          emoji: btn.dataset.emoji,
          playerName: playerName || 'Jugador'
        });
      });
    });

    document.querySelectorAll('.cotorreo-btn').forEach((btn) => {
      btn.addEventListener('click', () => {
        sendWS('send_reaction', {
          roomCode,
          emoji: btn.dataset.shout,
          playerName: playerName || 'Jugador'
        });
      });
    });

    $('btn-toggle-lang').addEventListener('click', () => {
      lang = lang === 'es' ? 'en' : 'es';
      localStorage.setItem('conquian_lang', lang);
      applyTranslations();
    });

    $('btn-toggle-deck-style').addEventListener('click', () => {
      deckStyle = deckStyle === 'baraja' ? 'poker' : 'baraja';
      localStorage.setItem('conquian_deck_style', deckStyle);
      applyTranslations();
    });

    $('btn-toggle-theme').addEventListener('click', () => {
      themeIndex = (themeIndex + 1) % THEMES.length;
      localStorage.setItem('conquian_theme_idx', String(themeIndex));
      applyTheme();
    });

    $('btn-toggle-sound').addEventListener('click', () => {
      window.conquianAudio.enabled = !window.conquianAudio.enabled;
      $('btn-toggle-sound').textContent = window.conquianAudio.enabled ? '🔊' : '🔇';
    });

    $('btn-toggle-speech').addEventListener('click', () => {
      window.conquianAudio.speechEnabled = !window.conquianAudio.speechEnabled;
      $('btn-toggle-speech').style.opacity = window.conquianAudio.speechEnabled ? '1' : '0.55';
    });

    $('btn-rules').addEventListener('click', () => {
      renderRulesBody();
      $('rules-modal').classList.add('open');
    });
    $('btn-close-rules').addEventListener('click', () => $('rules-modal').classList.remove('open'));

    $('btn-resume-game').addEventListener('click', () => {
      const raw = localStorage.getItem('conquian_session');
      if (!raw) return;
      try {
        const saved = JSON.parse(raw);
        playerName = saved.playerName || '';
        sendWS('reconnect_session', {
          roomCode: saved.roomCode,
          playerId: saved.playerId
        });
      } catch {
        clearSession();
      }
    });
    $('btn-dismiss-resume').addEventListener('click', clearSession);

    // PWA install prompt
    window.addEventListener('beforeinstallprompt', (e) => {
      e.preventDefault();
      deferredInstallPrompt = e;
      $('btn-pwa-install').style.display = 'inline-flex';
    });
    $('btn-pwa-install').addEventListener('click', async () => {
      if (!deferredInstallPrompt) return;
      deferredInstallPrompt.prompt();
      await deferredInstallPrompt.userChoice;
      deferredInstallPrompt = null;
      $('btn-pwa-install').style.display = 'none';
    });
  }

  // Initialization
  function init() {
    applyTheme();
    bindEvents();
    applyTranslations();

    // Check URL ?room=CODE invite parameter
    const params = new URLSearchParams(window.location.search);
    const inviteCode = params.get('room');
    if (inviteCode) {
      $('tab-join').click();
      $('join-code').value = inviteCode.toUpperCase();
    }

    // Check saved session banner
    const savedRaw = localStorage.getItem('conquian_session');
    if (savedRaw) {
      try {
        const saved = JSON.parse(savedRaw);
        if (saved && saved.roomCode && saved.playerId) {
          $('resume-room-code').textContent = saved.roomCode;
          $('resume-player-name').textContent = saved.playerName || 'Jugador';
          $('resume-banner').style.display = 'flex';
        }
      } catch {
        clearSession();
      }
    }

    if ('serviceWorker' in navigator) {
      navigator.serviceWorker.register('/sw.js').catch(() => {});
    }
  }

  document.addEventListener('DOMContentLoaded', init);
})();
