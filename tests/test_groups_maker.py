from itertools import chain

from faker import Faker

from groups_maker import compute_group_sizes, create_groups


def test_compute_group_size_when_divisible() -> None:
    assert compute_group_sizes(16, 4) == [4, 4, 4, 4]


def test_compute_group_size_when_one_less() -> None:
    assert compute_group_sizes(15, 4) == [4, 4, 4, 3]


def test_compute_group_size_when_one_more() -> None:
    assert compute_group_sizes(17, 4) == [4, 4, 4, 5]


def test_compute_group_size_when_mod_two() -> None:
    assert compute_group_sizes(18, 4) == [4, 4, 4, 3, 3]


def test_create_random_groups() -> None:
    faker = Faker()
    students = [f"{i:02}-{faker.first_name()}" for i in range(0, 15)]

    groups = create_groups(students, 4)

    students_in_groups = list(chain(*groups))
    assert sorted(students_in_groups) == sorted(students)
