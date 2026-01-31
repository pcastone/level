"""Tests for CLI reference resolution"""

import pytest
from uuid import uuid4

from src.cli.references import ReferenceChain


class TestUUIDResolution:
    """Tests for UUID reference resolution"""

    def test_resolve_valid_uuid(self):
        """Valid UUID should be resolved"""
        test_uuid = str(uuid4())
        result = ReferenceChain.resolve_uuid(test_uuid)
        assert result == test_uuid

    def test_resolve_invalid_uuid(self):
        """Invalid UUID should return None"""
        result = ReferenceChain.resolve_uuid("not-a-uuid")
        assert result is None

    def test_resolve_uuid_with_hyphens(self):
        """UUID with hyphens should be resolved"""
        test_uuid = "12345678-1234-5678-1234-567812345678"
        result = ReferenceChain.resolve_uuid(test_uuid)
        assert result == test_uuid


class TestShortNameResolution:
    """Tests for short_name reference resolution"""

    def test_resolve_valid_short_name(self):
        """Valid short_name format should be resolved"""
        result = ReferenceChain.resolve_short_name("IT-TASK-001")
        assert result == "IT-TASK-001"

    def test_resolve_short_name_various_types(self):
        """All noun type abbreviations should be resolved"""
        valid_short_names = [
            "IT-SOW-001",
            "HR-TASK-042",
            "PROJ-REQ-999",
            "TEST-MEET-001",
            "X-BLOCK-123",
        ]
        for short_name in valid_short_names:
            result = ReferenceChain.resolve_short_name(short_name)
            assert result == short_name

    def test_reject_invalid_short_name_lowercase(self):
        """Lowercase short_name should not match"""
        result = ReferenceChain.resolve_short_name("it-task-001")
        assert result is None

    def test_reject_invalid_short_name_format(self):
        """Invalid format should not match"""
        invalid_names = [
            "TASK-001",      # Missing SOW prefix
            "IT-TASK",       # Missing sequence
            "IT-TASK-1",     # Wrong sequence format (not 3 digits)
            "IT-task-001",   # Mixed case
        ]
        for name in invalid_names:
            result = ReferenceChain.resolve_short_name(name)
            assert result is None, f"Should reject: {name}"


class TestAliasResolution:
    """Tests for alias reference resolution"""

    def test_resolve_valid_alias(self, mock_aliases):
        """Alias with * prefix should be resolved"""
        result = ReferenceChain.resolve_alias("*budget", mock_aliases)
        assert result == mock_aliases["budget"]

    def test_resolve_alias_not_found(self, mock_aliases):
        """Unknown alias should return None"""
        result = ReferenceChain.resolve_alias("*unknown", mock_aliases)
        assert result is None

    def test_alias_without_prefix(self, mock_aliases):
        """Reference without * prefix should not match as alias"""
        result = ReferenceChain.resolve_alias("budget", mock_aliases)
        assert result is None

    def test_alias_empty_dict(self):
        """Empty aliases dict should return None"""
        result = ReferenceChain.resolve_alias("*test", {})
        assert result is None

    def test_alias_none_dict(self):
        """None aliases should return None"""
        result = ReferenceChain.resolve_alias("*test", None)
        assert result is None


class TestTitleResolution:
    """Tests for title reference resolution"""

    def test_resolve_quoted_title(self):
        """Quoted string should be resolved as title"""
        result = ReferenceChain.resolve_title('"My Task Title"')
        assert result == "My Task Title"

    def test_resolve_unquoted_string(self):
        """Unquoted string should be returned as-is"""
        result = ReferenceChain.resolve_title("Some Task")
        assert result == "Some Task"

    def test_title_with_sow_id(self):
        """Title resolution with sow_id context"""
        sow_id = str(uuid4())
        result = ReferenceChain.resolve_title('"Budget Planning"', sow_id)
        assert result == "Budget Planning"


class TestFullResolutionChain:
    """Tests for complete reference resolution chain"""

    def test_resolve_uuid_first(self, mock_aliases):
        """UUID should take priority"""
        test_uuid = str(uuid4())
        ref_type, ref_value = ReferenceChain.resolve(test_uuid, mock_aliases)
        assert ref_type == "uuid"
        assert ref_value == test_uuid

    def test_resolve_short_name_second(self, mock_aliases):
        """ShortName should be resolved if not UUID"""
        ref_type, ref_value = ReferenceChain.resolve("IT-TASK-001", mock_aliases)
        assert ref_type == "short_name"
        assert ref_value == "IT-TASK-001"

    def test_resolve_alias_third(self, mock_aliases):
        """Alias should be resolved if not UUID or ShortName"""
        ref_type, ref_value = ReferenceChain.resolve("*budget", mock_aliases)
        assert ref_type == "alias"
        assert ref_value == mock_aliases["budget"]

    def test_resolve_title_fourth(self, mock_aliases):
        """Title should be fallback"""
        ref_type, ref_value = ReferenceChain.resolve('"My Task"', mock_aliases)
        assert ref_type == "title"
        assert ref_value == "My Task"

    def test_resolve_plain_text_as_title(self, mock_aliases):
        """Plain text falls back to title resolution"""
        ref_type, ref_value = ReferenceChain.resolve("Some random text", mock_aliases)
        assert ref_type == "title"

    def test_resolution_priority_order(self, mock_aliases):
        """Test that resolution follows correct priority"""
        # UUID takes priority over everything
        uuid_ref = str(uuid4())
        ref_type, _ = ReferenceChain.resolve(uuid_ref, mock_aliases)
        assert ref_type == "uuid"

        # ShortName takes priority over alias/title
        ref_type, _ = ReferenceChain.resolve("IT-TASK-001", mock_aliases)
        assert ref_type == "short_name"


class TestEdgeCases:
    """Edge case tests for reference resolution"""

    def test_empty_reference(self):
        """Empty string resolution"""
        ref_type, ref_value = ReferenceChain.resolve("")
        # Empty string should fall through to title
        assert ref_type == "title"

    def test_whitespace_reference(self):
        """Whitespace-only reference"""
        ref_type, ref_value = ReferenceChain.resolve("   ")
        assert ref_type == "title"

    def test_resolve_with_no_aliases(self):
        """Resolution without aliases parameter"""
        ref_type, ref_value = ReferenceChain.resolve("*test")
        # Without aliases dict, alias resolution returns None
        assert ref_type == "title"

    def test_special_characters_in_title(self):
        """Title with special characters"""
        ref_type, ref_value = ReferenceChain.resolve('"Task: Fix Bug #123"')
        assert ref_type == "title"
        assert ref_value == "Task: Fix Bug #123"
