from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates

from webapp_project.utils import get_file_name_from_name_providers

app = FastAPI()


templates = Jinja2Templates(directory="templates")

# input du fichier
# choix de la taille des équipes
# choix du type de nom des équipes
# offset ?
@app.get("/", response_class=HTMLResponse)
def home_page(request: Request):
    sources = get_file_name_from_name_providers()
    context = {"request": request, "sources": sources}
    return templates.TemplateResponse("home/index.html", context)
