from itertools import chain

from faker import Faker

from teams_maker.teams import (
    compute_team_sizes,
    create_teams,
    get_team_name,
    check_name_provider,
)
import pytest


def test_compute_team_size_when_divisible() -> None:
    assert compute_team_sizes(16, 4) == [4, 4, 4, 4]


def test_compute_team_size_when_one_less() -> None:
    assert compute_team_sizes(15, 4) == [4, 4, 4, 3]


def test_compute_team_size_when_one_more() -> None:
    assert compute_team_sizes(17, 4) == [4, 4, 4, 5]


def test_compute_team_size_when_mod_two() -> None:
    assert compute_team_sizes(18, 4) == [4, 4, 4, 3, 3]


def test_create_random_teams() -> None:
    faker = Faker()
    participants = [f"{i:02}-{faker.first_name()}" for i in range(0, 15)]

    teams = create_teams(participants, 4)

    in_teams = list(chain(*teams))
    assert sorted(in_teams) == sorted(participants)


def test_get_team_name_from_colors_txt() -> None:
    name = get_team_name(name_provider="colors", index=0, offset=0)
    assert name == "Almond"


def test_check_name_provider_has_enough_names():
    check_name_provider("colors", num_teams=25)

    with pytest.raises(ValueError):
        check_name_provider("colors", num_teams=100)
