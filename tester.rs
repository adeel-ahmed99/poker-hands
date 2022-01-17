mod Poker;
use Poker::deal;
use std::collections::BTreeMap;

fn main()
{
    let perm: [u32; 9] = [40, 52, 46, 11, 48, 27, 29, 32, 37];
    let winner: [String; 5] = deal(perm);
}
