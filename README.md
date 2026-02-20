# teams-maker


**This repo has moved to [codeberg](https://codeberg.org/your-tools/teams-maker) - see you there !**

Create a list of teams with funny names from a list of participants

## Installation and usage

- Install [poetry](https://python-poetry.org/)
- Install required dependencies

```
poetry install
```

Put all the participants in a file, one per line, says in
`participants.txt`

Then run:

```
$ teams-maker participants --name-provider <PROVIDER> --team-size <TEAM_SIZE>
```

Where `<PROVIDER>` matches a file in the `teams_maker/name_providers` directory.

You'll get an output looking like this:

```text
 ---------- Team Almond ----------
Bailey
Peter
Randy
Ronald
Taylor
---------- Team Blue ----------
Christine
Jesse
Katherine
Melissa
Ryan
```

Note that some teams may have a little more or a little less than members
than `TEAM_SIZE`.

## Contributing

Before submitting a change, run the following commands:

```
poetry run invoke lint
poetry run pytest
```
