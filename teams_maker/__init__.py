from argparse import ArgumentParser
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


def get_team_name(*, source: str, index: int = 0, offset: int = 0) -> str:
    lines = Path(f"teams_maker/{source}").read_text().splitlines(keepends=False)
    return lines[index + offset]


def main() -> None:
    parser = ArgumentParser()
    parser.add_argument("partipant_list", type=Path)
    parser.add_argument("--team-size", type=int, required=True)
    parser.add_argument("--source")
    parser.add_argument("--offset", type=int, default=0)
    args = parser.parse_args()

    list_path = args.partipant_list
    team_size = args.team_size
    source = args.source
    offset = args.offset
    participants = list_path.read_text().splitlines()

    teams = create_teams(participants, team_size)
    for index, team in enumerate(teams):
        team_name = get_team_name(source=source, index=index, offset=offset)
        print("-" * 10, "Team", team_name, "-" * 10)
        for partipant in sorted(team):
            print(partipant)
        print()
