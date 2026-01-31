"""Shared test fixtures for Level test suite"""

import pytest
from typing import AsyncGenerator, Dict, Any
from uuid import uuid4
from datetime import datetime
from unittest.mock import AsyncMock, MagicMock

from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine, async_sessionmaker
from sqlalchemy.pool import StaticPool

from src.models.base import Base
from src.models.noun import Noun, NounType, NounState


# Use SQLite in-memory for tests (simpler than PostgreSQL)
TEST_DATABASE_URL = "sqlite+aiosqlite:///:memory:"


@pytest.fixture
async def async_engine():
    """Create async test engine"""
    engine = create_async_engine(
        TEST_DATABASE_URL,
        connect_args={"check_same_thread": False},
        poolclass=StaticPool,
        echo=False,
    )
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)
    yield engine
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.drop_all)
    await engine.dispose()


@pytest.fixture
async def db_session(async_engine) -> AsyncGenerator[AsyncSession, None]:
    """Create async database session with rollback"""
    async_session_maker = async_sessionmaker(
        async_engine,
        class_=AsyncSession,
        expire_on_commit=False,
    )
    async with async_session_maker() as session:
        yield session
        await session.rollback()


# Factory functions for test data
class NounFactory:
    """Factory for creating test Noun instances"""

    @staticmethod
    def create_sow(
        short_name: str = "TEST",
        title: str = "Test SOW",
        **kwargs
    ) -> Dict[str, Any]:
        """Create SOW test data"""
        sow_id = kwargs.get("id", uuid4())
        return {
            "id": sow_id,
            "type": NounType.SOW,
            "sow_id": sow_id,  # SOW references itself
            "short_name": short_name,
            "title": title,
            "state": NounState.NORMAL,
            "is_blocked": False,
            "custom_fields": {},
            **kwargs
        }

    @staticmethod
    def create_task(
        sow_id,
        short_name: str = "TEST-TASK-001",
        title: str = "Test Task",
        **kwargs
    ) -> Dict[str, Any]:
        """Create Task test data"""
        return {
            "id": uuid4(),
            "type": NounType.TASK,
            "sow_id": sow_id,
            "short_name": short_name,
            "title": title,
            "state": NounState.NORMAL,
            "is_blocked": False,
            "custom_fields": {},
            **kwargs
        }

    @staticmethod
    def create_blocker(
        sow_id,
        short_name: str = "TEST-BLOCK-001",
        title: str = "Test Blocker",
        **kwargs
    ) -> Dict[str, Any]:
        """Create Blocker test data"""
        return {
            "id": uuid4(),
            "type": NounType.BLOCKER,
            "sow_id": sow_id,
            "short_name": short_name,
            "title": title,
            "state": NounState.NORMAL,
            "is_blocked": False,
            "custom_fields": {},
            **kwargs
        }

    @staticmethod
    def create_project(
        sow_id,
        short_name: str = "TEST-PROJ-001",
        title: str = "Test Project",
        **kwargs
    ) -> Dict[str, Any]:
        """Create Project test data"""
        return {
            "id": uuid4(),
            "type": NounType.PROJECT,
            "sow_id": sow_id,
            "short_name": short_name,
            "title": title,
            "state": NounState.NORMAL,
            "is_blocked": False,
            "custom_fields": {"goal_noun_ids": []},
            **kwargs
        }

    @staticmethod
    def create_milestone(
        sow_id,
        short_name: str = "TEST-MILE-001",
        title: str = "Test MileStone",
        **kwargs
    ) -> Dict[str, Any]:
        """Create MileStone test data"""
        return {
            "id": uuid4(),
            "type": NounType.MILESTONE,
            "sow_id": sow_id,
            "short_name": short_name,
            "title": title,
            "state": NounState.NORMAL,
            "is_blocked": False,
            "custom_fields": {"auto_complete": False},
            **kwargs
        }


@pytest.fixture
def noun_factory():
    """Provide NounFactory to tests"""
    return NounFactory


@pytest.fixture
def sample_sow():
    """Create sample SOW data"""
    return NounFactory.create_sow()


@pytest.fixture
def sample_task(sample_sow):
    """Create sample Task data"""
    return NounFactory.create_task(sow_id=sample_sow["id"])


# Mock fixtures
@pytest.fixture
def mock_db_session():
    """Create mock database session"""
    session = AsyncMock(spec=AsyncSession)
    session.commit = AsyncMock()
    session.rollback = AsyncMock()
    session.add = MagicMock()
    session.delete = MagicMock()
    session.execute = AsyncMock()
    return session


@pytest.fixture
def mock_http_client():
    """Create mock HTTP client for CLI tests"""
    client = AsyncMock()
    client.get = AsyncMock(return_value={"status": "ok"})
    client.post = AsyncMock(return_value={"id": str(uuid4())})
    client.put = AsyncMock(return_value={"status": "updated"})
    client.delete = AsyncMock(return_value={"status": "deleted"})
    return client
