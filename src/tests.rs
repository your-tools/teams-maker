use super::*;

#[test]
fn test_split_group_in_teams() {
    let group = vec!["Alice", "Bob", "Charlie", "Dave", "Eve", "Fabrice"];
    let teams = split_goup(&group, 3).unwrap();
    assert_eq!(teams.len(), 2);
}

#[test]
fn test_compute_team_sizes_when_divisible() {
    assert_eq!(compute_team_sizes(16, 4).unwrap(), vec![4, 4, 4, 4]);
}

#[test]
fn test_compute_team_sizes_when_one_less() {
    assert_eq!(compute_team_sizes(15, 4).unwrap(), vec![4, 4, 4, 3]);
}

#[test]
fn test_compute_team_sizes_when_one_more() {
    assert_eq!(compute_team_sizes(17, 4).unwrap(), vec![4, 4, 4, 5]);
}

#[test]
fn test_compute_team_sizes_adjusting_last_two() {
    assert_eq!(compute_team_sizes(18, 4).unwrap(), vec![4, 4, 4, 3, 3]);
}

#[test]
#[ignore = "not working yet"]
fn test_compute_team_sizes_adjust_last_three() {
    assert_eq!(compute_team_sizes(30, 12).unwrap(), vec![11, 11, 8]);
}

#[test]
fn test_compute_team_sizes_() {
    assert_eq!(compute_team_sizes(31, 30).unwrap(), vec![31]);
}

use proptest::prelude::*;

proptest! {
    #[test]
    fn test_compute_team_sizes_does_not_crash(group_size in 0usize..100usize, team_size in 0usize..100usize) {
        let outcome = compute_team_sizes(group_size, team_size);
        if let Ok(mut team_sizes) = outcome {
            team_sizes.sort();
            let actual_sum =team_sizes.iter().sum::<usize>();
            assert_eq!(actual_sum, group_size, "Expected {team_sizes:?} to have a sum of {group_size}, got {actual_sum}");
        }
    }
}
