from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates

from webapp_project.utils import get_file_name_from_name_providers

app = FastAPI()
app.mount("/static", StaticFiles(directory="static"), name="static")

templates = Jinja2Templates(directory="templates")


@app.get("/", response_class=HTMLResponse)
def home_page(request: Request):
    sources = get_file_name_from_name_providers()
    context = {"request": request, "sources": sources}
    return templates.TemplateResponse("home/index.html", context)
