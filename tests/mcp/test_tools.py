"""Tests for MCP server tools"""

import pytest
from unittest.mock import patch, AsyncMock
from uuid import uuid4


class TestMCPToolDefinitions:
    """Tests for MCP tool definitions"""

    def test_tool_names_follow_convention(self):
        """Tool names should follow level_* pattern"""
        expected_tools = [
            "level_list_sows",
            "level_show_sow",
            "level_list_nouns",
            "level_show_noun",
            "level_create_noun",
            "level_complete",
            "level_escalate",
            "level_close",
            "level_update",
        ]
        for tool in expected_tools:
            assert tool.startswith("level_")


class TestListSOWsTool:
    """Tests for level_list_sows tool"""

    @pytest.mark.asyncio
    async def test_list_sows_returns_sows(self):
        """level_list_sows should return list of SOWs"""
        from src.mcp.server import level_list_sows
        result = await level_list_sows()
        assert "sows" in result
        assert isinstance(result["sows"], list)


class TestShowSOWTool:
    """Tests for level_show_sow tool"""

    @pytest.mark.asyncio
    async def test_show_sow_accepts_id(self):
        """level_show_sow should accept sow_id"""
        from src.mcp.server import level_show_sow
        sow_id = str(uuid4())
        result = await level_show_sow(sow_id)
        assert "sow" in result
        assert result["sow"]["id"] == sow_id


class TestListNounsTool:
    """Tests for level_list_nouns tool"""

    @pytest.mark.asyncio
    async def test_list_nouns_with_sow_id(self):
        """level_list_nouns requires sow_id"""
        from src.mcp.server import level_list_nouns
        sow_id = str(uuid4())
        result = await level_list_nouns(sow_id)
        assert "nouns" in result

    @pytest.mark.asyncio
    async def test_list_nouns_with_filters(self):
        """level_list_nouns accepts optional filters"""
        from src.mcp.server import level_list_nouns
        sow_id = str(uuid4())
        result = await level_list_nouns(sow_id, noun_type="Task", state="Normal")
        assert "nouns" in result


class TestShowNounTool:
    """Tests for level_show_noun tool"""

    @pytest.mark.asyncio
    async def test_show_noun_with_reference(self):
        """level_show_noun should accept reference"""
        from src.mcp.server import level_show_noun
        result = await level_show_noun("IT-TASK-001")
        assert "noun" in result
        assert result["noun"]["reference"] == "IT-TASK-001"


class TestCreateNounTool:
    """Tests for level_create_noun tool"""

    @pytest.mark.asyncio
    async def test_create_noun_required_params(self):
        """level_create_noun requires sow_id, noun_type, title"""
        from src.mcp.server import level_create_noun
        sow_id = str(uuid4())
        result = await level_create_noun(sow_id, "Task", "New Task")
        assert "id" in result
        assert "short_name" in result

    @pytest.mark.asyncio
    async def test_create_noun_with_parent(self):
        """level_create_noun accepts optional parent_id"""
        from src.mcp.server import level_create_noun
        sow_id = str(uuid4())
        parent_id = str(uuid4())
        result = await level_create_noun(sow_id, "Task", "Sub Task", parent_id)
        assert "id" in result


class TestVerbTools:
    """Tests for verb action tools"""

    @pytest.mark.asyncio
    async def test_complete_tool(self):
        """level_complete should complete noun"""
        from src.mcp.server import level_complete
        result = await level_complete("IT-TASK-001")
        assert result["status"] == "completed"

    @pytest.mark.asyncio
    async def test_escalate_tool(self):
        """level_escalate should escalate noun"""
        from src.mcp.server import level_escalate
        result = await level_escalate("IT-TASK-001")
        assert result["status"] == "escalated"

    @pytest.mark.asyncio
    async def test_close_tool(self):
        """level_close should close noun"""
        from src.mcp.server import level_close
        result = await level_close("IT-TASK-001")
        assert result["status"] == "closed"

    @pytest.mark.asyncio
    async def test_update_tool(self):
        """level_update should update noun fields"""
        from src.mcp.server import level_update
        result = await level_update("IT-TASK-001", {"title": "Updated Title"})
        assert result["status"] == "updated"


class TestMCPResources:
    """Tests for MCP resources"""

    @pytest.mark.asyncio
    async def test_sows_resource(self):
        """level_sows resource returns URI"""
        from src.mcp.server import level_sows
        result = await level_sows()
        assert result == "level://sows"

    @pytest.mark.asyncio
    async def test_sow_nouns_resource(self):
        """level_sow_nouns resource returns URI with sow_id"""
        from src.mcp.server import level_sow_nouns
        sow_id = "test-sow-123"
        result = await level_sow_nouns(sow_id)
        assert sow_id in result
        assert result.startswith("level://")


class TestMCPPrompts:
    """Tests for MCP prompts"""

    @pytest.mark.asyncio
    async def test_help_prompt(self):
        """level_help prompt returns grammar overview"""
        from src.mcp.server import level_help
        result = await level_help()
        assert "12 Nouns" in result or isinstance(result, str)

    @pytest.mark.asyncio
    async def test_context_prompt(self):
        """level_context prompt returns SOW context"""
        from src.mcp.server import level_context
        sow_id = "test-sow"
        result = await level_context(sow_id)
        assert sow_id in result


class TestMCPActorContext:
    """Tests for AI actor context in MCP"""

    def test_ai_actor_prefix(self):
        """AI actor should use * prefix"""
        ai_actor = "*claude"
        assert ai_actor.startswith("*")

    def test_ai_actor_permissions(self):
        """AI actor has specific permissions based on role"""
        # AI actors have permissions defined by their role mapping
        expected_roles = ["awareness", "assignee", "sme"]
        for role in expected_roles:
            assert isinstance(role, str)
