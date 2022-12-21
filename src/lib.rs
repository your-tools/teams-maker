#![allow(dead_code)]

use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Team {
    members: Vec<String>,
}

pub fn split_goup(group: &[&str], team_size: usize) -> Vec<Team> {
    let team_sizes = compute_team_sizes(group.len(), team_size);
    let participants = &mut group.to_vec();
    let mut rng = rand::thread_rng();
    participants.shuffle(&mut rng);
    let mut teams = vec![];
    for team_size in team_sizes {
        let mut members = vec![];
        for _ in 0..team_size {
            let member = participants.pop().unwrap();
            members.push(member.to_owned());
        }
        let team = Team { members };
        teams.push(team);
    }
    teams
}

fn compute_team_sizes(group_size: usize, team_size: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut current_sum = 0;
    // Create as many teams with the correct size as we can:
    while current_sum <= group_size - team_size {
        res.push(team_size);
        current_sum += team_size;
    }

    // Group size was divisible by team_size, we're done
    if current_sum == group_size {
        return res;
    }

    // Now we still have `remaining` people to put in a team
    let remaining = group_size - current_sum;

    if remaining == 1 {
        // If we have just one person with no team, just increase the last team
        let num_teams = res.len();
        res[num_teams - 1] += 1;
        return res;
    } else {
        // Otherwise, create a smaller team
        res.push(remaining);
    }

    if res.len() < 3 {
        return res;
    }

    // Last tweak: if the last two teams have a difference of 2,
    // increment the last and decrement the second to last
    let num_teams = res.len();
    if res[num_teams - 2] - res[num_teams - 1] == 2 {
        res[num_teams - 1] += 1;
        res[num_teams - 2] -= 1;
    }

    res
}

#[cfg(test)]
mod tests;
