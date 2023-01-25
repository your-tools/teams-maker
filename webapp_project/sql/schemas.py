from typing import List

from pydantic import BaseModel


class NameBase(BaseModel):
    name: str


class NameCreate(NameBase):
    pass


class Name(NameBase):
    id: int
    name_provider_id: int

    class Config:
        orm_mode = True


class NameProviderBase(BaseModel):
    name: str


class NameProviderCreate(NameProviderBase):
    pass


class NameProvider(NameProviderBase):
    id: int
    names: List[Name] = []

    class Config:
        orm_mode: True
