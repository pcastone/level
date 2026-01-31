"""Unit tests for Noun and related models"""

import pytest
from uuid import uuid4
from datetime import datetime

from src.models.noun import Noun, NounType, NounState


class TestNounType:
    """Tests for NounType enum"""

    def test_all_12_noun_types_exist(self):
        """Verify all 12 noun types are defined"""
        expected_types = [
            "SOW", "Item", "Task", "Request", "Meeting",
            "Deliverable", "Event", "Blocker", "Artifact",
            "Group", "Project", "MileStone"
        ]
        actual_types = [t.value for t in NounType]
        assert len(actual_types) == 12
        for expected in expected_types:
            assert expected in actual_types

    def test_noun_type_is_string_enum(self):
        """NounType should be a string enum"""
        assert isinstance(NounType.TASK.value, str)
        assert NounType.TASK == "Task"

    def test_container_types(self):
        """Verify container types (Group, Project, MileStone)"""
        container_types = [NounType.GROUP, NounType.PROJECT, NounType.MILESTONE]
        assert NounType.GROUP.value == "Group"
        assert NounType.PROJECT.value == "Project"
        assert NounType.MILESTONE.value == "MileStone"

    def test_leaf_only_artifact(self):
        """Artifact should be a valid type (leaf-only enforcement is service-level)"""
        assert NounType.ARTIFACT.value == "Artifact"


class TestNounState:
    """Tests for NounState enum"""

    def test_all_6_states_exist(self):
        """Verify all 6 noun states are defined"""
        expected_states = [
            "Normal", "Escalated", "Completed",
            "Incompleted", "Closed", "Archived"
        ]
        actual_states = [s.value for s in NounState]
        assert len(actual_states) == 6
        for expected in expected_states:
            assert expected in actual_states

    def test_initial_state_is_normal(self):
        """Normal should be the initial state"""
        assert NounState.NORMAL.value == "Normal"

    def test_terminal_state_is_archived(self):
        """Archived should be the terminal state"""
        assert NounState.ARCHIVED.value == "Archived"

    def test_completion_states(self):
        """Verify completion states"""
        assert NounState.COMPLETED.value == "Completed"
        assert NounState.INCOMPLETED.value == "Incompleted"


class TestNounModel:
    """Tests for Noun SQLAlchemy model"""

    def test_noun_tablename(self):
        """Noun table should be named 'nouns'"""
        assert Noun.__tablename__ == "nouns"

    def test_noun_has_required_columns(self):
        """Verify Noun has all required columns"""
        required_columns = [
            "id", "type", "sow_id", "parent_id", "short_name",
            "title", "description", "state", "is_blocked",
            "due_date", "completed_at", "closed_at", "custom_fields"
        ]
        column_names = [c.name for c in Noun.__table__.columns]
        for col in required_columns:
            assert col in column_names, f"Missing column: {col}"

    def test_noun_repr(self, noun_factory):
        """Test Noun string representation"""
        data = noun_factory.create_task(sow_id=uuid4(), short_name="IT-TASK-001")
        # Create instance (not persisted)
        noun = Noun(**{k: v for k, v in data.items() if k != "id"})
        noun.id = data["id"]
        assert "IT-TASK-001" in repr(noun)
        assert "Task" in repr(noun)

    def test_noun_custom_fields_default_empty(self, noun_factory):
        """custom_fields should default to empty dict"""
        data = noun_factory.create_sow()
        assert data["custom_fields"] == {}

    def test_noun_is_blocked_default_false(self, noun_factory):
        """is_blocked should default to False"""
        data = noun_factory.create_task(sow_id=uuid4())
        assert data["is_blocked"] is False

    def test_noun_state_default_normal(self, noun_factory):
        """state should default to Normal"""
        data = noun_factory.create_task(sow_id=uuid4())
        assert data["state"] == NounState.NORMAL


class TestNounTypeAbbreviations:
    """Tests for noun type abbreviations used in short_name generation"""

    def test_type_abbreviations(self):
        """Verify type abbreviations mapping"""
        from src.services import ShortNameService

        expected_abbrevs = {
            "SOW": "SOW",
            "Item": "ITEM",
            "Task": "TASK",
            "Request": "REQ",
            "Meeting": "MEET",
            "Deliverable": "DELIV",
            "Event": "EVT",
            "Blocker": "BLOCK",
            "Artifact": "ART",
            "Group": "GRP",
            "Project": "PROJ",
            "MileStone": "MILE",
        }
        for noun_type, abbrev in expected_abbrevs.items():
            assert ShortNameService.TYPE_ABBREVS.get(noun_type) == abbrev


class TestNounCustomFields:
    """Tests for Noun JSONB custom_fields"""

    def test_project_has_goal_noun_ids(self, noun_factory):
        """Project should have goal_noun_ids in custom_fields"""
        data = noun_factory.create_project(sow_id=uuid4())
        assert "goal_noun_ids" in data["custom_fields"]
        assert isinstance(data["custom_fields"]["goal_noun_ids"], list)

    def test_milestone_has_auto_complete(self, noun_factory):
        """MileStone should have auto_complete in custom_fields"""
        data = noun_factory.create_milestone(sow_id=uuid4())
        assert "auto_complete" in data["custom_fields"]
        assert data["custom_fields"]["auto_complete"] is False

    def test_custom_fields_can_store_arbitrary_data(self, noun_factory):
        """custom_fields should accept arbitrary JSON data"""
        data = noun_factory.create_task(
            sow_id=uuid4(),
            custom_fields={
                "priority": "high",
                "tags": ["urgent", "customer"],
                "metadata": {"source": "api", "version": 1}
            }
        )
        assert data["custom_fields"]["priority"] == "high"
        assert "urgent" in data["custom_fields"]["tags"]
        assert data["custom_fields"]["metadata"]["version"] == 1
