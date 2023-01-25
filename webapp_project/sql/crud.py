from sqlalchemy.orm import Session

from . import models, schemas

# Create
def create_name_provider(db: Session, name_provider: schemas.NameProviderCreate):
    db_name_provider = models.NameProvider(**name_provider.dict())
    db.add(db_name_provider)
    db.commit()
    db.refresh(db_name_provider)
    return db_name_provider

# Read
def get_a_name_provider(db: Session, name_provider_id: int):
    return (
        db.query(models.NameProvider)
        .filter(models.NameProvider.id == name_provider_id)
        .first()
    )


def get_names_providers(db: Session):
    return db.query(models.NameProvider).all()
