"""FastAPI routers for REST API"""

from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.ext.asyncio import AsyncSession

# SOW Router
sow_router = APIRouter(prefix="/sows", tags=["sows"])

@sow_router.get("/")
async def list_sows(db: AsyncSession = Depends(get_db)):
    """List all SOWs"""
    return {"sows": []}

@sow_router.post("/")
async def create_sow(data: dict, db: AsyncSession = Depends(get_db)):
    """Create new SOW"""
    return {"id": "uuid"}

# Noun Router
noun_router = APIRouter(prefix="/nouns/{sow_id}/nouns", tags=["nouns"])

@noun_router.get("/")
async def list_nouns(sow_id: str, db: AsyncSession = Depends(get_db)):
    """List nouns in SOW"""
    return {"nouns": []}

@noun_router.post("/")
async def create_noun(sow_id: str, data: dict, db: AsyncSession = Depends(get_db)):
    """Create noun in SOW"""
    return {"id": "uuid"}

# Transaction Router
transaction_router = APIRouter(prefix="/nouns/{sow_id}/transactions", tags=["transactions"])

@transaction_router.get("/")
async def list_transactions(sow_id: str, db: AsyncSession = Depends(get_db)):
    """List transactions"""
    return {"transactions": []}

# View Routers
view_router = APIRouter(prefix="/views/{sow_id}", tags=["views"])

@view_router.get("/timeline")
async def timeline_view(sow_id: str, db: AsyncSession = Depends(get_db)):
    """Timeline view"""
    return {"view": "timeline"}

@view_router.get("/kanban")
async def kanban_view(sow_id: str, db: AsyncSession = Depends(get_db)):
    """Kanban view"""
    return {"view": "kanban"}

@view_router.get("/calendar")
async def calendar_view(sow_id: str, db: AsyncSession = Depends(get_db)):
    """Calendar view"""
    return {"view": "calendar"}
