# teams-maker


Create a list of teams with funny names from a list of participants

## Usage

Put all the participants in a file, one per line.

Then run:

```
$ teams-maker PARTICIPANTS_PATH --source colors --team-size <TEAM_SIZE>
```

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

* Install [poetry](https://python-poetry.org/)
* Install required dependencies

```
poetry install
```

Before submitting a change, run the following commands:

```
poetry run invoke lint
poetry run pytest
```
