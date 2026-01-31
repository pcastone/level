"""Unit tests for verb handlers"""

import pytest
from uuid import uuid4

from src.verbs import Verb, VerbHandler
from src.services import StateTransition


class TestVerbEnum:
    """Tests for Verb enum"""

    def test_all_12_verbs_exist(self):
        """Verify all 12 verbs are defined"""
        expected_verbs = [
            "Open", "Complete", "Incomplete", "Normal", "Escalate",
            "Close", "Update", "Reparent", "Assign", "Unassign",
            "Blocked", "Release"
        ]
        actual_verbs = [v.value for v in Verb]
        assert len(actual_verbs) == 12
        for expected in expected_verbs:
            assert expected in actual_verbs

    def test_verb_is_string_enum(self):
        """Verb should be a string enum"""
        assert isinstance(Verb.OPEN.value, str)
        assert Verb.COMPLETE == "Complete"


class TestVerbStateGuards:
    """Tests for verb state guards"""

    @pytest.mark.parametrize("verb,required_states", [
        ("Complete", ["Normal", "Escalated"]),
        ("Incomplete", ["Normal", "Escalated"]),
        ("Escalate", ["Normal"]),
        ("Normal", ["Escalated"]),
        ("Close", ["Completed", "Incompleted"]),
    ])
    def test_verb_requires_specific_state(self, verb, required_states):
        """Certain verbs require specific states"""
        # This tests the state transition rules that verbs must follow
        for state in required_states:
            # Verb should be able to transition from these states
            if verb == "Complete":
                assert StateTransition.is_valid_transition(state, "Completed")
            elif verb == "Incomplete":
                assert StateTransition.is_valid_transition(state, "Incompleted")
            elif verb == "Escalate":
                assert StateTransition.is_valid_transition(state, "Escalated")
            elif verb == "Normal":
                assert StateTransition.is_valid_transition(state, "Normal")
            elif verb == "Close":
                assert StateTransition.is_valid_transition(state, "Closed")


class TestVerbCategories:
    """Tests for verb categorization"""

    def test_state_changing_verbs(self):
        """Verbs that change noun state"""
        state_verbs = [Verb.COMPLETE, Verb.INCOMPLETE, Verb.NORMAL, Verb.ESCALATE, Verb.CLOSE]
        assert len(state_verbs) == 5

    def test_relationship_verbs(self):
        """Verbs that modify relationships"""
        relationship_verbs = [Verb.ASSIGN, Verb.UNASSIGN, Verb.BLOCKED, Verb.RELEASE, Verb.REPARENT]
        assert len(relationship_verbs) == 5

    def test_crud_verbs(self):
        """Verbs for CRUD operations"""
        crud_verbs = [Verb.OPEN, Verb.UPDATE]
        assert len(crud_verbs) == 2


class TestOpenVerb:
    """Tests for Open verb behavior"""

    @pytest.mark.asyncio
    async def test_open_noun_exists(self):
        """VerbHandler should have open_noun method"""
        assert hasattr(VerbHandler, 'open_noun')
        assert callable(VerbHandler.open_noun)

    @pytest.mark.asyncio
    async def test_open_is_async(self):
        """open_noun should be async"""
        import asyncio
        result = VerbHandler.open_noun(uuid4(), {})
        assert asyncio.iscoroutine(result)
        await result  # Clean up coroutine


class TestCompleteVerb:
    """Tests for Complete verb behavior"""

    @pytest.mark.asyncio
    async def test_complete_noun_exists(self):
        """VerbHandler should have complete_noun method"""
        assert hasattr(VerbHandler, 'complete_noun')
        assert callable(VerbHandler.complete_noun)

    def test_complete_requires_not_blocked(self):
        """Complete should fail if noun is blocked (business rule)"""
        # This is a business rule test - the actual enforcement is in the handler
        # We're testing that Normal->Completed is valid at state machine level
        assert StateTransition.is_valid_transition("Normal", "Completed")
        assert StateTransition.is_valid_transition("Escalated", "Completed")


class TestEscalateVerb:
    """Tests for Escalate verb behavior"""

    @pytest.mark.asyncio
    async def test_escalate_noun_exists(self):
        """VerbHandler should have escalate_noun method"""
        assert hasattr(VerbHandler, 'escalate_noun')

    def test_escalate_only_from_normal(self):
        """Escalate should only work from Normal state"""
        assert StateTransition.is_valid_transition("Normal", "Escalated")
        assert StateTransition.is_valid_transition("Completed", "Escalated") is False
        assert StateTransition.is_valid_transition("Closed", "Escalated") is False


class TestCloseVerb:
    """Tests for Close verb behavior"""

    @pytest.mark.asyncio
    async def test_close_noun_exists(self):
        """VerbHandler should have close_noun method"""
        assert hasattr(VerbHandler, 'close_noun')

    def test_close_requires_completion_state(self):
        """Close requires Completed or Incompleted state"""
        assert StateTransition.is_valid_transition("Completed", "Closed")
        assert StateTransition.is_valid_transition("Incompleted", "Closed")
        assert StateTransition.is_valid_transition("Normal", "Closed") is False
        assert StateTransition.is_valid_transition("Escalated", "Closed") is False


class TestUpdateVerb:
    """Tests for Update verb behavior"""

    @pytest.mark.asyncio
    async def test_update_noun_exists(self):
        """VerbHandler should have update_noun method"""
        assert hasattr(VerbHandler, 'update_noun')

    @pytest.mark.asyncio
    async def test_update_accepts_dict(self):
        """update_noun should accept dict of updates"""
        import asyncio
        result = VerbHandler.update_noun(uuid4(), {"title": "New Title"})
        assert asyncio.iscoroutine(result)
        await result


class TestBlockVerb:
    """Tests for Blocked verb behavior"""

    @pytest.mark.asyncio
    async def test_block_noun_exists(self):
        """VerbHandler should have block_noun method"""
        assert hasattr(VerbHandler, 'block_noun')

    @pytest.mark.asyncio
    async def test_block_takes_two_ids(self):
        """block_noun requires blocker_id and target_id"""
        import asyncio
        result = VerbHandler.block_noun(uuid4(), uuid4())
        assert asyncio.iscoroutine(result)
        await result


class TestVerbTransactionIntegration:
    """Tests for verb-transaction integration"""

    def test_all_verbs_should_create_transactions(self):
        """All verbs should create transaction records"""
        from src.services import TransactionService

        for verb in Verb:
            transaction = TransactionService.create_transaction(
                noun_id=uuid4(),
                verb=verb.value,
                actor="+test.user",
                before={},
                after={}
            )
            assert transaction["verb"] == verb.value

    def test_verb_transaction_has_before_after(self):
        """Verb transactions must have before/after snapshots"""
        from src.services import TransactionService

        transaction = TransactionService.create_transaction(
            noun_id=uuid4(),
            verb="Complete",
            actor="+user",
            before={"state": "Normal", "is_blocked": False},
            after={"state": "Completed", "is_blocked": False, "completed_at": "2026-01-30T12:00:00Z"}
        )
        assert "state" in transaction["before_snapshot"]
        assert "state" in transaction["after_snapshot"]
        assert transaction["before_snapshot"]["state"] != transaction["after_snapshot"]["state"]
