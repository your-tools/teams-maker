from argparse import ArgumentParser
from copy import copy
from pathlib import Path
from random import shuffle

from faker import Faker


def compute_group_sizes(total: int, group_size: int) -> list[int]:
    res: list[int] = []
    # First off, create as many groups with the correct size that we can
    while sum(res) <= total - group_size:
        res.append(group_size)

    # total was divisible by group_size, we're done
    if sum(res) == total:
        return res

    remaining = total - sum(res)
    # Now we  have `remaining` people
    if remaining == 1:
        # Just one -> make the last group bigger
        res[-1] += 1
    else:
        # Less than one -> create a smaller group
        res.append(remaining)

    if len(res) < 3:
        return res

    # Last tweak: if the last two groups have a difference of
    # 2, increment the last and decrement the second to last
    if res[-2] - res[-1] == 2:
        res[-1] += 1
        res[-2] -= 1

    return res


def create_groups(students: list[str], group_size: int) -> list[list[str]]:
    group_sizes = compute_group_sizes(len(students), group_size)
    groups = []
    students_copy = copy(students)
    shuffle(students_copy)
    for group_size in group_sizes:
        group = []
        for _ in range(0, group_size):
            student = students_copy.pop()
            group.append(student)
        groups.append(group)

    return groups


def main() -> None:
    parser = ArgumentParser()
    parser.add_argument("student_list", type=Path)
    parser.add_argument("--group-size", type=int, required=True)
    parser.add_argument("--provider")
    args = parser.parse_args()

    list_path = args.student_list
    group_size = args.group_size
    students = list_path.read_text().splitlines()
    faker = Faker()

    groups = create_groups(students, group_size)
    for group in groups:
        group_name = faker.color_name()
        print("-" * 10, group_name, "-" * 10)
        for student in sorted(group):
            print(student)
