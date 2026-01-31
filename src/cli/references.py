"""CLI Noun reference resolution"""

import re
from uuid import UUID
from typing import Tuple, Optional

class ReferenceChain:
    """Reference resolution chain: UUID -> ShortName -> Alias -> Title"""

    @staticmethod
    def resolve_uuid(reference: str) -> Optional[str]:
        """Try UUID match"""
        try:
            UUID(reference)
            return reference
        except:
            return None

    @staticmethod
    def resolve_short_name(reference: str) -> Optional[str]:
        """Try short_name match (IT-TASK-001)"""
        if re.match(r'^[A-Z]+-[A-Z]+-\d{3}$', reference):
            return reference
        return None

    @staticmethod
    def resolve_alias(reference: str, aliases: dict) -> Optional[str]:
        """Try alias match"""
        if reference.startswith('*'):
            alias_name = reference[1:]
            return aliases.get(alias_name)
        return None

    @staticmethod
    def resolve_title(reference: str, sow_id: Optional[str] = None) -> Optional[str]:
        """Try title match"""
        if reference.startswith('"') and reference.endswith('"'):
            # Search for noun with this title
            return reference[1:-1]
        return reference

    @staticmethod
    def resolve(reference: str, aliases: dict = None, sow_id: Optional[str] = None) -> Tuple[str, str]:
        """Full resolution chain"""
        # UUID
        if uuid_match := ReferenceChain.resolve_uuid(reference):
            return "uuid", uuid_match

        # ShortName
        if short_name := ReferenceChain.resolve_short_name(reference):
            return "short_name", short_name

        # Alias
        if aliases and (alias_match := ReferenceChain.resolve_alias(reference, aliases)):
            return "alias", alias_match

        # Title
        if title := ReferenceChain.resolve_title(reference, sow_id):
            return "title", title

        return None, None
