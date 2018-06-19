/// A deck is basically just a vector of `card::Card`s with some fancy dressing.
use itertools::Itertools;

use card::*;

use rand::prng::XorShiftRng;
use rand::Rng;
use rand_core::SeedableRng;
use std::cmp;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Constructs a new deck with the standard 22 major arcana and 56 minor
    /// arcana. The order is unspecified but is *not* randomized.
    pub fn standard() -> Deck {
        let cards = iproduct!(Rank::standard(), Suit::standard())
            .map(|(&rank, &suit)| Card::Minor(MinorArcana { rank, suit }))
            .chain(MajorArcana::standard().map(|&arc| Card::Major(arc)))
            .collect();
        Deck { cards }
    }

    /// Constructs a new Silicon Dawn deck. The order is unspecified but is
    /// *not* randomized.
    pub fn silicon_dawn() -> Deck {
        let minor = iproduct!(Rank::standard(), Suit::standard())
            .map(|(&rank, &suit)| Card::Minor(MinorArcana { rank, suit }));
        let ninety_nines = Suit::standard().map(|&suit| {
            Card::Minor(MinorArcana {
                rank: Rank::NinetyNine,
                suit,
            })
        });
        let voids = [
            Rank::Zero,
            Rank::Progeny,
            Rank::Cavalier,
            Rank::Queen,
            Rank::King,
        ].into_iter()
            .map(|&rank| {
                Card::Minor(MinorArcana {
                    rank,
                    suit: Suit::VOID,
                })
            });
        let major = MajorArcana::standard()
            .chain(MajorArcana::silicon_dawn())
            .map(|&arc| Card::Major(arc));
        let extra = [Card::White, Card::Black].into_iter();
        let cards = minor
            .chain(ninety_nines)
            .chain(voids)
            .chain(major)
            .chain(extra.cloned())
            .collect();
        Deck { cards }
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn shuffle<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        if self.cards.len() == 1 {
            return;
        }
        // We'll cut the deck such the 'left' (i.e., top) half has cut_point cards in it.
        let cut_point = rng.gen_range(
            cmp::max(self.cards.len() / 2 - self.cards.len() / 6, 1),
            cmp::min(
                self.cards.len() / 2 + self.cards.len() / 6 + 1,
                self.cards.len(),
            ),
        );
        self.cards = {
            let left_half = &self.cards[0..cut_point];
            let right_half = &self.cards[cut_point..self.cards.len()];
            assert_ne!(left_half.len(), 0);
            assert_ne!(right_half.len(), 0);
            // The number of cards that fall at a time. In theory it'd be more
            // 'realistic' to let the number of chunks vary, but then we wouldn't
            // have this nice iterator-y implementation.
            let chunk_size = rng.gen_range(2, 4);
            let left_chunks = self.cards[..cut_point].chunks(chunk_size);
            let right_chunks = self.cards[cut_point..].chunks(chunk_size);
            right_chunks
                .interleave(left_chunks)
                .flatten()
                .cloned()
                .collect()
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn rng() -> XorShiftRng {
        XorShiftRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
    }

    #[test]
    fn count_standard_deck() {
        assert_eq!(Deck::standard().cards().len(), 78)
    }

    #[test]
    fn count_silicon_dawn() {
        assert_eq!(Deck::silicon_dawn().cards().len(), 94)
    }

    #[test]
    fn shuffle_one_card_deck() {
        Deck {
            cards: vec![Card::Black],
        }.shuffle(&mut rng())
    }

    #[test]
    fn shuffle_two_card_deck() {
        Deck {
            cards: vec![Card::Black],
        }.shuffle(&mut rng())
    }

    #[test]
    fn shuffle_two_card_deck_always_swaps() {
        let mut deck = Deck {
            cards: vec![Card::Black, Card::White],
        };
        deck.shuffle(&mut rng());
        assert_eq!(deck.cards, vec![Card::White, Card::Black]);
        deck.shuffle(&mut rng());
        assert_eq!(deck.cards, vec![Card::Black, Card::White]);
        deck.shuffle(&mut rng());
        assert_eq!(deck.cards, vec![Card::White, Card::Black]);
        deck.shuffle(&mut rng());
        assert_eq!(deck.cards, vec![Card::Black, Card::White]);
    }

    #[test]
    fn shuffle_four_card_deck_always_swaps_pairs() {
        let cards = vec![
            Card::Major(MajorArcana::Fool),
            Card::Major(MajorArcana::Magician),
            Card::Major(MajorArcana::HighPriestess),
            Card::Major(MajorArcana::Empress),
        ];
        let swapped = vec![
            Card::Major(MajorArcana::HighPriestess),
            Card::Major(MajorArcana::Empress),
            Card::Major(MajorArcana::Fool),
            Card::Major(MajorArcana::Magician),
        ];
        let mut deck = Deck {
            cards: cards.clone(),
        };
        deck.shuffle(&mut rng());
        assert_eq!(deck.cards, swapped);
        deck.shuffle(&mut rng());
        assert_eq!(deck.cards, cards);
        deck.shuffle(&mut rng());
        assert_eq!(deck.cards, swapped);
        deck.shuffle(&mut rng());
        assert_eq!(deck.cards, cards);
    }
}
