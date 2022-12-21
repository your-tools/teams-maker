use super::*;

#[test]
fn test_split_group_in_teams() {
    let group = vec!["Alice", "Bob", "Charlie", "Dave", "Eve", "Fabrice"];
    let teams = split_goup(&group, 3);
    assert_eq!(teams.len(), 2);
}

#[test]
fn test_compute_team_sizes_when_divisible() {
    assert_eq!(compute_team_sizes(16, 4), vec![4, 4, 4, 4]);
}

#[test]
fn test_compute_team_sizes_when_one_less() {
    assert_eq!(compute_team_sizes(15, 4), vec![4, 4, 4, 3]);
}

#[test]
fn test_compute_team_sizes_when_one_more() {
    assert_eq!(compute_team_sizes(17, 4), vec![4, 4, 4, 5]);
}

#[test]
fn test_compute_team_sizes_adjusting_last_two() {
    assert_eq!(compute_team_sizes(18, 4), vec![4, 4, 4, 3, 3]);
}
