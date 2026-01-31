"""Domain services: State Machine, Blocking, Transactions"""

from enum import Enum
from typing import Dict, Any, List
from uuid import UUID

class StateTransition:
    """State machine implementation"""

    VALID_TRANSITIONS = {
        "Normal": ["Escalated", "Completed", "Incompleted"],
        "Escalated": ["Normal", "Completed", "Incompleted"],
        "Completed": ["Closed"],
        "Incompleted": ["Closed"],
        "Closed": ["Archived"],
        "Archived": [],
    }

    @staticmethod
    def is_valid_transition(from_state: str, to_state: str) -> bool:
        """Check if state transition is valid"""
        return to_state in StateTransition.VALID_TRANSITIONS.get(from_state, [])

class BlockingService:
    """Blocking propagation service"""

    @staticmethod
    def propagate_block_downward(blocker_id: UUID, target_id: UUID) -> List[UUID]:
        """Propagate block to target and all children"""
        affected = [target_id]
        # TODO: Recursively add all children
        return affected

    @staticmethod
    def compute_is_blocked(noun_id: UUID) -> bool:
        """Compute is_blocked status for noun"""
        # TODO: Check NounBlock relationships
        return False

class TransactionService:
    """Event sourcing service"""

    @staticmethod
    def create_transaction(
        noun_id: UUID,
        verb: str,
        actor: str,
        before: Dict[str, Any],
        after: Dict[str, Any],
        context: Dict[str, Any] = None
    ) -> Dict[str, Any]:
        """Create event transaction"""
        return {
            "noun_id": str(noun_id),
            "verb": verb,
            "actor": actor,
            "before_snapshot": before,
            "after_snapshot": after,
            "context": context or {}
        }

class ShortNameService:
    """Short name generation service"""

    TYPE_ABBREVS = {
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

    @staticmethod
    def generate_short_name(sow_short_name: str, noun_type: str, sequence: int) -> str:
        """Generate short name: {SOW.short_name}-{TYPE_ABBREV}-{SEQUENCE}"""
        abbrev = ShortNameService.TYPE_ABBREVS.get(noun_type, "UNKNOWN")
        return f"{sow_short_name}-{abbrev}-{sequence:03d}"
