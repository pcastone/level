"""MCP Server test fixtures"""

import pytest
from unittest.mock import MagicMock, AsyncMock
from uuid import uuid4


@pytest.fixture
def mock_mcp_server():
    """Mock MCP server instance"""
    server = MagicMock()
    server.tool = MagicMock(return_value=lambda fn: fn)
    server.resource = MagicMock(return_value=lambda fn: fn)
    server.prompt = MagicMock(return_value=lambda fn: fn)
    return server


@pytest.fixture
def sample_sow_data():
    """Sample SOW data for tests"""
    return {
        "id": str(uuid4()),
        "type": "SOW",
        "short_name": "TEST",
        "title": "Test Project",
        "state": "Normal"
    }


@pytest.fixture
def sample_noun_data():
    """Sample noun data for tests"""
    sow_id = str(uuid4())
    return {
        "id": str(uuid4()),
        "sow_id": sow_id,
        "type": "Task",
        "short_name": "TEST-TASK-001",
        "title": "Test Task",
        "state": "Normal",
        "is_blocked": False
    }
