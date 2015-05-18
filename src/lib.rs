extern crate cards;
extern crate holdem;

mod types;
mod lookups;
mod original;
//mod perfect;
pub mod utils;

use cards::card::{Card};
use holdem::{HandCards, CommunityCards};

use holdem::{HandRankClass};

pub type HandRank = u32;

pub fn eval_5cards(cards: [&Card; 5]) -> HandRank {
    original::eval_5cards(cards)
}

pub fn eval_7cards(cards: [&Card; 7]) -> HandRank {
    original::eval_7cards(cards)
}

//TODO: is panic an acceptable behaviour here? Be more verbose about this function expecting all cards to be present?
pub fn eval_for_player(player_cards: &HandCards, community_cards: &CommunityCards) -> HandRank {
    let cards : [&Card; 7] = [
        &player_cards.0.expect_borrow(),
        &player_cards.1.expect_borrow(),
        &community_cards.0.expect_borrow(),
        &community_cards.1.expect_borrow(),
        &community_cards.2.expect_borrow(),
        &community_cards.3.expect_borrow(),
        &community_cards.4.expect_borrow()
    ];

    eval_7cards(cards)
}

pub fn hand_rank(val: HandRank) -> HandRankClass {
    if val > 6185 {
        return HandRankClass::HighCard        // 1277 high card
    } else if val > 3325 {
        return HandRankClass::OnePair         // 2860 one pair
    } else if val > 2467 {
        return HandRankClass::TwoPair         //  858 two pair
    } else if val > 1609 {
        return HandRankClass::ThreeOfAKind    //  858 three-kind
    } else if val > 1599 {
        return HandRankClass::Straight        //   10 straights
    } else if val > 322 {
        return HandRankClass::Flush           // 1277 flushes
    } else if val > 166 {
        return HandRankClass::FullHouse       //  156 full house
    } else if val > 10 {
        return HandRankClass::FourOfAKind     //  156 four-kind
    } else {
        HandRankClass::StraightFlush          //   10 straight-flushes
    }
}
