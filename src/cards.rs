use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Suit {
    Oros,
    Copas,
    Espadas,
    Bastos,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Oros, Suit::Copas, Suit::Espadas, Suit::Bastos];

    pub fn name_es(self) -> &'static str {
        match self {
            Suit::Oros => "Oros",
            Suit::Copas => "Copas",
            Suit::Espadas => "Espadas",
            Suit::Bastos => "Bastos",
        }
    }

    pub fn name_en(self) -> &'static str {
        match self {
            Suit::Oros => "Coins",
            Suit::Copas => "Cups",
            Suit::Espadas => "Swords",
            Suit::Bastos => "Clubs",
        }
    }
}

pub const VALID_RANKS: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 10, 11, 12];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub id: u8,
    pub suit: Suit,
    pub rank: u8,
    #[serde(rename = "deckIndex")]
    pub deck_index: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeldKind {
    Set,
    Run,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meld {
    pub id: String,
    pub kind: MeldKind,
    pub cards: Vec<Card>,
}

/// Maps Baraja Española ranks [1,2,3,4,5,6,7,10,11,12] to consecutive indices 0..=9.
/// Notice 7 -> 6 and 10 -> 7 so 7 and 10 (Sota) are consecutive in Conquián runs.
pub fn rank_order(rank: u8) -> Option<u8> {
    match rank {
        1 => Some(0),
        2 => Some(1),
        3 => Some(2),
        4 => Some(3),
        5 => Some(4),
        6 => Some(5),
        7 => Some(6),
        10 => Some(7),
        11 => Some(8),
        12 => Some(9),
        _ => None,
    }
}

pub fn rank_name_es(rank: u8) -> &'static str {
    match rank {
        1 => "As",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        10 => "Sota",
        11 => "Caballo",
        12 => "Rey",
        _ => "?",
    }
}

pub fn rank_name_en(rank: u8) -> &'static str {
    match rank {
        1 => "Ace",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        10 => "Jack",
        11 => "Queen",
        12 => "King",
        _ => "?",
    }
}

pub fn card_name_es(rank: u8, suit: Suit) -> String {
    format!("{} de {}", rank_name_es(rank), suit.name_es())
}

pub fn card_name_en(rank: u8, suit: Suit) -> String {
    format!("{} of {}", rank_name_en(rank), suit.name_en())
}

pub fn generate_deck(double_deck: bool) -> Vec<Card> {
    let mut cards = Vec::with_capacity(if double_deck { 80 } else { 40 });
    let deck_count = if double_deck { 2 } else { 1 };
    let mut next_id: u8 = 1;

    for deck_index in 0..deck_count {
        for &suit in &Suit::ALL {
            for &rank in &VALID_RANKS {
                cards.push(Card {
                    id: next_id,
                    suit,
                    rank,
                    deck_index,
                });
                next_id += 1;
            }
        }
    }

    let mut rng = rand::rng();
    cards.shuffle(&mut rng);
    cards
}

/// Validates whether a group of cards forms a legal Conquián Set (Tercia/Cuarteta)
/// or Run (Corrida), returning the normalized sorted cards on success.
pub fn validate_meld(cards: &[Card]) -> Option<(MeldKind, Vec<Card>)> {
    if cards.len() < 3 {
        return None;
    }

    // Ensure unique physical card IDs
    let mut seen_ids = HashSet::with_capacity(cards.len());
    for c in cards {
        if !seen_ids.insert(c.id) || rank_order(c.rank).is_none() {
            return None;
        }
    }

    // Check Set (3 or 4 cards of the same rank, all distinct suits)
    if cards.len() <= 4 {
        let first_rank = cards[0].rank;
        if cards.iter().all(|c| c.rank == first_rank) {
            let distinct_suits: HashSet<Suit> = cards.iter().map(|c| c.suit).collect();
            if distinct_suits.len() == cards.len() {
                let mut sorted = cards.to_vec();
                sorted.sort_by_key(|c| (c.suit, c.id));
                return Some((MeldKind::Set, sorted));
            }
        }
    }

    // Check Run (3..=10 cards of the same suit in consecutive rank_order)
    if cards.len() <= VALID_RANKS.len() {
        let first_suit = cards[0].suit;
        if cards.iter().all(|c| c.suit == first_suit) {
            let mut sorted = cards.to_vec();
            sorted.sort_by_key(|c| rank_order(c.rank).unwrap_or(255));
            let is_consecutive =
                sorted
                    .windows(2)
                    .all(|w| match (rank_order(w[0].rank), rank_order(w[1].rank)) {
                        (Some(a), Some(b)) => b == a + 1,
                        _ => false,
                    });
            if is_consecutive {
                return Some((MeldKind::Run, sorted));
            }
        }
    }

    None
}

pub fn make_meld_id(cards: &[Card]) -> String {
    let mut ids: Vec<u8> = cards.iter().map(|c| c.id).collect();
    ids.sort_unstable();
    let joined: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
    format!("m_{}", joined.join("-"))
}

pub fn can_extend_meld(meld: &Meld, card: &Card) -> Option<Meld> {
    let mut combined = meld.cards.clone();
    combined.push(card.clone());
    let (kind, sorted) = validate_meld(&combined)?;
    Some(Meld {
        id: meld.id.clone(),
        kind,
        cards: sorted,
    })
}

fn validate_meld_partition(
    card_pool: &HashMap<u8, Card>,
    expected_ids: &HashSet<u8>,
    proposed_melds: &[Vec<u8>],
) -> Result<Vec<Meld>, String> {
    let mut seen_proposed: HashSet<u8> = HashSet::new();
    let mut result_melds = Vec::with_capacity(proposed_melds.len());

    for group_ids in proposed_melds {
        if group_ids.len() < 3 {
            return Err("Each meld must contain at least 3 cards".to_string());
        }
        let mut group_cards = Vec::with_capacity(group_ids.len());
        for &cid in group_ids {
            if !seen_proposed.insert(cid) {
                return Err(format!("Card {} appears in multiple melds", cid));
            }
            let card = card_pool
                .get(&cid)
                .ok_or_else(|| format!("Card {} is not available for melding", cid))?;
            group_cards.push(card.clone());
        }

        let (kind, sorted) = validate_meld(&group_cards).ok_or_else(|| {
            "Invalid meld: must be a valid set (tercia/cuarteta) or same-suit run (corrida)"
                .to_string()
        })?;
        result_melds.push(Meld {
            id: make_meld_id(&sorted),
            kind,
            cards: sorted,
        });
    }

    if &seen_proposed != expected_ids {
        return Err(
            "Proposed melds do not conserve all existing meld cards and selected cards".to_string(),
        );
    }

    Ok(result_melds)
}

/// Validates a player's meld action using `offered_card`, selected cards from `hand`,
/// and any existing table melds (supporting both direct additions and rearranging 4+ card melds).
pub fn validate_meld_submission(
    offered_card: &Card,
    hand: &[Card],
    existing_melds: &[Meld],
    used_hand_ids: &[u8],
    proposed_melds: &[Vec<u8>],
) -> Result<(Vec<Meld>, Vec<Card>), String> {
    let mut used_set: HashSet<u8> = HashSet::new();
    for &hid in used_hand_ids {
        if !used_set.insert(hid) {
            return Err("Duplicate hand card selected".to_string());
        }
    }

    let hand_map: HashMap<u8, Card> = hand.iter().cloned().map(|c| (c.id, c)).collect();
    for &hid in &used_set {
        if !hand_map.contains_key(&hid) {
            return Err(format!("Selected card {} is not in your hand", hid));
        }
    }

    let mut card_pool: HashMap<u8, Card> = HashMap::new();
    let mut expected_ids: HashSet<u8> = HashSet::new();

    for meld in existing_melds {
        for c in &meld.cards {
            card_pool.insert(c.id, c.clone());
            expected_ids.insert(c.id);
        }
    }
    for &hid in &used_set {
        if let Some(c) = hand_map.get(&hid) {
            card_pool.insert(c.id, c.clone());
            expected_ids.insert(c.id);
        }
    }

    card_pool.insert(offered_card.id, offered_card.clone());
    expected_ids.insert(offered_card.id);

    let updated_melds = validate_meld_partition(&card_pool, &expected_ids, proposed_melds)?;
    let remaining_hand: Vec<Card> = hand
        .iter()
        .filter(|c| !used_set.contains(&c.id))
        .cloned()
        .collect();

    Ok((updated_melds, remaining_hand))
}

/// Validates laying down or rearranging melds purely from a player's own hand + existing table melds
/// (used during MustDiscard when the player wants to lay additional melds before discarding).
pub fn validate_hand_rearrange(
    hand: &[Card],
    existing_melds: &[Meld],
    used_hand_ids: &[u8],
    proposed_melds: &[Vec<u8>],
) -> Result<(Vec<Meld>, Vec<Card>), String> {
    if used_hand_ids.is_empty() && proposed_melds.len() == existing_melds.len() {
        return Err("No new cards selected to meld".to_string());
    }

    let mut used_set: HashSet<u8> = HashSet::new();
    for &hid in used_hand_ids {
        if !used_set.insert(hid) {
            return Err("Duplicate hand card selected".to_string());
        }
    }

    let hand_map: HashMap<u8, Card> = hand.iter().cloned().map(|c| (c.id, c)).collect();
    for &hid in &used_set {
        if !hand_map.contains_key(&hid) {
            return Err(format!("Selected card {} is not in your hand", hid));
        }
    }

    let mut card_pool: HashMap<u8, Card> = HashMap::new();
    let mut expected_ids: HashSet<u8> = HashSet::new();

    for meld in existing_melds {
        for c in &meld.cards {
            card_pool.insert(c.id, c.clone());
            expected_ids.insert(c.id);
        }
    }
    for &hid in &used_set {
        if let Some(c) = hand_map.get(&hid) {
            card_pool.insert(c.id, c.clone());
            expected_ids.insert(c.id);
        }
    }

    let updated_melds = validate_meld_partition(&card_pool, &expected_ids, proposed_melds)?;
    let remaining_hand: Vec<Card> = hand
        .iter()
        .filter(|c| !used_set.contains(&c.id))
        .cloned()
        .collect();

    Ok((updated_melds, remaining_hand))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(id: u8, suit: Suit, rank: u8) -> Card {
        Card {
            id,
            suit,
            rank,
            deck_index: if id > 40 { 1 } else { 0 },
        }
    }

    #[test]
    fn test_deck_sizes() {
        let single = generate_deck(false);
        assert_eq!(single.len(), 40);
        let double = generate_deck(true);
        assert_eq!(double.len(), 80);
        let ids: HashSet<u8> = double.iter().map(|x| x.id).collect();
        assert_eq!(ids.len(), 80);
    }

    #[test]
    fn test_run_with_7_10_11_and_1_2_3() {
        let run_7_10_11 = vec![
            c(7, Suit::Oros, 7),
            c(8, Suit::Oros, 10),
            c(9, Suit::Oros, 11),
        ];
        let res = validate_meld(&run_7_10_11);
        assert!(res.is_some());
        let (kind, sorted) = res.unwrap();
        assert_eq!(kind, MeldKind::Run);
        assert_eq!(
            sorted.iter().map(|x| x.rank).collect::<Vec<_>>(),
            vec![7, 10, 11]
        );

        let run_ace = vec![
            c(13, Suit::Copas, 3),
            c(11, Suit::Copas, 1),
            c(12, Suit::Copas, 2),
        ];
        let res2 = validate_meld(&run_ace).unwrap();
        assert_eq!(res2.0, MeldKind::Run);
        assert_eq!(
            res2.1.iter().map(|x| x.rank).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );

        // Gap 7-11-12 without 10 is invalid
        let broken = vec![
            c(1, Suit::Oros, 7),
            c(2, Suit::Oros, 11),
            c(3, Suit::Oros, 12),
        ];
        assert!(validate_meld(&broken).is_none());
    }

    #[test]
    fn test_sets_and_duplicate_suit_rejection() {
        let set3 = vec![
            c(1, Suit::Oros, 5),
            c(11, Suit::Copas, 5),
            c(21, Suit::Espadas, 5),
        ];
        assert_eq!(validate_meld(&set3).unwrap().0, MeldKind::Set);

        let set4 = vec![
            c(1, Suit::Oros, 5),
            c(11, Suit::Copas, 5),
            c(21, Suit::Espadas, 5),
            c(31, Suit::Bastos, 5),
        ];
        assert_eq!(validate_meld(&set4).unwrap().0, MeldKind::Set);

        // Duplicate suit in double deck is invalid for a set
        let dup_suit = vec![
            c(1, Suit::Oros, 5),
            c(41, Suit::Oros, 5),
            c(11, Suit::Copas, 5),
        ];
        assert!(validate_meld(&dup_suit).is_none());
    }

    #[test]
    fn test_can_extend_meld_for_obligado() {
        let meld_run = Meld {
            id: "m1".to_string(),
            kind: MeldKind::Run,
            cards: vec![
                c(1, Suit::Espadas, 4),
                c(2, Suit::Espadas, 5),
                c(3, Suit::Espadas, 6),
            ],
        };
        assert!(can_extend_meld(&meld_run, &c(4, Suit::Espadas, 7)).is_some());
        assert!(can_extend_meld(&meld_run, &c(5, Suit::Espadas, 3)).is_some());
        assert!(can_extend_meld(&meld_run, &c(6, Suit::Oros, 7)).is_none());

        let meld_set = Meld {
            id: "m2".to_string(),
            kind: MeldKind::Set,
            cards: vec![
                c(10, Suit::Oros, 12),
                c(20, Suit::Copas, 12),
                c(30, Suit::Espadas, 12),
            ],
        };
        assert!(can_extend_meld(&meld_set, &c(40, Suit::Bastos, 12)).is_some());
        assert!(can_extend_meld(&meld_set, &c(50, Suit::Oros, 12)).is_none());
    }

    #[test]
    fn test_rearrange_existing_4card_meld() {
        // Existing table meld: 4-card set of 6s (Oros, Copas, Espadas, Bastos)
        let existing = vec![Meld {
            id: "m_existing".to_string(),
            kind: MeldKind::Set,
            cards: vec![
                c(6, Suit::Oros, 6),
                c(16, Suit::Copas, 6),
                c(26, Suit::Espadas, 6),
                c(36, Suit::Bastos, 6),
            ],
        }];
        // Player holds 5 of Oros (id 5) and another card 1 of Bastos (id 31)
        let hand = vec![c(5, Suit::Oros, 5), c(31, Suit::Bastos, 1)];
        // Offered card is 4 of Oros (id 4)
        let offered = c(4, Suit::Oros, 4);

        // Borrow 6 of Oros (id 6) from the 4-card set to pair with 4 & 5 of Oros!
        let proposed = vec![
            vec![16, 26, 36], // Remaining 3-card set of 6s
            vec![4, 5, 6],    // New 3-card run 4-5-6 of Oros
        ];
        let (melds, rem_hand) =
            validate_meld_submission(&offered, &hand, &existing, &[5], &proposed).unwrap();
        assert_eq!(melds.len(), 2);
        assert_eq!(rem_hand.len(), 1);
        assert_eq!(rem_hand[0].id, 31);
    }
}
