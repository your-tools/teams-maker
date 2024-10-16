from copy import copy
from pathlib import Path
from random import shuffle


def compute_team_sizes(total: int, team_size: int) -> list[int]:
    res: list[int] = []
    # First off, create as many teams with the correct size that we can
    while sum(res) <= total - team_size:
        res.append(team_size)

    # total was divisible by team_size, we're done
    if sum(res) == total:
        return res

    remaining = total - sum(res)
    # Now we  have `remaining` people
    if remaining == 1:
        # Just one -> make the last team bigger
        res[-1] += 1
    else:
        # Less than one -> create a smaller team
        res.append(remaining)

    if len(res) < 3:
        return res

    # Last tweak: if the last two teams have a difference of
    # 2, increment the last and decrement the second to last
    if res[-2] - res[-1] == 2:
        res[-1] += 1
        res[-2] -= 1

    return res


def create_teams(participants: list[str], team_size: int) -> list[list[str]]:
    participants_copy = copy(participants)
    team_sizes = compute_team_sizes(len(participants), team_size)
    teams = []
    shuffle(participants_copy)
    for team_size in team_sizes:
        team = []
        for _ in range(0, team_size):
            participant = participants_copy.pop()
            team.append(participant)
        teams.append(team)

    return teams

def read_name_provider(name: str) -> list[str]:
    this_path = Path(__file__).parent
    lines = (
        (this_path / "name_providers" / name)
        .read_text()
        .splitlines(keepends=False)
    )
    return  lines

def get_team_name(*, name_provider: str, index: int = 0, offset: int = 0) -> str:
    lines = read_name_provider(name=name_provider)
    return lines[index + offset]

def check_name_provider(name_provider: str, *, num_teams: int):
    lines = read_name_provider(name=name_provider)
    if len(lines) < num_teams:
        raise ValueError(f"name provider '{name_provider}' only contains {len(lines)} psosible team names, but you need {num_teams}")

