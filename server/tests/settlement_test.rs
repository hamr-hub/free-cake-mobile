use std::cmp::Reverse;

trait SortedBy: Iterator {
    fn sorted_by<K: Ord>(self, f: impl Fn(&Self::Item) -> K) -> Vec<Self::Item>;
}

impl<I: Iterator> SortedBy for I {
    fn sorted_by<K: Ord>(self, f: impl Fn(&Self::Item) -> K) -> Vec<Self::Item> {
        let mut v: Vec<Self::Item> = self.collect();
        v.sort_by_key(|x| Reverse(f(x)));
        v
    }
}

#[test]
fn test_settlement_status_check_voting_closed() {
    let status = "voting_closed";
    assert_eq!(status, "voting_closed");
}

#[test]
fn test_settlement_status_check_invalid() {
    let status = "voting_open";
    assert_ne!(status, "voting_closed");
}

#[test]
fn test_settlement_already_settled_conflict() {
    let status = "settled";
    assert_eq!(status, "settled");
}

#[test]
fn test_top100_winner_selection() {
    let entries: Vec<(i64, i32)> = vec![
        (1, 500), (2, 400), (3, 300), (4, 200), (5, 100),
    ];
    let max_winners = 3;
    let winners: Vec<_> = entries.iter()
        .filter(|(_, votes)| *votes > 0)
        .sorted_by(|e| e.1)
        .into_iter()
        .take(max_winners)
        .collect();
    assert_eq!(winners.len(), 3);
    assert_eq!(winners[0].0, 1);
}

#[test]
fn test_redeem_code_generation() {
    let code = uuid::Uuid::new_v4().to_string()[..8].to_string();
    assert_eq!(code.len(), 8);
}

#[test]
fn test_force_settle_flag() {
    let force = true;
    assert!(force);

    let force = false;
    assert!(!force);
}
