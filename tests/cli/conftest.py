"""CLI test fixtures"""

import pytest
from unittest.mock import MagicMock, AsyncMock
from uuid import uuid4


@pytest.fixture
def mock_config():
    """Mock CLI configuration"""
    return {
        "api_url": "http://localhost:8000",
        "api_key": "test-api-key",
        "default_sow": "TEST",
        "output_format": "table"
    }


@pytest.fixture
def mock_aliases():
    """Mock user aliases"""
    return {
        "budget": str(uuid4()),
        "sprint1": str(uuid4()),
        "urgent": str(uuid4()),
    }


@pytest.fixture
def mock_http_response():
    """Factory for mock HTTP responses"""
    def _create(data, status_code=200):
        response = MagicMock()
        response.status_code = status_code
        response.json.return_value = data
        return response
    return _create
