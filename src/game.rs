use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::cards::{
    can_extend_meld, card_name_en, card_name_es, generate_deck, rank_order,
    validate_hand_rearrange, validate_meld, validate_meld_submission, Card, Meld,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    #[serde(default = "default_deck_mode", rename = "deckMode")]
    pub deck_mode: String, // "auto", "single", "double"
    #[serde(default = "default_allow_obligado", rename = "allowObligado")]
    pub allow_obligado: bool,
    #[serde(default = "default_enable_chips", rename = "enableChips")]
    pub enable_chips: bool,
    #[serde(default = "default_buy_in", rename = "buyIn")]
    pub buy_in: u32,
    #[serde(default = "default_turn_speed_ms", rename = "turnSpeedMs")]
    pub turn_speed_ms: u64,
}

fn default_deck_mode() -> String {
    "auto".to_string()
}
fn default_allow_obligado() -> bool {
    true
}
fn default_enable_chips() -> bool {
    true
}
fn default_buy_in() -> u32 {
    5
}
fn default_turn_speed_ms() -> u64 {
    1200
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            deck_mode: default_deck_mode(),
            allow_obligado: default_allow_obligado(),
            enable_chips: default_enable_chips(),
            buy_in: default_buy_in(),
            turn_speed_ms: default_turn_speed_ms(),
        }
    }
}

/// Resolves `(is_double_deck, hand_size, target_meld_count)` based on player count and deck mode.
pub fn resolve_scaling(player_count: usize, deck_mode: &str) -> (bool, usize, usize) {
    match deck_mode {
        "ten" => (player_count >= 4, 10, 11),
        "double" => (true, 8, 9),
        "single" => match player_count {
            0..=3 => (false, 8, 9),
            4 => (false, 7, 8),
            _ => (true, 8, 9),
        },
        _ => {
            // "auto": Classic 9-Card Conquián (deal 8, win with 9 — 3 tercias = 9!)
            if player_count <= 3 {
                (false, 8, 9)
            } else {
                (true, 8, 9)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoomState {
    Lobby,
    Playing,
    Ended,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TurnPhase {
    AwaitingFlip,
    ClaimingOffer,
    MustDiscard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub hand: Vec<Card>,
    pub melds: Vec<Meld>,
    pub wins: u32,
    pub ready: bool,
    pub connected: bool,
    pub conn_id: String,
    pub chips: u32,
    pub is_bot: bool,
}

impl Player {
    pub fn melded_count(&self) -> usize {
        self.melds.iter().map(|m| m.cards.len()).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizedPlayer {
    pub id: String,
    pub name: String,
    #[serde(rename = "seatIndex")]
    pub seat_index: usize,
    #[serde(rename = "isHost")]
    pub is_host: bool,
    #[serde(rename = "handCount")]
    pub hand_count: usize,
    pub melds: Vec<Meld>,
    #[serde(rename = "meldedCount")]
    pub melded_count: usize,
    pub wins: u32,
    pub connected: bool,
    pub chips: u32,
    #[serde(rename = "isBot")]
    pub is_bot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObligadoTarget {
    #[serde(rename = "targetPlayerId")]
    pub target_player_id: String,
    #[serde(rename = "targetPlayerName")]
    pub target_player_name: String,
    #[serde(rename = "meldId")]
    pub meld_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinnerInfo {
    pub id: String,
    pub name: String,
    pub melds: Vec<Meld>,
    #[serde(rename = "potWon")]
    pub pot_won: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionNotice {
    pub kind: String, // "flip", "meld", "obligado", "discard", "pass", "burn", "win", "tie"
    #[serde(rename = "messageEs")]
    pub message_es: String,
    #[serde(rename = "messageEn")]
    pub message_en: String,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub code: String,
    pub host_id: String,
    pub state: RoomState,
    pub settings: GameSettings,
    pub seat_order: Vec<String>,
    pub players: HashMap<String, Player>,
    pub pot: u32,
    pub pot_rolled_over: bool,
    pub round_number: u32,
    pub is_double_deck: bool,
    pub hand_size: usize,
    pub target_meld_count: usize,
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub offered_card: Option<Card>,
    pub offered_from: String, // "stock" | "discard"
    pub offered_by: Option<String>,
    pub current_turn_idx: usize,
    pub claim_turn_idx: usize,
    pub passed_player_ids: Vec<String>,
    pub must_discard_player_id: Option<String>,
    pub phase: TurnPhase,
    pub winner: Option<WinnerInfo>,
    pub is_tie: bool,
    pub last_notice: Option<ActionNotice>,
    pub game_generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BotAction {
    FlipStock,
    ExtendOwnMeld {
        meld_id: String,
    },
    MeldSubmission {
        used_hand_ids: Vec<u8>,
        proposed_melds: Vec<Vec<u8>>,
    },
    Obligado {
        target_player_id: String,
        target_meld_id: String,
    },
    Pass,
    RearrangeWin {
        used_hand_ids: Vec<u8>,
        proposed_melds: Vec<Vec<u8>>,
    },
    Discard(u8),
}

impl Room {
    pub fn new(code: String, host_id: String, host_name: String, settings: GameSettings) -> Self {
        let mut players = HashMap::new();
        let host_player = Player {
            id: host_id.clone(),
            name: host_name,
            hand: Vec::new(),
            melds: Vec::new(),
            wins: 0,
            ready: true,
            connected: true,
            conn_id: host_id.clone(),
            chips: 50,
            is_bot: false,
        };
        players.insert(host_id.clone(), host_player);

        Self {
            code,
            host_id: host_id.clone(),
            state: RoomState::Lobby,
            settings,
            seat_order: vec![host_id],
            players,
            pot: 0,
            pot_rolled_over: false,
            round_number: 0,
            is_double_deck: false,
            hand_size: 8,
            target_meld_count: 9,
            stock: Vec::new(),
            waste: Vec::new(),
            offered_card: None,
            offered_from: "stock".to_string(),
            offered_by: None,
            current_turn_idx: 0,
            claim_turn_idx: 0,
            passed_player_ids: Vec::new(),
            must_discard_player_id: None,
            phase: TurnPhase::AwaitingFlip,
            winner: None,
            is_tie: false,
            last_notice: None,
            game_generation: 0,
        }
    }

    pub fn add_human_player(
        &mut self,
        player_id: String,
        name: String,
        conn_id: String,
    ) -> Result<(), String> {
        if self.players.len() >= 6 {
            return Err("Room is full (max 6 players)".to_string());
        }
        if self.state != RoomState::Lobby {
            return Err("Game has already started".to_string());
        }
        let player = Player {
            id: player_id.clone(),
            name,
            hand: Vec::new(),
            melds: Vec::new(),
            wins: 0,
            ready: true,
            connected: true,
            conn_id,
            chips: 50,
            is_bot: false,
        };
        self.players.insert(player_id.clone(), player);
        if !self.seat_order.contains(&player_id) {
            self.seat_order.push(player_id);
        }
        Ok(())
    }

    pub fn add_bot(&mut self) -> Result<Player, String> {
        if self.players.len() >= 6 {
            return Err("Room is full (max 6 players)".to_string());
        }
        if self.state != RoomState::Lobby {
            return Err("Cannot add bots after game starts".to_string());
        }

        const BOT_PROFILES: [&str; 6] = [
            "🤖 Don Ramón Bot",
            "🤖 La Catrina Bot",
            "🤖 Cantinflas Bot",
            "🤖 El Chavo Bot",
            "🤖 Frida Bot",
            "🤖 El Santo Bot",
        ];

        let existing_names: HashSet<&str> =
            self.players.values().map(|p| p.name.as_str()).collect();
        let bot_name = BOT_PROFILES
            .iter()
            .copied()
            .find(|n| !existing_names.contains(n))
            .unwrap_or("🤖 Pancho Bot");

        let bot_id = format!("bot_{:x}", rand::random::<u32>());
        let bot_player = Player {
            id: bot_id.clone(),
            name: bot_name.to_string(),
            hand: Vec::new(),
            melds: Vec::new(),
            wins: 0,
            ready: true,
            connected: true,
            conn_id: format!("conn_{}", bot_id),
            chips: 50,
            is_bot: true,
        };
        self.players.insert(bot_id.clone(), bot_player.clone());
        self.seat_order.push(bot_id);
        Ok(bot_player)
    }

    pub fn remove_bot(&mut self, bot_id: Option<&str>) -> Result<String, String> {
        if self.state != RoomState::Lobby {
            return Err("Cannot remove bots while game is running".to_string());
        }
        let target_id = if let Some(id) = bot_id {
            if self.players.get(id).map(|p| p.is_bot).unwrap_or(false) {
                Some(id.to_string())
            } else {
                None
            }
        } else {
            self.seat_order
                .iter()
                .rev()
                .find(|id| self.players.get(*id).map(|p| p.is_bot).unwrap_or(false))
                .cloned()
        };

        if let Some(id) = target_id {
            self.players.remove(&id);
            self.seat_order.retain(|s| s != &id);
            Ok(id)
        } else {
            Err("No bots in this room".to_string())
        }
    }

    pub fn remove_player(&mut self, player_id: &str) {
        self.players.remove(player_id);
        self.seat_order.retain(|s| s != player_id);
        if self.host_id == player_id {
            if let Some(next_human) = self
                .seat_order
                .iter()
                .find(|id| self.players.get(*id).map(|p| !p.is_bot).unwrap_or(false))
            {
                self.host_id = next_human.clone();
            } else if let Some(next_any) = self.seat_order.first() {
                self.host_id = next_any.clone();
            }
        }
    }

    pub fn start_round(&mut self) -> Result<(), String> {
        let count = self.seat_order.len();
        if count < 2 {
            return Err("Need at least 2 players to start Conquián".to_string());
        }
        if count > 6 {
            return Err("Maximum 6 players supported".to_string());
        }

        let (is_double, hand_size, target_meld) = resolve_scaling(count, &self.settings.deck_mode);

        self.is_double_deck = is_double;
        self.hand_size = hand_size;
        self.target_meld_count = target_meld;
        self.round_number += 1;
        self.state = RoomState::Playing;
        self.winner = None;
        self.is_tie = false;
        self.offered_card = None;
        self.offered_from = "stock".to_string();
        self.offered_by = None;
        self.passed_player_ids.clear();
        self.must_discard_player_id = None;
        self.waste.clear();
        self.phase = TurnPhase::AwaitingFlip;
        self.game_generation += 1;

        // Handle chips buy-in (preserving rolled-over pot if previous hand ended in Tablas)
        let prev_pot = self.pot;
        self.pot_rolled_over = prev_pot > 0;
        if self.settings.enable_chips && self.settings.buy_in > 0 {
            let buy_in = self.settings.buy_in;
            let mut round_pot = prev_pot;
            for pid in &self.seat_order {
                if let Some(p) = self.players.get_mut(pid) {
                    let ante = p.chips.min(buy_in);
                    p.chips -= ante;
                    round_pot += ante;
                }
            }
            self.pot = round_pot;
        }

        let mut deck = generate_deck(is_double);
        for pid in &self.seat_order {
            if let Some(p) = self.players.get_mut(pid) {
                p.hand.clear();
                p.melds.clear();
                for _ in 0..hand_size {
                    if let Some(card) = deck.pop() {
                        p.hand.push(card);
                    }
                }
                sort_hand_default(&mut p.hand);
            }
        }
        self.stock = deck;

        // Rotate starting player each round
        self.current_turn_idx = ((self.round_number as usize).saturating_sub(1)) % count;
        self.claim_turn_idx = self.current_turn_idx;

        let starter_name = self
            .seat_order
            .get(self.current_turn_idx)
            .and_then(|id| self.players.get(id))
            .map(|p| p.name.clone())
            .unwrap_or_default();

        self.last_notice = Some(ActionNotice {
            kind: "start".to_string(),
            message_es: format!(
                "¡Ronda #{}! Repartiendo {} cartas (Meta: {} cartas). Empieza {}.",
                self.round_number, hand_size, target_meld, starter_name
            ),
            message_en: format!(
                "Round #{}! Dealt {} cards each (Target: {} melded). {} flips first.",
                self.round_number, hand_size, target_meld, starter_name
            ),
        });

        Ok(())
    }

    pub fn active_actor_id(&self) -> Option<String> {
        if self.state != RoomState::Playing {
            return None;
        }
        match self.phase {
            TurnPhase::AwaitingFlip => self.seat_order.get(self.current_turn_idx).cloned(),
            TurnPhase::ClaimingOffer => self.seat_order.get(self.claim_turn_idx).cloned(),
            TurnPhase::MustDiscard => self.must_discard_player_id.clone(),
        }
    }

    pub fn flip_stock(&mut self, player_id: &str) -> Result<Card, String> {
        if self.state != RoomState::Playing || self.phase != TurnPhase::AwaitingFlip {
            return Err("Not awaiting a stock flip".to_string());
        }
        let expected_id = self
            .seat_order
            .get(self.current_turn_idx)
            .ok_or_else(|| "Invalid seat index".to_string())?;
        if expected_id != player_id {
            return Err("It is not your turn to flip from the stock".to_string());
        }

        let card = match self.stock.pop() {
            Some(c) => c,
            None => {
                self.finish_as_tie();
                return Err("Stock is empty".to_string());
            }
        };

        let actor_name = self
            .players
            .get(player_id)
            .map(|p| p.name.clone())
            .unwrap_or_default();

        self.offered_card = Some(card.clone());
        self.offered_from = "stock".to_string();
        self.offered_by = Some(player_id.to_string());
        self.claim_turn_idx = self.current_turn_idx;
        self.passed_player_ids.clear();
        self.phase = TurnPhase::ClaimingOffer;
        self.last_notice = Some(ActionNotice {
            kind: "flip".to_string(),
            message_es: format!(
                "{} volteó {}",
                actor_name,
                card_name_es(card.rank, card.suit)
            ),
            message_en: format!(
                "{} flipped {}",
                actor_name,
                card_name_en(card.rank, card.suit)
            ),
        });

        Ok(card)
    }

    pub fn apply_meld(
        &mut self,
        player_id: &str,
        used_hand_ids: &[u8],
        proposed_melds: &[Vec<u8>],
        extend_meld_id: Option<&str>,
    ) -> Result<bool, String> {
        if self.state != RoomState::Playing || self.phase != TurnPhase::ClaimingOffer {
            return Err("No offered card is currently claimable".to_string());
        }
        let priority_id = self
            .seat_order
            .get(self.claim_turn_idx)
            .ok_or_else(|| "Invalid claim seat".to_string())?;
        if priority_id != player_id {
            return Err("It is not your priority turn to claim this card".to_string());
        }

        let offered = self
            .offered_card
            .clone()
            .ok_or_else(|| "No card is currently offered".to_string())?;

        let player = self
            .players
            .get(player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        let mut derived_proposed: Vec<Vec<u8>> = proposed_melds.to_vec();
        let mut derived_used_hand: Vec<u8> = used_hand_ids.to_vec();

        if let Some(target_mid) = extend_meld_id {
            if derived_proposed.is_empty() {
                let mut found = false;
                for m in &player.melds {
                    let mut ids: Vec<u8> = m.cards.iter().map(|c| c.id).collect();
                    if m.id == target_mid {
                        ids.push(offered.id);
                        found = true;
                    }
                    derived_proposed.push(ids);
                }
                if !found {
                    return Err("Target meld not found".to_string());
                }
                derived_used_hand.clear();
            }
        }

        let (updated_melds, remaining_hand) = validate_meld_submission(
            &offered,
            &player.hand,
            &player.melds,
            &derived_used_hand,
            &derived_proposed,
        )?;

        let actor_name = player.name.clone();
        let melded_total: usize = updated_melds.iter().map(|m| m.cards.len()).sum();

        if let Some(p_mut) = self.players.get_mut(player_id) {
            p_mut.melds = updated_melds;
            p_mut.hand = remaining_hand;
        }

        self.offered_card = None;
        self.passed_player_ids.clear();

        if melded_total >= self.target_meld_count {
            self.finish_with_winner(player_id);
            return Ok(true);
        }

        self.phase = TurnPhase::MustDiscard;
        self.must_discard_player_id = Some(player_id.to_string());
        self.last_notice = Some(ActionNotice {
            kind: "meld".to_string(),
            message_es: format!(
                "{} bajó {} — debe descartar 1 carta",
                actor_name,
                card_name_es(offered.rank, offered.suit)
            ),
            message_en: format!(
                "{} melded {} — must discard 1 card",
                actor_name,
                card_name_en(offered.rank, offered.suit)
            ),
        });

        Ok(false)
    }

    pub fn apply_rearrange_melds(
        &mut self,
        player_id: &str,
        used_hand_ids: &[u8],
        proposed_melds: &[Vec<u8>],
    ) -> Result<bool, String> {
        if self.state != RoomState::Playing {
            return Err("Game is not currently active".to_string());
        }
        if self.active_actor_id().as_deref() != Some(player_id) {
            return Err(
                "You can only lay hand melds or mix sets during your active turn".to_string(),
            );
        }

        let player = self
            .players
            .get(player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        let (updated_melds, remaining_hand) =
            validate_hand_rearrange(&player.hand, &player.melds, used_hand_ids, proposed_melds)?;

        let melded_total: usize = updated_melds.iter().map(|m| m.cards.len()).sum();
        if remaining_hand.is_empty() && melded_total < self.target_meld_count {
            return Err(
                "Must keep at least 1 card in hand until reaching target meld count".to_string(),
            );
        }

        let actor_name = player.name.clone();

        if let Some(p_mut) = self.players.get_mut(player_id) {
            p_mut.melds = updated_melds;
            p_mut.hand = remaining_hand;
        }

        if melded_total >= self.target_meld_count {
            self.finish_with_winner(player_id);
            return Ok(true);
        }

        self.last_notice = Some(ActionNotice {
            kind: "meld".to_string(),
            message_es: format!("{} acomodó sus juegos en la mesa", actor_name),
            message_en: format!("{} arranged their table melds", actor_name),
        });

        Ok(false)
    }

    pub fn apply_obligado(
        &mut self,
        caller_id: &str,
        target_player_id: &str,
        target_meld_id: &str,
    ) -> Result<(), String> {
        if !self.settings.allow_obligado {
            return Err("¡Obligado! rule is disabled in this room".to_string());
        }
        if self.state != RoomState::Playing || self.phase != TurnPhase::ClaimingOffer {
            return Err("No offered card to force".to_string());
        }
        let priority_id = self
            .seat_order
            .get(self.claim_turn_idx)
            .ok_or_else(|| "Invalid priority seat".to_string())?;
        if priority_id != caller_id {
            return Err("Only the player with priority can call ¡Obligado!".to_string());
        }
        if caller_id == target_player_id {
            return Err("Use normal meld to add to your own melds".to_string());
        }

        let offered = self
            .offered_card
            .clone()
            .ok_or_else(|| "No card is currently offered".to_string())?;

        let caller_name = self
            .players
            .get(caller_id)
            .map(|p| p.name.clone())
            .ok_or_else(|| "Caller not found".to_string())?;

        let target = self
            .players
            .get(target_player_id)
            .ok_or_else(|| "Target player not found".to_string())?;

        if target.hand.is_empty() {
            return Err(
                "Cannot force a player who has no cards left in hand to discard".to_string(),
            );
        }

        let meld_idx = target
            .melds
            .iter()
            .position(|m| m.id == target_meld_id)
            .ok_or_else(|| "Target meld not found".to_string())?;

        let extended = can_extend_meld(&target.melds[meld_idx], &offered)
            .ok_or_else(|| "Offered card does not fit that opponent's meld".to_string())?;

        let target_name = target.name.clone();

        if let Some(target_mut) = self.players.get_mut(target_player_id) {
            target_mut.melds[meld_idx] = extended;
        }

        self.offered_card = None;
        self.passed_player_ids.clear();
        self.phase = TurnPhase::MustDiscard;
        self.must_discard_player_id = Some(target_player_id.to_string());
        self.last_notice = Some(ActionNotice {
            kind: "obligado".to_string(),
            message_es: format!(
                "⚡ ¡OBLIGADO! {} obligó a {} a bajar {}",
                caller_name,
                target_name,
                card_name_es(offered.rank, offered.suit)
            ),
            message_en: format!(
                "⚡ ¡OBLIGADO! {} forced {} to take {}",
                caller_name,
                target_name,
                card_name_en(offered.rank, offered.suit)
            ),
        });

        Ok(())
    }

    pub fn pass_offer(&mut self, player_id: &str) -> Result<(), String> {
        if self.state != RoomState::Playing || self.phase != TurnPhase::ClaimingOffer {
            return Err("No active offer to pass".to_string());
        }
        let priority_id = self
            .seat_order
            .get(self.claim_turn_idx)
            .ok_or_else(|| "Invalid priority seat".to_string())?;
        if priority_id != player_id {
            return Err("It is not your turn to decide on this card".to_string());
        }

        if !self.passed_player_ids.iter().any(|id| id == player_id) {
            self.passed_player_ids.push(player_id.to_string());
        }

        let seat_count = self.seat_order.len();
        let required_passes = if self.offered_from == "discard" {
            seat_count.saturating_sub(1)
        } else {
            seat_count
        };

        if self.passed_player_ids.len() >= required_passes {
            if let Some(burned) = self.offered_card.take() {
                let card_es = card_name_es(burned.rank, burned.suit);
                let card_en = card_name_en(burned.rank, burned.suit);
                self.waste.push(burned);

                if self.offered_from == "stock" {
                    self.current_turn_idx = (self.current_turn_idx + 1) % seat_count;
                }
                self.passed_player_ids.clear();

                if self.stock.is_empty() {
                    self.finish_as_tie();
                    return Ok(());
                }

                self.phase = TurnPhase::AwaitingFlip;
                let next_name = self
                    .seat_order
                    .get(self.current_turn_idx)
                    .and_then(|id| self.players.get(id))
                    .map(|p| p.name.clone())
                    .unwrap_or_default();

                self.last_notice = Some(ActionNotice {
                    kind: "burn".to_string(),
                    message_es: format!(
                        "Todos pasaron — {} fue a la basura. Turno de {}.",
                        card_es, next_name
                    ),
                    message_en: format!(
                        "Everyone passed — {} burned. {} flips next.",
                        card_en, next_name
                    ),
                });
            }
            return Ok(());
        }

        // Advance claim priority to the next eligible seat (skipping discarder when offered_from == "discard")
        let mut next_idx = (self.claim_turn_idx + 1) % seat_count;
        if self.offered_from == "discard" {
            for _ in 0..seat_count {
                let candidate = &self.seat_order[next_idx];
                if self.offered_by.as_deref() != Some(candidate.as_str())
                    && !self.passed_player_ids.contains(candidate)
                {
                    break;
                }
                next_idx = (next_idx + 1) % seat_count;
            }
        }
        self.claim_turn_idx = next_idx;

        let passer_name = self
            .players
            .get(player_id)
            .map(|p| p.name.clone())
            .unwrap_or_default();
        let next_name = self
            .seat_order
            .get(self.claim_turn_idx)
            .and_then(|id| self.players.get(id))
            .map(|p| p.name.clone())
            .unwrap_or_default();

        self.last_notice = Some(ActionNotice {
            kind: "pass".to_string(),
            message_es: format!("{} pasó — decide {}", passer_name, next_name),
            message_en: format!("{} passed — {}'s decision", passer_name, next_name),
        });

        Ok(())
    }

    pub fn discard_card(&mut self, player_id: &str, card_id: u8) -> Result<bool, String> {
        if self.state != RoomState::Playing || self.phase != TurnPhase::MustDiscard {
            return Err("Not currently waiting for a discard".to_string());
        }
        if self.must_discard_player_id.as_deref() != Some(player_id) {
            return Err("It is not your turn to discard".to_string());
        }

        let discarded_card;
        let actor_name;
        let melded_total;
        {
            let player = self
                .players
                .get_mut(player_id)
                .ok_or_else(|| "Player not found".to_string())?;
            let pos = player
                .hand
                .iter()
                .position(|c| c.id == card_id)
                .ok_or_else(|| "Card is not in your hand".to_string())?;
            discarded_card = player.hand.remove(pos);
            actor_name = player.name.clone();
            melded_total = player.melded_count();
        }

        if melded_total >= self.target_meld_count {
            self.finish_with_winner(player_id);
            return Ok(true);
        }

        let discarder_idx = self
            .seat_order
            .iter()
            .position(|id| id == player_id)
            .ok_or_else(|| "Player seat not found".to_string())?;
        let next_idx = (discarder_idx + 1) % self.seat_order.len();

        self.must_discard_player_id = None;
        self.offered_card = Some(discarded_card.clone());
        self.offered_from = "discard".to_string();
        self.offered_by = Some(player_id.to_string());
        self.current_turn_idx = next_idx;
        self.claim_turn_idx = next_idx;
        self.passed_player_ids.clear();
        self.phase = TurnPhase::ClaimingOffer;

        self.last_notice = Some(ActionNotice {
            kind: "discard".to_string(),
            message_es: format!(
                "{} descartó {}",
                actor_name,
                card_name_es(discarded_card.rank, discarded_card.suit)
            ),
            message_en: format!(
                "{} discarded {}",
                actor_name,
                card_name_en(discarded_card.rank, discarded_card.suit)
            ),
        });

        Ok(false)
    }

    fn finish_with_winner(&mut self, winner_id: &str) {
        self.state = RoomState::Ended;
        self.game_generation += 1;
        self.is_tie = false;
        self.must_discard_player_id = None;
        self.offered_card = None;

        let pot_won = self.pot;
        self.pot = 0;

        if let Some(p) = self.players.get_mut(winner_id) {
            p.wins += 1;
            p.chips += pot_won;
            self.winner = Some(WinnerInfo {
                id: p.id.clone(),
                name: p.name.clone(),
                melds: p.melds.clone(),
                pot_won,
            });
            self.last_notice = Some(ActionNotice {
                kind: "win".to_string(),
                message_es: format!("🏆 ¡CONQUIÁN! {} ganó la partida!", p.name),
                message_en: format!("🏆 ¡CONQUIÁN! {} won the hand!", p.name),
            });
        }
    }

    fn finish_as_tie(&mut self) {
        self.state = RoomState::Ended;
        self.game_generation += 1;
        self.winner = None;
        self.is_tie = true;
        self.must_discard_player_id = None;
        self.offered_card = None;
        // Note: self.pot intentionally remains intact to roll over into the next deal!
        self.last_notice = Some(ActionNotice {
            kind: "tie".to_string(),
            message_es: format!(
                "🤝 ¡TABLAS! Se acabó el mazo. El Pozo ({} fichas) se acumula para la siguiente mano.",
                self.pot
            ),
            message_en: format!(
                "🤝 TIE GAME (TABLAS)! Stock exhausted. The Pot ({} chips) rolls over!",
                self.pot
            ),
        });
    }

    pub fn obligado_targets_for(&self, viewer_id: &str) -> Vec<ObligadoTarget> {
        if !self.settings.allow_obligado
            || self.state != RoomState::Playing
            || self.phase != TurnPhase::ClaimingOffer
        {
            return Vec::new();
        }
        let offered = match &self.offered_card {
            Some(c) => c,
            None => return Vec::new(),
        };
        let mut out = Vec::new();
        for pid in &self.seat_order {
            if pid == viewer_id {
                continue;
            }
            if let Some(p) = self.players.get(pid) {
                if p.hand.is_empty() {
                    continue;
                }
                for m in &p.melds {
                    if can_extend_meld(m, offered).is_some() {
                        out.push(ObligadoTarget {
                            target_player_id: p.id.clone(),
                            target_player_name: p.name.clone(),
                            meld_id: m.id.clone(),
                        });
                    }
                }
            }
        }
        out
    }

    pub fn extendable_own_melds_for(&self, viewer_id: &str) -> Vec<String> {
        if self.state != RoomState::Playing || self.phase != TurnPhase::ClaimingOffer {
            return Vec::new();
        }
        let offered = match &self.offered_card {
            Some(c) => c,
            None => return Vec::new(),
        };
        let player = match self.players.get(viewer_id) {
            Some(p) => p,
            None => return Vec::new(),
        };
        player
            .melds
            .iter()
            .filter(|m| can_extend_meld(m, offered).is_some())
            .map(|m| m.id.clone())
            .collect()
    }

    pub fn sanitize_players(&self) -> Vec<SanitizedPlayer> {
        self.seat_order
            .iter()
            .enumerate()
            .filter_map(|(idx, pid)| {
                self.players.get(pid).map(|p| SanitizedPlayer {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    seat_index: idx,
                    is_host: p.id == self.host_id,
                    hand_count: p.hand.len(),
                    melds: p.melds.clone(),
                    melded_count: p.melded_count(),
                    wins: p.wins,
                    connected: p.connected,
                    chips: p.chips,
                    is_bot: p.is_bot,
                })
            })
            .collect()
    }

    pub fn compute_bot_action(&self, bot_id: &str) -> Option<BotAction> {
        if self.state != RoomState::Playing {
            return None;
        }
        let bot = self.players.get(bot_id)?;
        if !bot.is_bot {
            return None;
        }

        match self.phase {
            TurnPhase::AwaitingFlip => {
                if self
                    .seat_order
                    .get(self.current_turn_idx)
                    .map(|s| s.as_str())
                    == Some(bot_id)
                {
                    Some(BotAction::FlipStock)
                } else {
                    None
                }
            }
            TurnPhase::ClaimingOffer => {
                if self.seat_order.get(self.claim_turn_idx).map(|s| s.as_str()) != Some(bot_id) {
                    return None;
                }
                let offered = self.offered_card.as_ref()?;

                // 1. Extend own meld (especially if it wins, or generally whenever valid)
                for m in &bot.melds {
                    if can_extend_meld(m, offered).is_some() {
                        return Some(BotAction::ExtendOwnMeld {
                            meld_id: m.id.clone(),
                        });
                    }
                }

                // 2. Meld offered card + 2 cards from hand
                let n = bot.hand.len();
                for i in 0..n {
                    for j in (i + 1)..n {
                        let candidate =
                            vec![offered.clone(), bot.hand[i].clone(), bot.hand[j].clone()];
                        if validate_meld(&candidate).is_some() {
                            let mut proposed: Vec<Vec<u8>> = bot
                                .melds
                                .iter()
                                .map(|m| m.cards.iter().map(|c| c.id).collect())
                                .collect();
                            proposed.push(vec![offered.id, bot.hand[i].id, bot.hand[j].id]);
                            return Some(BotAction::MeldSubmission {
                                used_hand_ids: vec![bot.hand[i].id, bot.hand[j].id],
                                proposed_melds: proposed,
                            });
                        }
                    }
                }

                // 3. Force an opponent via ¡Obligado! if enabled
                if self.settings.allow_obligado {
                    let targets = self.obligado_targets_for(bot_id);
                    if let Some(first) = targets.first() {
                        return Some(BotAction::Obligado {
                            target_player_id: first.target_player_id.clone(),
                            target_meld_id: first.meld_id.clone(),
                        });
                    }
                }

                // 4. Otherwise pass
                Some(BotAction::Pass)
            }
            TurnPhase::MustDiscard => {
                if self.must_discard_player_id.as_deref() != Some(bot_id) {
                    return None;
                }
                // Check if bot can lay its final card(s) onto existing melds to win immediately
                if bot.hand.len() == 1 && bot.melded_count() + 1 >= self.target_meld_count {
                    let last_card = &bot.hand[0];
                    for (idx, m) in bot.melds.iter().enumerate() {
                        if can_extend_meld(m, last_card).is_some() {
                            let mut proposed: Vec<Vec<u8>> = bot
                                .melds
                                .iter()
                                .map(|existing| existing.cards.iter().map(|c| c.id).collect())
                                .collect();
                            proposed[idx].push(last_card.id);
                            return Some(BotAction::RearrangeWin {
                                used_hand_ids: vec![last_card.id],
                                proposed_melds: proposed,
                            });
                        }
                    }
                }

                // Pick the least synergistic card in hand to discard
                let card_to_discard = pick_least_useful_card(&bot.hand)?;
                Some(BotAction::Discard(card_to_discard))
            }
        }
    }
}

pub fn sort_hand_default(hand: &mut [Card]) {
    hand.sort_by_key(|c| (c.suit, rank_order(c.rank).unwrap_or(255), c.id));
}

fn pick_least_useful_card(hand: &[Card]) -> Option<u8> {
    if hand.is_empty() {
        return None;
    }
    let mut best_id = hand[0].id;
    let mut lowest_score = i32::MAX;

    for (idx, card) in hand.iter().enumerate() {
        let mut score = 0i32;
        let card_ord = rank_order(card.rank).unwrap_or(0) as i32;
        for (j, other) in hand.iter().enumerate() {
            if idx == j {
                continue;
            }
            if other.rank == card.rank && other.suit != card.suit {
                score += 6;
            }
            if other.suit == card.suit {
                let other_ord = rank_order(other.rank).unwrap_or(0) as i32;
                let diff = (card_ord - other_ord).abs();
                if diff == 1 {
                    score += 5;
                } else if diff == 2 {
                    score += 2;
                }
            }
        }
        if score < lowest_score {
            lowest_score = score;
            best_id = card.id;
        }
    }

    Some(best_id)
}

pub fn generate_room_code() -> String {
    const CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rng = rand::rng();
    (0..5)
        .map(|_| {
            let idx = rng.random_range(0..CHARS.len());
            CHARS[idx] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Suit;

    fn card(id: u8, suit: Suit, rank: u8) -> Card {
        Card {
            id,
            suit,
            rank,
            deck_index: if id > 40 { 1 } else { 0 },
        }
    }

    #[test]
    fn test_scaling_rules() {
        assert_eq!(resolve_scaling(2, "auto"), (false, 8, 9));
        assert_eq!(resolve_scaling(3, "auto"), (false, 8, 9));
        assert_eq!(resolve_scaling(4, "auto"), (true, 8, 9));
        assert_eq!(resolve_scaling(5, "auto"), (true, 8, 9));
        assert_eq!(resolve_scaling(6, "auto"), (true, 8, 9));

        assert_eq!(resolve_scaling(2, "single"), (false, 8, 9));
        assert_eq!(resolve_scaling(3, "single"), (false, 8, 9));
        assert_eq!(resolve_scaling(4, "single"), (false, 7, 8));
        assert_eq!(resolve_scaling(5, "single"), (true, 8, 9));

        assert_eq!(resolve_scaling(2, "double"), (true, 8, 9));
        assert_eq!(resolve_scaling(3, "double"), (true, 8, 9));

        assert_eq!(resolve_scaling(2, "ten"), (false, 10, 11));
        assert_eq!(resolve_scaling(4, "ten"), (true, 10, 11));
    }

    #[test]
    fn test_turn_flip_pass_ring_and_burn() {
        let mut room = Room::new(
            "TEST1".to_string(),
            "p1".to_string(),
            "Ana".to_string(),
            GameSettings::default(),
        );
        room.add_human_player("p2".to_string(), "Beto".to_string(), "c2".to_string())
            .unwrap();
        room.add_human_player("p3".to_string(), "Carlos".to_string(), "c3".to_string())
            .unwrap();
        room.start_round().unwrap();

        assert_eq!(room.hand_size, 8);
        assert_eq!(room.target_meld_count, 9);
        assert_eq!(room.stock.len(), 40 - 24);
        assert_eq!(room.phase, TurnPhase::AwaitingFlip);
        assert_eq!(room.active_actor_id().as_deref(), Some("p1"));

        let flipped = room.flip_stock("p1").unwrap();
        assert_eq!(room.phase, TurnPhase::ClaimingOffer);
        assert_eq!(room.active_actor_id().as_deref(), Some("p1"));

        room.pass_offer("p1").unwrap();
        assert_eq!(room.active_actor_id().as_deref(), Some("p2"));

        room.pass_offer("p2").unwrap();
        assert_eq!(room.active_actor_id().as_deref(), Some("p3"));

        room.pass_offer("p3").unwrap();
        assert_eq!(room.waste.len(), 1);
        assert_eq!(room.waste[0].id, flipped.id);
        assert_eq!(room.phase, TurnPhase::AwaitingFlip);
        assert_eq!(room.active_actor_id().as_deref(), Some("p2"));
    }

    #[test]
    fn test_meld_discard_obligado_and_win() {
        let mut room = Room::new(
            "TEST2".to_string(),
            "p1".to_string(),
            "Ana".to_string(),
            GameSettings {
                deck_mode: "single".to_string(),
                allow_obligado: true,
                enable_chips: true,
                buy_in: 10,
                turn_speed_ms: 1000,
            },
        );
        room.add_human_player("p2".to_string(), "Beto".to_string(), "c2".to_string())
            .unwrap();
        room.start_round().unwrap();
        assert_eq!(room.pot, 20);

        // Rig p1 & p2 hands cleanly for deterministic test
        room.players.get_mut("p1").unwrap().hand = vec![
            card(1, Suit::Oros, 1),
            card(2, Suit::Oros, 2),
            card(15, Suit::Copas, 5),
        ];
        room.players.get_mut("p2").unwrap().melds = vec![Meld {
            id: "m_p2".to_string(),
            kind: crate::cards::MeldKind::Run,
            cards: vec![
                card(21, Suit::Espadas, 1),
                card(22, Suit::Espadas, 2),
                card(23, Suit::Espadas, 3),
            ],
        }];
        room.players.get_mut("p2").unwrap().hand = vec![card(35, Suit::Bastos, 5)];

        // Put 3 of Oros on top of stock, then 4 of Espadas beneath it
        room.stock = vec![card(24, Suit::Espadas, 4), card(3, Suit::Oros, 3)];
        room.current_turn_idx = 0;
        room.phase = TurnPhase::AwaitingFlip;

        // P1 flips 3 of Oros and melds 1-2-3 of Oros
        room.flip_stock("p1").unwrap();
        let won = room
            .apply_meld("p1", &[1, 2], &[vec![1, 2, 3]], None)
            .unwrap();
        assert!(!won);
        assert_eq!(room.phase, TurnPhase::MustDiscard);
        assert_eq!(room.must_discard_player_id.as_deref(), Some("p1"));

        // P1 discards 5 of Copas -> offered to P2
        room.discard_card("p1", 15).unwrap();
        assert_eq!(room.phase, TurnPhase::ClaimingOffer);
        assert_eq!(room.active_actor_id().as_deref(), Some("p2"));

        // P2 passes -> burned, now P2 flips 4 of Espadas
        room.pass_offer("p2").unwrap();
        assert_eq!(room.phase, TurnPhase::AwaitingFlip);
        assert_eq!(room.active_actor_id().as_deref(), Some("p2"));

        room.flip_stock("p2").unwrap();
        // P2 passes on 4 of Espadas -> P1 gets priority and forces ¡Obligado! onto P2's Espadas run!
        room.pass_offer("p2").unwrap();
        assert_eq!(room.active_actor_id().as_deref(), Some("p1"));
        room.apply_obligado("p1", "p2", "m_p2").unwrap();
        assert_eq!(room.phase, TurnPhase::MustDiscard);
        assert_eq!(room.must_discard_player_id.as_deref(), Some("p2"));
        assert_eq!(room.players.get("p2").unwrap().melds[0].cards.len(), 4);
    }

    #[test]
    fn test_tablas_tie_rolls_over_pot() {
        let mut room = Room::new(
            "TIE01".to_string(),
            "p1".to_string(),
            "Ana".to_string(),
            GameSettings {
                enable_chips: true,
                buy_in: 10,
                ..GameSettings::default()
            },
        );
        room.add_human_player("p2".to_string(), "Beto".to_string(), "c2".to_string())
            .unwrap();
        room.start_round().unwrap();
        assert_eq!(room.pot, 20);

        // Leave only 1 card in stock and pass around
        room.stock.truncate(1);
        room.flip_stock("p1").unwrap();
        room.pass_offer("p1").unwrap();
        room.pass_offer("p2").unwrap();

        assert_eq!(room.state, RoomState::Ended);
        assert!(room.is_tie);
        assert!(room.winner.is_none());
        assert_eq!(room.pot, 20);

        // Next round rolls over the 20 chips + new 20 buy-in = 40 chips!
        room.start_round().unwrap();
        assert_eq!(room.pot, 40);
        assert!(room.pot_rolled_over);
    }

    #[test]
    fn test_win_at_target_meld_count() {
        let mut room = Room::new(
            "WIN01".to_string(),
            "p1".to_string(),
            "Ana".to_string(),
            GameSettings {
                enable_chips: true,
                buy_in: 5,
                ..GameSettings::default()
            },
        );
        room.add_human_player("p2".to_string(), "Beto".to_string(), "c2".to_string())
            .unwrap();
        room.start_round().unwrap();
        assert_eq!(room.hand_size, 8);
        assert_eq!(room.target_meld_count, 9);

        // Give P1 2 tercias (6 melded cards) + 2 hand cards (total 8), target = 9 (3 tercias!)
        room.players.get_mut("p1").unwrap().melds = vec![
            Meld {
                id: "m1".to_string(),
                kind: crate::cards::MeldKind::Set,
                cards: vec![
                    card(1, Suit::Oros, 1),
                    card(11, Suit::Copas, 1),
                    card(21, Suit::Espadas, 1),
                ],
            },
            Meld {
                id: "m2".to_string(),
                kind: crate::cards::MeldKind::Set,
                cards: vec![
                    card(2, Suit::Oros, 2),
                    card(12, Suit::Copas, 2),
                    card(22, Suit::Espadas, 2),
                ],
            },
        ];
        room.players.get_mut("p1").unwrap().hand =
            vec![card(5, Suit::Oros, 5), card(15, Suit::Copas, 5)];
        room.stock = vec![card(25, Suit::Espadas, 5)];
        room.current_turn_idx = 0;
        room.phase = TurnPhase::AwaitingFlip;

        room.flip_stock("p1").unwrap();
        let won = room
            .apply_meld(
                "p1",
                &[5, 15],
                &[vec![1, 11, 21], vec![2, 12, 22], vec![5, 15, 25]],
                None,
            )
            .unwrap();
        assert!(won);
        assert_eq!(room.state, RoomState::Ended);
        assert_eq!(room.winner.as_ref().unwrap().id, "p1");
        assert_eq!(room.winner.as_ref().unwrap().pot_won, 10);
        assert_eq!(room.players.get("p1").unwrap().chips, 55);
    }

    #[test]
    fn test_hand_meld_and_mix_without_offered_card() {
        let mut room = Room::new(
            "MIX01".to_string(),
            "p1".to_string(),
            "Ana".to_string(),
            GameSettings::default(),
        );
        room.add_human_player("p2".to_string(), "Beto".to_string(), "c2".to_string())
            .unwrap();
        room.start_round().unwrap();

        // 1) During AwaitingFlip, P1 lays a 3-card set straight from hand without an offered card
        room.players.get_mut("p1").unwrap().melds.clear();
        room.players.get_mut("p1").unwrap().hand = vec![
            card(6, Suit::Oros, 6),
            card(16, Suit::Copas, 6),
            card(26, Suit::Espadas, 6),
            card(36, Suit::Bastos, 6),
            card(4, Suit::Oros, 4),
            card(5, Suit::Oros, 5),
            card(31, Suit::Bastos, 1),
            card(32, Suit::Bastos, 2),
        ];
        room.stock = vec![card(29, Suit::Espadas, 12)];
        room.current_turn_idx = 0;
        room.phase = TurnPhase::AwaitingFlip;

        let won1 = room
            .apply_rearrange_melds("p1", &[6, 16, 26, 36], &[vec![6, 16, 26, 36]])
            .unwrap();
        assert!(!won1);
        assert_eq!(room.phase, TurnPhase::AwaitingFlip);
        assert_eq!(room.players.get("p1").unwrap().melds.len(), 1);
        assert_eq!(room.players.get("p1").unwrap().hand.len(), 4);

        // 2) Flip an unrelated stock card -> ClaimingOffer -> P1 mixes table 4-card set + 2 hand cards WITHOUT using the offered card
        room.flip_stock("p1").unwrap();
        assert_eq!(room.phase, TurnPhase::ClaimingOffer);
        assert!(room.offered_card.is_some());

        let won2 = room
            .apply_rearrange_melds("p1", &[4, 5], &[vec![16, 26, 36], vec![4, 5, 6]])
            .unwrap();
        assert!(!won2);
        assert_eq!(room.phase, TurnPhase::ClaimingOffer);
        assert!(room.offered_card.is_some());
        assert_eq!(room.players.get("p1").unwrap().melds.len(), 2);
        assert_eq!(room.players.get("p1").unwrap().melded_count(), 6);
        assert_eq!(room.players.get("p1").unwrap().hand.len(), 2);
    }
}
