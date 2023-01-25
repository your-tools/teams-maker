from fastapi import FastAPI, Request, UploadFile, Form, Depends, HTTPException
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from teams_maker.teams import create_teams, get_team_name
from webapp_project.utils import (
    PATH_TO_NAME_PROVIDERS,
    get_file_names_from_name_providers,
    read_decode_and_splitlines,
)
from sqlalchemy.orm import Session
from sql import models, crud, schemas
from sql.database import SessionLocal, engine

models.Base.metadata.create_all(bind=engine)

app = FastAPI()
app.mount("/static", StaticFiles(directory="static"), name="static")

templates = Jinja2Templates(directory="templates")


def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


@app.get("/")
def home_page(request: Request):
    sources = get_file_names_from_name_providers()
    context = {"request": request, "sources": sources}
    return templates.TemplateResponse("home/index.html", context)


@app.post("/teams/create/")
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


@app.get("/teams/names-providers/")
@app.get("/teams/name-provider/")
def form_to_create_name_provider(request: Request):
    context = {"request": request}
    return templates.TemplateResponse("teams/name-provider.html", context)


@app.post("/teams/name-provider/")
def create_a_name_provider(
    request: Request,
    name: str = Form(...),
    db: Session = Depends(get_db),
):
    crud.create_name_provider(db, schemas.NameProviderCreate(name=name))
    names_providers = crud.get_names_providers(db=db)

    context = {
        "request": request,
        "names_providers": names_providers,
    }

    return templates.TemplateResponse("teams/names-providers.html", context)
