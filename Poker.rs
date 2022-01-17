/**
    Adeel Ahmed
*/

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

pub fn deal(perm: [u32; 9]) -> [String; 5]
{
    let cards = perm.to_vec();
    let (first, second) = split(cards);

    let mut p1 = build_player(first);
    p1.determine_hand();

    let mut p2 = build_player(second);
    p2.determine_hand();

    if &p1.rank > &p2.rank {
        return pop(&p2.best_hand);
    }
    else if &p1.rank < &p2.rank {
        return pop(&p1.best_hand);
    }
    else {
        let result: u32 = tie_break(&p1.best_hand, &p2.best_hand);
        if result == 1 {
            return pop(&p1.best_hand);
        }
        else {
            return pop(&p2.best_hand);
        }
    }
}

fn pop(cards: &Vec<u32>) -> [String; 5] {
    let mut strings: Vec<String> = Vec::new();
    for card in cards.iter() {
        strings.push(card_to_string(*card))
    }
    let mut hand: [String; 5] = [strings[0].clone(), strings[1].clone(), strings[2].clone(), strings[3].clone(), strings[4].clone()];
    hand.sort();
    hand
}

fn build_player(cards: Vec<u32>) -> Hand
{
    let mut player = Hand {
        cards,
        ranks: BTreeMap::new(),
        suits: BTreeMap::new(),
        straight: 0,
        rank: 0,
        best_hand: Vec::new(),
    };
    player.count_ranks();
    player.count_suits();
    player
}

// to split 9 card input into two 7 card hands for each player
fn split(pool: Vec<u32>) -> (Vec<u32>, Vec<u32>)
{
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();
    first.push(pool[0]);
    first.push(pool[2]);
    first.extend_from_slice(&pool[4..]);

    second.push(pool[1]);
    second.push(pool[3]);
    second.extend_from_slice(&pool[4..]);

    (first, second)
}

fn get_rank(card: u32) -> u32
{
    if card % 13 == 0 { 13 } else { card % 13 }
}

fn get_suit(card: u32) -> String
{
    let number = ((card - 1)/ 13) + 1;
    match number {
        1 => String::from("C"),
        2 => String::from("D"),
        3 => String::from("H"),
        4 => String::from("S"),
        _ => "ERROR".to_string()
    }
}

fn card_to_string(card: u32) -> String
{
    let mut rank: String = get_rank(card).to_string();
    let suit: String = get_suit(card);
    rank.push_str(&suit);
    rank
}

fn sorted_by_rank(cards: Vec<u32>) -> Vec<u32>
{
    let mut sorted: Vec<u32> = cards.clone();

    for num in sorted.iter_mut() {
        *num = get_rank(*num);
    }
    sorted.sort();
    sorted
}

fn ace_sort(cards: &Vec<u32>) -> Vec<u32>
{
    let mut cpy: Vec<u32> = cards.clone();
    let mut sorted: Vec<u32> = Vec::new();

    // collect all Aces
    for num in cpy.iter() {
        if get_rank(*num) == 1 {
            sorted.push(*num);
        }
    }
    let mut idx = 0;
    while cpy.len() > idx {
        if get_rank(cpy[idx]) == 1 {cpy.remove(idx);}
        else {idx += 1;}
    }
    // sort and add non aces
    cpy.sort_by_key(|x| get_rank(*x));
    cpy = cpy.into_iter().rev().collect();
    sorted.extend_from_slice(&cpy);
    sorted
}

fn tie_break(p1: &Vec<u32>, p2: &Vec<u32>) -> u32 {
    let mut idx = 0;

    while 5 > idx {
        if get_rank(p1[idx]) == 1 || get_rank(p2[idx]) == 1
        {
            if get_rank(p1[idx]) == get_rank(p2[idx]) {
                idx += 1;
                continue;
            }
            else if get_rank(p1[idx]) == 1 {
                return 1;
            }
            else {
                return 2;
            }
        }
        else {
            if get_rank(p1[idx]) == get_rank(p2[idx]) {
                idx += 1;
                continue;
            }
            else if get_rank(p1[idx]) >= get_rank(p2[idx]) {
                return 1;
            }
            else {
                return 2;
            }
        }
    }
    1
}

struct Hand {
    cards: Vec<u32>,
    ranks: BTreeMap<u32, u32>,
    suits: BTreeMap<String, u32>,
    straight: u32,
    rank: u32,
    best_hand: Vec<u32>,
}

impl Hand {
    fn to_string(&self) -> Vec<String> {
        let mut hand_string: Vec<String> = Vec::new();

        for card in &self.best_hand {
            hand_string.push(card_to_string(*card));
        }
        hand_string
    }

    fn count_ranks(&mut self) {
        let mut map: BTreeMap<u32, u32> = BTreeMap::new();

        for card in self.cards.iter() {
            let key = get_rank(*card);
            if map.contains_key(&key) {
                let value = map.get(&key).unwrap() + 1;
                map.insert(key, value);
            }
            else {
                map.insert(get_rank(*card), 1);
            }
        }
        self.ranks = map;
    }

    fn count_suits(&mut self) {
        let mut map: BTreeMap<String, u32> = BTreeMap::new();

        for card in self.cards.iter() {
            let key = get_suit(*card);
            if map.contains_key(&key) {
                let value = map.get(&key).unwrap() + 1;
                map.insert(key, value);
            }
            else {
                map.insert(get_suit(*card), 1);
            }
        }
        self.suits = map;
    }

    fn print(&self) {
        println!("cards: {:?}", self.cards);
        println!("ranks: {:?}", self.ranks);
        println!("suits: {:?}", self.suits);
        println!("straight: {}", self.straight);
        println!("RANK: {}", self.rank);
        println!("BEST HAND: {:?}", self.best_hand);
    }

    fn determine_hand(&mut self) {

        if self.is_straight_flush() {self.rank = 1; self.get_straight_flush()}
        else if self.is_four_of_a_kind() {self.rank = 2; self.get_four_of_a_kind();}
        else if self.is_full_house() {self.rank = 3; self.get_full_house();}
        else if self.is_flush() {self.rank = 4; self.get_flush();}
        else if self.is_straight() {self.rank = 5; self.get_straight();}
        else if self.is_three_of_a_kind() {self.rank = 6; self.get_three_of_a_kind();}
        else if self.is_two_pair() {self.rank = 7; self.get_two_pair();}
        else if self.is_pair() {self.rank = 8; self.get_pair();}
        else if self.is_high_card() {self.rank = 9; self.get_high_card();}
        else {println!("broken input");}
    }

    fn is_straight_flush(&mut self) -> bool {
        let mut suit: String = String::new();
        let mut foundFlush = false;
        let mut foundStraight = false;
        for (key, value) in &self.suits {
            if *value >= 5 {
                suit = key.clone();
                foundFlush = true;
                break;
            }
        }
        if !foundFlush { return false; }

        let mut suitedPool: Vec<u32> = self.cards.clone().into_iter().filter(|x| get_suit(*x) == suit).collect();

        let mut consec = 0;

        for num in suitedPool.iter_mut() {
            *num = get_rank(*num);
        }

        let mut newList: Vec<u32> = sorted_by_rank(suitedPool);
        newList.dedup();

        if 5 > newList.len() {return false;}
        let mut has_one = false;
        if newList[0] == 1 {newList.remove(0); has_one = true;}
        let mut consec = 0;
        let mut previous = newList[0] - 1;


        for num in newList.iter() {
            if (*num - 1 == previous) {consec += 1;}
            else {consec = 1;}
            if consec >= 5 {self.straight = *num; foundStraight = true;}
            previous = *num;
        }
        // check for Ace high straight
        if(has_one) {
            newList.insert(0, 1);
            let mut set: HashSet<u32> = newList.into_iter().collect();
            if set.contains(&1) && set.contains(&10) && set.contains(&11) && set.contains(&12) && set.contains(&13) {
                self.straight = 1;
                foundStraight = true;
            }
        }

        foundFlush && foundStraight
    }

    fn get_straight_flush(&mut self) {
        let mut hand: Vec<u32> = Vec::new();
        let mut suit: String = String::new();
        let mut straight: Vec<u32> = Vec::new();

        for (key, value) in &self.suits {
            if *value >= 5 {suit = key.clone().to_string();}
        }

        if self.straight == 1 {
            straight = vec![1, 10, 11, 12 ,13];
        }
        else {
            for i in (self.straight-4..self.straight+1) {
                straight.push(i);
            }
        }
        for rank in straight {
            for card in &self.cards {
                if rank == get_rank(*card) && suit == get_suit(*card) {
                    hand.push(*card);
                    break;
                }
            }
        }
        self.best_hand = hand;
        return;
    }

    fn is_four_of_a_kind(&self) -> bool {
        let values: Vec<u32> = self.ranks.values().cloned().collect();
        let mut found = false;

        for num in values.iter() {
            if *num == 4 {found = true; break;}
        }
        found
    }

    fn get_four_of_a_kind(&mut self) {
        let mut quad = 0;
        let mut hand: Vec<u32> = Vec::new();
        let mut cpy = self.cards.clone();
        for (key, value) in &self.ranks {
            if *value == 4 {quad = *key; break;}
        }

        // add quads
        let mut idx = 0;
        while cpy.len() > idx {
            if get_rank(cpy[idx]) == quad {
                hand.push(cpy[idx]);
                cpy.remove(idx);
            }
            else {idx += 1;}
        }
        hand.extend_from_slice(&ace_sort(&cpy)[..1]);
        self.best_hand = hand;
    }

    fn is_full_house(&self) -> bool {
        let values: Vec<u32> = self.ranks.values().cloned().collect();
        let mut found = false;

        for num in values.iter() {
            if *num == 3 {found = true; break;}
        }

        found && (5 > values.len())
    }

    fn get_full_house(&mut self) {
        let mut hand: Vec<u32> = Vec::new();
        let mut trips: Vec<u32> = Vec::new();
        let mut cpy = self.cards.clone();
        let mut threeCnt = 0;

        for (key, value) in &self.ranks {
            if *value == 3 {trips.push(*key);}
        }

        // two sets of trips
        if trips.len() > 1 {
            trips = ace_sort(&trips);

            // add 3 of higher card
            for card in &self.cards {
                if get_rank(*card) == trips[0] {hand.push(*card);}
            }
            // add 2 of lower card
            for card in &self.cards {
                if hand.len() == 5 {break;}
                if get_rank(*card) == trips[1] {hand.push(*card);}
            }
        } else {
            // add trips
            for card in &self.cards {
                if get_rank(*card) == trips[0] {hand.push(*card);}
            }
            // find pairs (could be multiple)
            let mut pairs: Vec<u32> = Vec::new();
            for (key, value) in &self.ranks {
                if *value == 2 {pairs.push(*key);}
            }
            pairs = ace_sort(&pairs);
            for card in &self.cards {
                if get_rank(*card) == pairs[0] {hand.push(*card);}
            }
        }
        self.best_hand = hand;
    }

    fn is_flush(&self) -> bool {
        let values: Vec<u32> = self.suits.values().cloned().collect();
        let mut found = false;

        for num in values.iter() {
            if *num >= 5 {found = true; break;}
        }
        found
    }

    fn get_flush(&mut self) {
        let mut suit: String = String::new();
        let mut hand: Vec<u32> = Vec::new();

        for (key, value) in &self.suits {
            if *value >= 5 {suit = key.clone().to_string();}
        }
        for card in &self.cards {
            if suit == get_suit(*card) {hand.push(*card);}
        }
        hand = ace_sort(&hand)[..5].to_vec();
        self.best_hand = hand;
    }

    fn is_straight(&mut self) -> bool {
        let mut consec = 0;
        let mut foundStraight = false;

        let mut sorted: Vec<u32> = sorted_by_rank(self.cards.clone());
        sorted.dedup();

        if 5 > sorted.len() {return false;}
        let mut has_one = false;
        if sorted[0] == 1 {
            sorted.remove(0);
            has_one = true;
        }
        let mut previous = sorted[0] - 1;
        let mut consec = 0;

        for num in sorted.iter() {
            if (*num - 1 == previous) {consec += 1;}
            else {consec = 1;}
            if consec >= 5 {self.straight = *num; foundStraight = true;}
            previous = *num;
        }

        // detect 10-J-Q-K-A case using sets
        if(has_one) {
            sorted.insert(0, 1);
            let mut set: HashSet<u32> = sorted.into_iter().collect();
            if set.contains(&1) && set.contains(&10) && set.contains(&11) && set.contains(&12) && set.contains(&13) {
                self.straight = 1;
                foundStraight = true;;
            }
        }

        foundStraight
    }

    fn get_straight(&mut self) {
        let mut hand: Vec<u32> = Vec::new();
        let mut straight: Vec<u32> = Vec::new();
        if self.straight == 1 {
            straight = vec![1, 10, 11, 12, 13];
        }
        else {
            for i in (self.straight-4..self.straight+1) {
                straight.push(i);
            }
        }
        for rank in straight {
            for card in &self.cards {
                if rank == get_rank(*card) {
                    hand.push(*card);
                    break;
                }
            }
        }
        self.best_hand = hand;
    }

    fn is_three_of_a_kind(&self) -> bool {
        let values: Vec<u32> = self.ranks.values().cloned().collect();
        let mut found = false;
        let mut count = 0;

        for num in values.iter() {
            if *num == 3 {found = true; count += 1;}
        }
        found && (count == 1)
    }

    fn get_three_of_a_kind(&mut self) {
        let mut trip = 0;
        let mut hand: Vec<u32> = Vec::new();
        let mut cpy = self.cards.clone();
        for (key, value) in &self.ranks {
            if *value == 3 {trip = *key; break;}
        }
        // add trips
        let mut idx = 0;
        while cpy.len() > idx {
            if get_rank(cpy[idx]) == trip {
                hand.push(cpy[idx]);
                cpy.remove(idx);
            }
            else {idx += 1;}
        }
        hand.extend_from_slice(&ace_sort(&cpy)[..2]);
        self.best_hand = hand;
    }

    fn is_two_pair(&self) -> bool {
        let values: Vec<u32> = self.ranks.values().cloned().collect();
        let mut count = 0;

        for num in values.iter() {
            if *num == 2 {count += 1;}
        }

        count >= 2
    }

    fn get_two_pair(&mut self) {
        // get pairs (could be 3)
        let mut pairs = Vec::new();
        let mut hand: Vec<u32> = Vec::new();
        let mut cpy   = self.cards.clone();

        for (key, value) in &self.ranks {
            if *value == 2 {pairs.push(*key);}
        }
        pairs = ace_sort(&pairs);
        // add both pairs and remove them from cards
        let mut idx = 0;
        while cpy.len() > idx {
            if get_rank(cpy[idx]) == pairs[0] {
                hand.push(cpy[idx]);
                cpy.remove(idx);
            }
            else {idx += 1;}
        }
        idx = 0;

        while cpy.len() > idx {
            if get_rank(cpy[idx]) == pairs[1] {
                hand.push(cpy[idx]);
                cpy.remove(idx);
            }
            else {idx += 1;}
        }
        hand.extend_from_slice(&ace_sort(&cpy)[..1]);
        self.best_hand = hand;
    }

    fn is_pair(&self) -> bool {
        let values: Vec<u32> = self.ranks.values().cloned().collect();
        let mut count = 0;
        let mut found = false;

        for num in values.iter() {
            if *num == 2 {count += 1; found = true;}
        }

        found && (count == 1)
    }

    fn get_pair(&mut self) {
        // get pair
        let mut pair = 0;
        let mut hand = Vec::new();
        let mut cpy = self.cards.clone();
        for (key, value) in &self.ranks {
            if *value == 2 {pair = *key; break;}
        }

        // add pairs
        let mut idx = 0;
        while cpy.len() > idx {
            if get_rank(cpy[idx]) == pair {
                hand.push(cpy[idx]);
                cpy.remove(idx);
            }
            else {idx += 1;}
        }
        hand.extend_from_slice(&ace_sort(&cpy)[..3]);
        self.best_hand = hand;
    }

    fn is_high_card(&self) -> bool {
        let values: Vec<u32> = self.ranks.values().cloned().collect();
        let mut error = false;

        for num in values.iter() {
            if *num != 1 {error = true; break;}
        }

        true
    }

    fn get_high_card(&mut self) {
        self.best_hand = ace_sort(&self.cards).into_iter().take(5).collect();
    }
}
