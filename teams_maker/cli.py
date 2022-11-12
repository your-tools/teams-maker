from argparse import ArgumentParser
from pathlib import Path

from teams_maker.teams import create_teams, get_team_name


def main() -> None:
    parser = ArgumentParser(prog="teams-maker")
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


if __name__ == "__main__":
    main()
