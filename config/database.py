"""Database configuration and connection setup"""

from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession
from sqlalchemy.orm import sessionmaker
from config.settings import settings

# Create async engine for PostgreSQL with asyncpg driver
engine = create_async_engine(
    settings.database_url,
    echo=settings.api_debug,
    future=True,
    pool_size=20,
    max_overflow=0,
)

# Create async session factory
async_session = sessionmaker(
    engine,
    class_=AsyncSession,
    expire_on_commit=False,
    future=True,
)

async def get_db_session():
    """Dependency for getting database session"""
    async with async_session() as session:
        yield session
