from sqlalchemy import Column, ForeignKey, Integer, String
from sqlalchemy.orm import relationship

from .database import Base


class NameProvider(Base):
    __tablename__ = "names_providers"

    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, index=True)

    names = relationship("Name", back_populates="name_provider")


class Name(Base):
    __tablename__ = "names"

    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, index=True)
    name_provider_id = Column(Integer, ForeignKey("names_providers.id"))

    name_provider = relationship("NameProvider", back_populates="names")
