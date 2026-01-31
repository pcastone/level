"""CLI utilities and helpers"""

from enum import Enum
from typing import List
import re

class ReferenceType(Enum):
    UUID = "uuid"
    SHORT_NAME = "short_name"
    ALIAS = "alias"
    TITLE = "title"

class ReferenceResolver:
    """Resolve noun references in multiple formats"""

    @staticmethod
    def resolve(reference: str) -> tuple:
        """Resolve reference to (type, value)"""
        # UUID format
        if re.match(r'^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$', reference):
            return ReferenceType.UUID, reference

        # ShortName format (e.g., IT-TASK-001)
        if re.match(r'^[A-Z]+-[A-Z]+-\d{3}$', reference):
            return ReferenceType.SHORT_NAME, reference

        # Alias format (starts with *)
        if reference.startswith('*'):
            return ReferenceType.ALIAS, reference[1:]

        # Title format (quoted)
        if reference.startswith('"') and reference.endswith('"'):
            return ReferenceType.TITLE, reference[1:-1]

        return ReferenceType.TITLE, reference

class OutputFormatter:
    """Format output in different styles"""

    @staticmethod
    def table(data: List[dict], columns: List[str]) -> str:
        """Format as ASCII table"""
        # Simple table formatter
        lines = []
        header = " | ".join(columns)
        lines.append(header)
        lines.append("-" * len(header))
        for row in data:
            values = [str(row.get(col, '')) for col in columns]
            lines.append(" | ".join(values))
        return "\n".join(lines)

    @staticmethod
    def json(data: dict) -> str:
        """Format as JSON"""
        import json
        return json.dumps(data, indent=2)

    @staticmethod
    def minimal(data: dict) -> str:
        """Format minimal output"""
        if isinstance(data, dict):
            return " | ".join(f"{k}={v}" for k, v in data.items())
        return str(data)
