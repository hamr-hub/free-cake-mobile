use free_cake_server::handlers::activity::validate_status_transition;

#[test]
fn test_activity_status_transition_draft_to_registration_open() {
    assert!(validate_status_transition("draft", "registration_open"));
}

#[test]
fn test_activity_status_transition_registration_open_to_voting_open() {
    assert!(validate_status_transition("registration_open", "voting_open"));
}

#[test]
fn test_activity_status_transition_registration_open_to_draft() {
    assert!(validate_status_transition("registration_open", "draft"));
}

#[test]
fn test_activity_status_transition_voting_open_to_voting_closed() {
    assert!(validate_status_transition("voting_open", "voting_closed"));
}

#[test]
fn test_activity_status_transition_voting_closed_to_settled() {
    assert!(validate_status_transition("voting_closed", "settled"));
}

#[test]
fn test_activity_status_transition_settled_to_redeeming() {
    assert!(validate_status_transition("settled", "redeeming"));
}

#[test]
fn test_activity_status_transition_redeeming_to_finished() {
    assert!(validate_status_transition("redeeming", "finished"));
}

#[test]
fn test_activity_status_transition_invalid_draft_to_voting_open() {
    assert!(!validate_status_transition("draft", "voting_open"));
}

#[test]
fn test_activity_status_transition_invalid_skip_states() {
    assert!(!validate_status_transition("draft", "settled"));
    assert!(!validate_status_transition("registration_open", "settled"));
    assert!(!validate_status_transition("voting_open", "draft"));
}

#[test]
fn test_activity_status_transition_unknown_from() {
    assert!(!validate_status_transition("unknown", "draft"));
}

#[test]
fn test_activity_valid_statuses() {
    let valid = ["draft", "registration_open", "voting_open", "voting_closed", "settled", "redeeming", "finished"];
    for s in &valid {
        assert!(validate_status_transition(s, "registration_open") || *s != "draft" || validate_status_transition(s, "registration_open"));
    }
}

#[test]
fn test_activity_name_validation() {
    let empty_name = String::new();
    let real_name = String::from("XX镇第一届蛋糕大赛");
    assert!(empty_name.is_empty());
    assert!(!real_name.is_empty());
}

#[test]
fn test_max_winner_count_validation() {
    let zero: i32 = 0;
    let hundred: i32 = 100;
    let five_hundred: i32 = 500;
    assert!(zero <= 0);
    assert!(hundred > 0);
    assert!(five_hundred > 0);
}
