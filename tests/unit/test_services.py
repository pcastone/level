"""Unit tests for domain services"""

import pytest
from uuid import uuid4
from datetime import datetime

from src.services import (
    StateTransition,
    BlockingService,
    TransactionService,
    ShortNameService
)


class TestStateTransition:
    """Tests for state machine transitions"""

    def test_valid_transitions_from_normal(self):
        """Normal can transition to Escalated, Completed, Incompleted"""
        assert StateTransition.is_valid_transition("Normal", "Escalated") is True
        assert StateTransition.is_valid_transition("Normal", "Completed") is True
        assert StateTransition.is_valid_transition("Normal", "Incompleted") is True

    def test_invalid_transitions_from_normal(self):
        """Normal cannot directly go to Closed or Archived"""
        assert StateTransition.is_valid_transition("Normal", "Closed") is False
        assert StateTransition.is_valid_transition("Normal", "Archived") is False

    def test_valid_transitions_from_escalated(self):
        """Escalated can transition to Normal, Completed, Incompleted"""
        assert StateTransition.is_valid_transition("Escalated", "Normal") is True
        assert StateTransition.is_valid_transition("Escalated", "Completed") is True
        assert StateTransition.is_valid_transition("Escalated", "Incompleted") is True

    def test_valid_transitions_from_completed(self):
        """Completed can only transition to Closed"""
        assert StateTransition.is_valid_transition("Completed", "Closed") is True
        assert StateTransition.is_valid_transition("Completed", "Normal") is False
        assert StateTransition.is_valid_transition("Completed", "Archived") is False

    def test_valid_transitions_from_incompleted(self):
        """Incompleted can only transition to Closed"""
        assert StateTransition.is_valid_transition("Incompleted", "Closed") is True
        assert StateTransition.is_valid_transition("Incompleted", "Normal") is False

    def test_valid_transitions_from_closed(self):
        """Closed can only transition to Archived"""
        assert StateTransition.is_valid_transition("Closed", "Archived") is True
        assert StateTransition.is_valid_transition("Closed", "Normal") is False

    def test_archived_is_terminal(self):
        """Archived cannot transition to any state"""
        assert StateTransition.is_valid_transition("Archived", "Normal") is False
        assert StateTransition.is_valid_transition("Archived", "Closed") is False
        assert StateTransition.is_valid_transition("Archived", "Archived") is False

    def test_invalid_from_state(self):
        """Unknown from_state returns False"""
        assert StateTransition.is_valid_transition("Unknown", "Normal") is False

    def test_same_state_transition(self):
        """Transitioning to same state is not in valid transitions"""
        assert StateTransition.is_valid_transition("Normal", "Normal") is False


class TestBlockingService:
    """Tests for blocking propagation service"""

    def test_propagate_block_returns_target(self):
        """propagate_block_downward should return at least the target"""
        blocker_id = uuid4()
        target_id = uuid4()
        affected = BlockingService.propagate_block_downward(blocker_id, target_id)
        assert target_id in affected

    def test_compute_is_blocked_default_false(self):
        """compute_is_blocked should return False by default"""
        noun_id = uuid4()
        result = BlockingService.compute_is_blocked(noun_id)
        assert result is False


class TestTransactionService:
    """Tests for event sourcing transactions"""

    def test_create_transaction_basic(self):
        """create_transaction should return proper structure"""
        noun_id = uuid4()
        transaction = TransactionService.create_transaction(
            noun_id=noun_id,
            verb="Complete",
            actor="+john.doe",
            before={"state": "Normal"},
            after={"state": "Completed"}
        )
        assert transaction["noun_id"] == str(noun_id)
        assert transaction["verb"] == "Complete"
        assert transaction["actor"] == "+john.doe"
        assert transaction["before_snapshot"]["state"] == "Normal"
        assert transaction["after_snapshot"]["state"] == "Completed"

    def test_create_transaction_with_context(self):
        """create_transaction should include context when provided"""
        transaction = TransactionService.create_transaction(
            noun_id=uuid4(),
            verb="Blocked",
            actor="+admin",
            before={},
            after={"is_blocked": True},
            context={"target_id": str(uuid4()), "reason": "Waiting for approval"}
        )
        assert "target_id" in transaction["context"]
        assert transaction["context"]["reason"] == "Waiting for approval"

    def test_create_transaction_empty_context(self):
        """create_transaction should default to empty context"""
        transaction = TransactionService.create_transaction(
            noun_id=uuid4(),
            verb="Update",
            actor="*claude",
            before={},
            after={}
        )
        assert transaction["context"] == {}


class TestShortNameService:
    """Tests for short name generation"""

    def test_generate_short_name_format(self):
        """Short name format: {SOW}-{TYPE_ABBREV}-{SEQ:03d}"""
        result = ShortNameService.generate_short_name("IT", "Task", 1)
        assert result == "IT-TASK-001"

    def test_generate_short_name_padding(self):
        """Sequence should be zero-padded to 3 digits"""
        assert ShortNameService.generate_short_name("HR", "Request", 5) == "HR-REQ-005"
        assert ShortNameService.generate_short_name("HR", "Request", 42) == "HR-REQ-042"
        assert ShortNameService.generate_short_name("HR", "Request", 999) == "HR-REQ-999"

    def test_generate_short_name_all_types(self):
        """Test short name generation for all noun types"""
        test_cases = [
            ("SOW", "SOW", "TEST-SOW-001"),
            ("Item", "ITEM", "TEST-ITEM-001"),
            ("Task", "TASK", "TEST-TASK-001"),
            ("Request", "REQ", "TEST-REQ-001"),
            ("Meeting", "MEET", "TEST-MEET-001"),
            ("Deliverable", "DELIV", "TEST-DELIV-001"),
            ("Event", "EVT", "TEST-EVT-001"),
            ("Blocker", "BLOCK", "TEST-BLOCK-001"),
            ("Artifact", "ART", "TEST-ART-001"),
            ("Group", "GRP", "TEST-GRP-001"),
            ("Project", "PROJ", "TEST-PROJ-001"),
            ("MileStone", "MILE", "TEST-MILE-001"),
        ]
        for noun_type, abbrev, expected in test_cases:
            result = ShortNameService.generate_short_name("TEST", noun_type, 1)
            assert result == expected, f"Failed for {noun_type}"

    def test_generate_short_name_unknown_type(self):
        """Unknown type should use UNKNOWN abbreviation"""
        result = ShortNameService.generate_short_name("TEST", "Unknown", 1)
        assert result == "TEST-UNKNOWN-001"

    def test_type_abbrevs_completeness(self):
        """TYPE_ABBREVS should have all 12 noun types"""
        from src.models.noun import NounType
        for noun_type in NounType:
            assert noun_type.value in ShortNameService.TYPE_ABBREVS, f"Missing: {noun_type.value}"


class TestStateTransitionGraph:
    """Integration tests for full state machine flow"""

    def test_full_happy_path_flow(self):
        """Test Normal -> Escalated -> Completed -> Closed -> Archived"""
        # Normal to Escalated
        assert StateTransition.is_valid_transition("Normal", "Escalated")
        # Escalated to Completed
        assert StateTransition.is_valid_transition("Escalated", "Completed")
        # Completed to Closed
        assert StateTransition.is_valid_transition("Completed", "Closed")
        # Closed to Archived
        assert StateTransition.is_valid_transition("Closed", "Archived")

    def test_direct_completion_flow(self):
        """Test Normal -> Completed -> Closed -> Archived"""
        assert StateTransition.is_valid_transition("Normal", "Completed")
        assert StateTransition.is_valid_transition("Completed", "Closed")
        assert StateTransition.is_valid_transition("Closed", "Archived")

    def test_incompleted_flow(self):
        """Test Normal -> Incompleted -> Closed -> Archived"""
        assert StateTransition.is_valid_transition("Normal", "Incompleted")
        assert StateTransition.is_valid_transition("Incompleted", "Closed")
        assert StateTransition.is_valid_transition("Closed", "Archived")

    def test_de_escalation(self):
        """Test Escalated -> Normal (de-escalation)"""
        assert StateTransition.is_valid_transition("Escalated", "Normal")
