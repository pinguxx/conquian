use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::game::{generate_room_code, BotAction, GameSettings, Room, RoomState, TurnPhase};

pub type RoomSockets = HashMap<String, mpsc::UnboundedSender<Message>>;

#[derive(Clone)]
pub struct AppState {
    pub rooms: Arc<RwLock<HashMap<String, Room>>>,
    pub sockets: Arc<RwLock<HashMap<String, RoomSockets>>>,
    pub bot_generations: Arc<RwLock<HashMap<String, u64>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            sockets: Arc::new(RwLock::new(HashMap::new())),
            bot_generations: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "create_room")]
    CreateRoom {
        #[serde(rename = "playerName")]
        player_name: String,
        settings: Option<GameSettings>,
    },
    #[serde(rename = "join_room")]
    JoinRoom {
        #[serde(rename = "roomCode")]
        room_code: String,
        #[serde(rename = "playerName")]
        player_name: String,
    },
    #[serde(rename = "update_settings")]
    UpdateSettings {
        #[serde(rename = "roomCode")]
        room_code: String,
        settings: GameSettings,
    },
    #[serde(rename = "add_bot")]
    AddBot {
        #[serde(rename = "roomCode")]
        room_code: String,
    },
    #[serde(rename = "remove_bot")]
    RemoveBot {
        #[serde(rename = "roomCode")]
        room_code: String,
        #[serde(rename = "botId")]
        bot_id: Option<String>,
    },
    #[serde(rename = "start_game")]
    StartGame {
        #[serde(rename = "roomCode")]
        room_code: String,
    },
    #[serde(rename = "flip_stock")]
    FlipStock {
        #[serde(rename = "roomCode")]
        room_code: String,
    },
    #[serde(rename = "pass_card")]
    PassCard {
        #[serde(rename = "roomCode")]
        room_code: String,
    },
    #[serde(rename = "meld_card")]
    MeldCard {
        #[serde(rename = "roomCode")]
        room_code: String,
        #[serde(default, rename = "usedHandIds")]
        used_hand_ids: Vec<u8>,
        #[serde(default, rename = "proposedMelds")]
        proposed_melds: Vec<Vec<u8>>,
        #[serde(default, rename = "extendMeldId")]
        extend_meld_id: Option<String>,
    },
    #[serde(rename = "rearrange_melds")]
    RearrangeMelds {
        #[serde(rename = "roomCode")]
        room_code: String,
        #[serde(default, rename = "usedHandIds")]
        used_hand_ids: Vec<u8>,
        #[serde(default, rename = "proposedMelds")]
        proposed_melds: Vec<Vec<u8>>,
    },
    #[serde(rename = "force_obligado")]
    ForceObligado {
        #[serde(rename = "roomCode")]
        room_code: String,
        #[serde(rename = "targetPlayerId")]
        target_player_id: String,
        #[serde(rename = "targetMeldId")]
        target_meld_id: String,
    },
    #[serde(rename = "discard_card")]
    DiscardCard {
        #[serde(rename = "roomCode")]
        room_code: String,
        #[serde(rename = "cardId")]
        card_id: u8,
    },
    #[serde(rename = "restart_round")]
    RestartRound {
        #[serde(rename = "roomCode")]
        room_code: String,
    },
    #[serde(rename = "send_reaction")]
    SendReaction {
        #[serde(rename = "roomCode")]
        room_code: String,
        emoji: String,
        #[serde(rename = "playerName")]
        player_name: String,
    },
    #[serde(rename = "leave_room")]
    LeaveRoom {
        #[serde(rename = "roomCode")]
        room_code: String,
    },
    #[serde(rename = "reconnect_session")]
    ReconnectSession {
        #[serde(rename = "roomCode")]
        room_code: String,
        #[serde(rename = "playerId")]
        player_id: String,
    },
}

pub fn send_event(tx: &mpsc::UnboundedSender<Message>, event_name: &str, data: Value) {
    let msg = json!({
        "event": event_name,
        "data": data
    })
    .to_string();
    let _ = tx.send(Message::Text(msg));
}

pub async fn broadcast_event(state: &AppState, room_code: &str, event_name: &str, data: Value) {
    let msg = json!({
        "event": event_name,
        "data": data
    })
    .to_string();

    let sockets = state.sockets.read().await;
    if let Some(room_map) = sockets.get(room_code) {
        for tx in room_map.values() {
            let _ = tx.send(Message::Text(msg.clone()));
        }
    }
}

pub fn build_player_state_payload(room: &Room, viewer_id: &str) -> Value {
    let sanitized_players = room.sanitize_players();
    let my_hand = room
        .players
        .get(viewer_id)
        .map(|p| p.hand.clone())
        .unwrap_or_default();

    let current_turn_player_id = room.seat_order.get(room.current_turn_idx).cloned();
    let claim_turn_player_id = room.seat_order.get(room.claim_turn_idx).cloned();
    let offered_by_name = room
        .offered_by
        .as_ref()
        .and_then(|id| room.players.get(id))
        .map(|p| p.name.clone());

    json!({
        "roomCode": room.code,
        "hostId": room.host_id,
        "state": match room.state {
            RoomState::Lobby => "lobby",
            RoomState::Playing => "playing",
            RoomState::Ended => "ended",
        },
        "settings": room.settings,
        "pot": room.pot,
        "potRolledOver": room.pot_rolled_over,
        "roundNumber": room.round_number,
        "isDoubleDeck": room.is_double_deck,
        "handSize": room.hand_size,
        "targetMeldCount": room.target_meld_count,
        "players": sanitized_players,
        "myId": viewer_id,
        "myHand": my_hand,
        "stockCount": room.stock.len(),
        "wasteCount": room.waste.len(),
        "offeredCard": room.offered_card,
        "offeredFrom": room.offered_from,
        "offeredBy": room.offered_by,
        "offeredByName": offered_by_name,
        "currentTurnPlayerId": current_turn_player_id,
        "claimTurnPlayerId": claim_turn_player_id,
        "mustDiscardPlayerId": room.must_discard_player_id,
        "activeActorId": room.active_actor_id(),
        "passedPlayerIds": room.passed_player_ids,
        "phase": match room.phase {
            TurnPhase::AwaitingFlip => "awaiting_flip",
            TurnPhase::ClaimingOffer => "claiming_offer",
            TurnPhase::MustDiscard => "must_discard",
        },
        "obligadoTargets": room.obligado_targets_for(viewer_id),
        "canExtendMyMelds": room.extendable_own_melds_for(viewer_id),
        "winner": room.winner,
        "isTie": room.is_tie,
        "lastNotice": room.last_notice,
    })
}

pub async fn broadcast_room_state(state: &AppState, room_code: &str) {
    let outbound: Vec<(mpsc::UnboundedSender<Message>, String)> = {
        let rooms = state.rooms.read().await;
        let room = match rooms.get(room_code) {
            Some(r) => r,
            None => return,
        };
        let sockets = state.sockets.read().await;
        let room_map = match sockets.get(room_code) {
            Some(m) => m,
            None => return,
        };

        room_map
            .iter()
            .map(|(pid, tx)| {
                let payload = build_player_state_payload(room, pid);
                let msg = json!({
                    "event": "room_state",
                    "data": payload
                })
                .to_string();
                (tx.clone(), msg)
            })
            .collect()
    };

    for (tx, msg) in outbound {
        let _ = tx.send(Message::Text(msg));
    }
}

pub async fn maybe_drive_bots(state: AppState, room_code: String) {
    let (should_spawn, driver_seq, game_gen) = {
        let rooms = state.rooms.read().await;
        let room = match rooms.get(&room_code) {
            Some(r) => r,
            None => return,
        };
        if room.state != RoomState::Playing {
            return;
        }
        let is_bot_turn = room
            .active_actor_id()
            .and_then(|id| room.players.get(&id))
            .map(|p| p.is_bot)
            .unwrap_or(false);
        if !is_bot_turn {
            return;
        }
        let mut gens = state.bot_generations.write().await;
        let next_seq = gens.get(&room_code).copied().unwrap_or(0) + 1;
        gens.insert(room_code.clone(), next_seq);
        (true, next_seq, room.game_generation)
    };

    if !should_spawn {
        return;
    }

    tokio::spawn(async move {
        loop {
            let speed_ms = {
                let rooms = state.rooms.read().await;
                rooms
                    .get(&room_code)
                    .map(|r| r.settings.turn_speed_ms.clamp(600, 3000))
                    .unwrap_or(1100)
            };

            let jitter = rand::thread_rng().gen_range(0..350);
            tokio::time::sleep(tokio::time::Duration::from_millis(speed_ms + jitter)).await;

            // Verify this driver is still the latest active driver for this room & generation
            {
                let gens = state.bot_generations.read().await;
                if gens.get(&room_code).copied() != Some(driver_seq) {
                    break;
                }
            }

            let mut reaction_to_emit: Option<(String, String)> = None;
            let mut round_ended = false;
            let mut continue_loop = false;

            {
                let mut rooms = state.rooms.write().await;
                let room = match rooms.get_mut(&room_code) {
                    Some(r) => r,
                    None => break,
                };
                if room.state != RoomState::Playing || room.game_generation != game_gen {
                    break;
                }
                let actor_id = match room.active_actor_id() {
                    Some(id) => id,
                    None => break,
                };
                let actor_name = match room.players.get(&actor_id) {
                    Some(p) if p.is_bot => p.name.clone(),
                    _ => break,
                };
                let action = match room.compute_bot_action(&actor_id) {
                    Some(a) => a,
                    None => break,
                };

                match action {
                    BotAction::FlipStock => {
                        let _ = room.flip_stock(&actor_id);
                    }
                    BotAction::ExtendOwnMeld { meld_id } => {
                        if let Ok(won) = room.apply_meld(&actor_id, &[], &[], Some(&meld_id)) {
                            round_ended = won;
                            if won {
                                reaction_to_emit =
                                    Some((actor_name.clone(), "¡CONQUIÁN! 🏆🎉".to_string()));
                            } else {
                                reaction_to_emit =
                                    Some((actor_name.clone(), "¡Va por fuera! 🔥".to_string()));
                            }
                        }
                    }
                    BotAction::MeldSubmission {
                        used_hand_ids,
                        proposed_melds,
                    } => {
                        if let Ok(won) =
                            room.apply_meld(&actor_id, &used_hand_ids, &proposed_melds, None)
                        {
                            round_ended = won;
                            if won {
                                reaction_to_emit =
                                    Some((actor_name.clone(), "¡CONQUIÁN! 🏆🎉".to_string()));
                            }
                        }
                    }
                    BotAction::Obligado {
                        target_player_id,
                        target_meld_id,
                    } => {
                        if room
                            .apply_obligado(&actor_id, &target_player_id, &target_meld_id)
                            .is_ok()
                        {
                            reaction_to_emit =
                                Some((actor_name.clone(), "⚡ ¡OBLIGADO, compadre!".to_string()));
                        }
                    }
                    BotAction::Pass => {
                        let _ = room.pass_offer(&actor_id);
                        if room.state == RoomState::Ended {
                            round_ended = true;
                        }
                    }
                    BotAction::RearrangeWin {
                        used_hand_ids,
                        proposed_melds,
                    } => {
                        if let Ok(won) =
                            room.apply_rearrange_melds(&actor_id, &used_hand_ids, &proposed_melds)
                        {
                            round_ended = won;
                            if won {
                                reaction_to_emit =
                                    Some((actor_name.clone(), "¡CONQUIÁN! 🏆🎉".to_string()));
                            }
                        }
                    }
                    BotAction::Discard(card_id) => {
                        if let Ok(won) = room.discard_card(&actor_id, card_id) {
                            round_ended = won;
                        }
                    }
                }

                if !round_ended && room.state == RoomState::Playing {
                    continue_loop = room
                        .active_actor_id()
                        .and_then(|id| room.players.get(&id))
                        .map(|p| p.is_bot)
                        .unwrap_or(false);
                }
            }

            if let Some((sender, emoji)) = reaction_to_emit {
                broadcast_event(
                    &state,
                    &room_code,
                    "new_reaction",
                    json!({
                        "emoji": emoji,
                        "sender": sender,
                        "id": format!("{:x}", rand::random::<u64>())
                    }),
                )
                .await;
            }

            broadcast_room_state(&state, &room_code).await;

            if !continue_loop {
                break;
            }
        }
    });
}

pub async fn leave_room_helper(state: &AppState, pid: &str, room_code: &str) {
    let mut should_broadcast = false;
    {
        let mut rooms = state.rooms.write().await;
        if let Some(room) = rooms.get_mut(room_code) {
            if room.state == RoomState::Playing {
                // Keep card conservation intact by converting departed human into a bot during live play
                let other_humans = room
                    .players
                    .values()
                    .any(|p| !p.is_bot && p.id != pid && p.connected);
                if !other_humans {
                    rooms.remove(room_code);
                } else if let Some(p) = room.players.get_mut(pid) {
                    p.is_bot = true;
                    p.connected = true;
                    p.name = format!("🤖 {} (Auto)", p.name);
                    if room.host_id == pid {
                        if let Some(next_host) = room
                            .seat_order
                            .iter()
                            .find(|id| room.players.get(*id).map(|pl| !pl.is_bot).unwrap_or(false))
                        {
                            room.host_id = next_host.clone();
                        }
                    }
                    should_broadcast = true;
                }
            } else {
                room.remove_player(pid);
                let has_human = room.players.values().any(|p| !p.is_bot);
                if !has_human {
                    rooms.remove(room_code);
                } else {
                    should_broadcast = true;
                }
            }
        }
    }

    {
        let mut sockets = state.sockets.write().await;
        if let Some(room_sockets) = sockets.get_mut(room_code) {
            room_sockets.remove(pid);
            if room_sockets.is_empty() {
                sockets.remove(room_code);
            }
        }
    }

    if should_broadcast {
        broadcast_room_state(state, room_code).await;
        maybe_drive_bots(state.clone(), room_code.to_string()).await;
    }
}

pub async fn handle_ws_connection(socket: WebSocket, state: AppState) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let conn_id = format!("{:x}", rand::random::<u128>());
    let mut current_room: Option<String> = None;
    let mut current_player_id: Option<String> = None;

    let state_clone = state.clone();
    let conn_id_clone = conn_id.clone();
    let tx_clone = tx.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let Message::Text(text) = msg {
                let client_msg = match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(m) => m,
                    Err(e) => {
                        send_event(
                            &tx_clone,
                            "action_error",
                            json!({ "message": format!("Invalid message: {}", e) }),
                        );
                        continue;
                    }
                };

                match client_msg {
                    ClientMessage::CreateRoom {
                        player_name,
                        settings,
                    } => {
                        let mut code = generate_room_code();
                        {
                            let rooms = state_clone.rooms.read().await;
                            while rooms.contains_key(&code) {
                                code = generate_room_code();
                            }
                        }

                        let clean_name = if player_name.trim().is_empty() {
                            "Anfitrión".to_string()
                        } else {
                            player_name.trim().to_string()
                        };
                        let room_settings = settings.unwrap_or_default();
                        let room = Room::new(
                            code.clone(),
                            conn_id_clone.clone(),
                            clean_name,
                            room_settings,
                        );

                        {
                            let mut rooms = state_clone.rooms.write().await;
                            rooms.insert(code.clone(), room);
                        }
                        {
                            let mut sockets = state_clone.sockets.write().await;
                            sockets
                                .entry(code.clone())
                                .or_default()
                                .insert(conn_id_clone.clone(), tx_clone.clone());
                        }

                        current_room = Some(code.clone());
                        current_player_id = Some(conn_id_clone.clone());

                        send_event(
                            &tx_clone,
                            "room_created",
                            json!({
                                "roomCode": code,
                                "playerId": conn_id_clone,
                                "isHost": true
                            }),
                        );
                        broadcast_room_state(&state_clone, &code).await;
                    }

                    ClientMessage::JoinRoom {
                        room_code,
                        player_name,
                    } => {
                        let code = room_code.trim().to_uppercase();
                        let mut rooms = state_clone.rooms.write().await;
                        if let Some(room) = rooms.get_mut(&code) {
                            let clean_name = if player_name.trim().is_empty() {
                                format!("Jugador {}", room.players.len() + 1)
                            } else {
                                player_name.trim().to_string()
                            };
                            match room.add_human_player(
                                conn_id_clone.clone(),
                                clean_name,
                                conn_id_clone.clone(),
                            ) {
                                Ok(()) => {
                                    let is_host = room.host_id == conn_id_clone;
                                    drop(rooms);
                                    {
                                        let mut sockets = state_clone.sockets.write().await;
                                        sockets
                                            .entry(code.clone())
                                            .or_default()
                                            .insert(conn_id_clone.clone(), tx_clone.clone());
                                    }
                                    current_room = Some(code.clone());
                                    current_player_id = Some(conn_id_clone.clone());

                                    send_event(
                                        &tx_clone,
                                        "joined_room",
                                        json!({
                                            "roomCode": code,
                                            "playerId": conn_id_clone,
                                            "isHost": is_host
                                        }),
                                    );
                                    broadcast_room_state(&state_clone, &code).await;
                                }
                                Err(err) => {
                                    send_event(&tx_clone, "join_error", json!({ "message": err }));
                                }
                            }
                        } else {
                            send_event(
                                &tx_clone,
                                "join_error",
                                json!({ "message": "Room code not found / Sala no encontrada." }),
                            );
                        }
                    }

                    ClientMessage::UpdateSettings {
                        room_code,
                        settings,
                    } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let mut updated = false;
                        {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                if room.host_id == pid && room.state != RoomState::Playing {
                                    room.settings = settings;
                                    updated = true;
                                }
                            }
                        }
                        if updated {
                            broadcast_room_state(&state_clone, &room_code).await;
                        }
                    }

                    ClientMessage::AddBot { room_code } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let mut result = Ok(());
                        {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                if room.host_id == pid {
                                    result = room.add_bot().map(|_| ());
                                }
                            }
                        }
                        match result {
                            Ok(()) => broadcast_room_state(&state_clone, &room_code).await,
                            Err(e) => {
                                send_event(&tx_clone, "action_error", json!({ "message": e }))
                            }
                        }
                    }

                    ClientMessage::RemoveBot { room_code, bot_id } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let mut changed = false;
                        {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                if room.host_id == pid {
                                    changed = room.remove_bot(bot_id.as_deref()).is_ok();
                                }
                            }
                        }
                        if changed {
                            broadcast_room_state(&state_clone, &room_code).await;
                        }
                    }

                    ClientMessage::StartGame { room_code }
                    | ClientMessage::RestartRound { room_code } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let res = {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                if room.host_id != pid {
                                    Err("Only host can start the round".to_string())
                                } else {
                                    room.start_round()
                                }
                            } else {
                                Err("Room not found".to_string())
                            }
                        };
                        match res {
                            Ok(()) => {
                                broadcast_event(
                                    &state_clone,
                                    &room_code,
                                    "game_started",
                                    json!({ "roomCode": room_code }),
                                )
                                .await;
                                broadcast_room_state(&state_clone, &room_code).await;
                                maybe_drive_bots(state_clone.clone(), room_code).await;
                            }
                            Err(e) => {
                                send_event(&tx_clone, "action_error", json!({ "message": e }))
                            }
                        }
                    }

                    ClientMessage::FlipStock { room_code } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let res = {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                room.flip_stock(pid).map(|_| ())
                            } else {
                                Err("Room not found".to_string())
                            }
                        };
                        match res {
                            Ok(()) => {
                                broadcast_room_state(&state_clone, &room_code).await;
                                maybe_drive_bots(state_clone.clone(), room_code).await;
                            }
                            Err(e) => {
                                send_event(&tx_clone, "action_error", json!({ "message": e }))
                            }
                        }
                    }

                    ClientMessage::PassCard { room_code } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let res = {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                room.pass_offer(pid)
                            } else {
                                Err("Room not found".to_string())
                            }
                        };
                        match res {
                            Ok(()) => {
                                broadcast_room_state(&state_clone, &room_code).await;
                                maybe_drive_bots(state_clone.clone(), room_code).await;
                            }
                            Err(e) => {
                                send_event(&tx_clone, "action_error", json!({ "message": e }))
                            }
                        }
                    }

                    ClientMessage::MeldCard {
                        room_code,
                        used_hand_ids,
                        proposed_melds,
                        extend_meld_id,
                    } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let res = {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                room.apply_meld(
                                    pid,
                                    &used_hand_ids,
                                    &proposed_melds,
                                    extend_meld_id.as_deref(),
                                )
                            } else {
                                Err("Room not found".to_string())
                            }
                        };
                        match res {
                            Ok(_) => {
                                broadcast_room_state(&state_clone, &room_code).await;
                                maybe_drive_bots(state_clone.clone(), room_code).await;
                            }
                            Err(e) => {
                                send_event(&tx_clone, "action_error", json!({ "message": e }))
                            }
                        }
                    }

                    ClientMessage::RearrangeMelds {
                        room_code,
                        used_hand_ids,
                        proposed_melds,
                    } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let res = {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                room.apply_rearrange_melds(pid, &used_hand_ids, &proposed_melds)
                            } else {
                                Err("Room not found".to_string())
                            }
                        };
                        match res {
                            Ok(_) => {
                                broadcast_room_state(&state_clone, &room_code).await;
                                maybe_drive_bots(state_clone.clone(), room_code).await;
                            }
                            Err(e) => {
                                send_event(&tx_clone, "action_error", json!({ "message": e }))
                            }
                        }
                    }

                    ClientMessage::ForceObligado {
                        room_code,
                        target_player_id,
                        target_meld_id,
                    } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let res = {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                room.apply_obligado(pid, &target_player_id, &target_meld_id)
                            } else {
                                Err("Room not found".to_string())
                            }
                        };
                        match res {
                            Ok(()) => {
                                broadcast_room_state(&state_clone, &room_code).await;
                                maybe_drive_bots(state_clone.clone(), room_code).await;
                            }
                            Err(e) => {
                                send_event(&tx_clone, "action_error", json!({ "message": e }))
                            }
                        }
                    }

                    ClientMessage::DiscardCard { room_code, card_id } => {
                        let pid = current_player_id.as_deref().unwrap_or(&conn_id_clone);
                        let res = {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&room_code) {
                                room.discard_card(pid, card_id)
                            } else {
                                Err("Room not found".to_string())
                            }
                        };
                        match res {
                            Ok(_) => {
                                broadcast_room_state(&state_clone, &room_code).await;
                                maybe_drive_bots(state_clone.clone(), room_code).await;
                            }
                            Err(e) => {
                                send_event(&tx_clone, "action_error", json!({ "message": e }))
                            }
                        }
                    }

                    ClientMessage::SendReaction {
                        room_code,
                        emoji,
                        player_name,
                    } => {
                        broadcast_event(
                            &state_clone,
                            &room_code,
                            "new_reaction",
                            json!({
                                "emoji": emoji,
                                "sender": player_name,
                                "id": format!("{:x}", rand::random::<u64>())
                            }),
                        )
                        .await;
                    }

                    ClientMessage::LeaveRoom { room_code } => {
                        let pid = current_player_id
                            .clone()
                            .unwrap_or_else(|| conn_id_clone.clone());
                        leave_room_helper(&state_clone, &pid, &room_code).await;
                        current_room = None;
                        current_player_id = None;
                        send_event(&tx_clone, "left_room", json!({ "success": true }));
                    }

                    ClientMessage::ReconnectSession {
                        room_code,
                        player_id,
                    } => {
                        let code = room_code.trim().to_uppercase();
                        let mut restored_payload = None;
                        {
                            let mut rooms = state_clone.rooms.write().await;
                            if let Some(room) = rooms.get_mut(&code) {
                                if let Some(player) = room.players.get_mut(&player_id) {
                                    player.connected = true;
                                    player.conn_id = conn_id_clone.clone();
                                    restored_payload =
                                        Some(build_player_state_payload(room, &player_id));
                                }
                            }
                        }

                        if let Some(payload) = restored_payload {
                            {
                                let mut sockets = state_clone.sockets.write().await;
                                sockets
                                    .entry(code.clone())
                                    .or_default()
                                    .insert(player_id.clone(), tx_clone.clone());
                            }
                            current_room = Some(code.clone());
                            current_player_id = Some(player_id.clone());
                            send_event(&tx_clone, "session_restored", payload);
                            broadcast_room_state(&state_clone, &code).await;
                            maybe_drive_bots(state_clone.clone(), code).await;
                        } else {
                            send_event(
                                &tx_clone,
                                "reconnect_failed",
                                json!({ "message": "Room expired or no longer active." }),
                            );
                        }
                    }
                }
            }
        }

        // Disconnect grace period (60s for mobile reloads / screen locks)
        if let (Some(code), Some(pid)) = (current_room, current_player_id) {
            let mut should_wait = false;
            {
                let mut rooms = state_clone.rooms.write().await;
                if let Some(room) = rooms.get_mut(&code) {
                    if let Some(player) = room.players.get_mut(&pid) {
                        if player.conn_id == conn_id_clone {
                            player.connected = false;
                            should_wait = true;
                        }
                    }
                }
            }

            if should_wait {
                {
                    let mut sockets = state_clone.sockets.write().await;
                    if let Some(room_sockets) = sockets.get_mut(&code) {
                        room_sockets.remove(&pid);
                    }
                }
                broadcast_room_state(&state_clone, &code).await;

                let state_del = state_clone.clone();
                let code_del = code.clone();
                let pid_del = pid.clone();
                let conn_del = conn_id_clone.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    let still_disconnected = {
                        let rooms = state_del.rooms.read().await;
                        rooms
                            .get(&code_del)
                            .and_then(|r| r.players.get(&pid_del))
                            .map(|p| !p.connected && p.conn_id == conn_del)
                            .unwrap_or(false)
                    };
                    if still_disconnected {
                        leave_room_helper(&state_del, &pid_del, &code_del).await;
                    }
                });
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
