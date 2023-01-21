from fastapi import FastAPI, Request, UploadFile, Form
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from teams_maker.teams import create_teams, get_team_name

from webapp_project.utils import (
    PATH_TO_NAME_PROVIDERS,
    get_file_names_from_name_providers,
    read_decode_and_splitlines,
)

app = FastAPI()
app.mount("/static", StaticFiles(directory="static"), name="static")

templates = Jinja2Templates(directory="templates")


@app.get("/", response_class=HTMLResponse)
def home_page(request: Request):
    sources = get_file_names_from_name_providers()
    context = {"request": request, "sources": sources}
    return templates.TemplateResponse("home/index.html", context)


@app.post("/teams/create")
def teams_create(
    request: Request,
    file: UploadFile = Form(...),
    team_size: int = Form(...),
    source: str = Form(...),
):
    participants = read_decode_and_splitlines(file)
    teams = create_teams(participants, team_size)
    offset = 0
    teams_names = []
    source_path = PATH_TO_NAME_PROVIDERS + source

    for index in range(len(teams)):
        team_name = get_team_name(source_path=source_path, index=index, offset=offset)
        teams_names.append(team_name)

    context = {
        "request": request,
        "team_size": team_size,
        "teams_names": teams_names,
        "teams": teams,
    }

    return templates.TemplateResponse("teams/teams.html", context)
