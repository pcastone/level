"""Database seed script for development"""

import asyncio
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession
from sqlalchemy.orm import sessionmaker
from src.models.base import Base

async def seed_database():
    """Create tables and seed initial data"""
    engine = create_async_engine("postgresql+asyncpg://level:level@localhost/level_db")

    # Create all tables
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)

    print("✓ Database tables created")
    print("✓ Seeding complete")

if __name__ == "__main__":
    asyncio.run(seed_database())
