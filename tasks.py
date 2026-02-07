# type: ignore
from invoke import call, task

SOURCES = "teams_maker tests"


@task
def black(c, check=False):
    print("Running black")
    cmd = f"black {SOURCES}"
    if check:
        cmd += " --check"
    c.run(cmd)


@task
def isort(c, check=False):
    print("Running isort")
    cmd = f"isort {SOURCES}"
    if check:
        cmd += " --check"
    c.run(cmd)


@task
def flake8(c):
    print("Running flake8")
    c.run(f"flake8 {SOURCES}")


@task
def mypy(c):
    print("Running mypy")
    cmd = "mypy"
    c.run(cmd)


@task
def test(c):
    print("Running pytest")
    c.run("pytest", pty=True)


@task(
    pre=[
        call(black, check=True),
        call(isort, check=True),
        call(flake8),
        call(mypy),
    ]
)
def lint(c):
    pass
