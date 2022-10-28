# Groups maker


Create a list of groups from a list of students

## Usage

Put all the students in a file, one per line.

Then run:

```
$ groups-maker STUDENT_LIST_PATH --group-size <GROUP_SIZE>
```

You'll get an output looking like this:

```text
 ---------- Green ----------
Bailey
Peter
Randy
Ronald
Taylor
---------- Blue ----------
Christine
Jesse
Katherine
Melissa
Ryan
```


## Contributing

* Install [poetry](https://python-poetry.org/)
* Install required dependencies

```
poetry install --group dev
```

Before submitting a change, run the following commands:

```
poetry run invoke lint
poetry run pytest
```
