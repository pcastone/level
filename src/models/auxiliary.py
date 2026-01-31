"""Auxiliary models for sequences, aliases, and federation"""

from sqlalchemy import Column, String, Integer, ForeignKey, Enum as SQLEnum
from sqlalchemy.dialects.postgresql import UUID
from .base import Base, IDMixin, TimestampMixin

class NounSequence(Base, IDMixin):
    """Per-SOW+type sequence for short_name generation"""
    __tablename__ = "noun_sequences"

    sow_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, unique=True)
    noun_type = Column(String(50), nullable=False, unique=True)
    next_sequence = Column(Integer, default=1, nullable=False)

class Alias(Base, IDMixin, TimestampMixin):
    """User-defined aliases for quick reference"""
    __tablename__ = "aliases"

    actor = Column(String(255), nullable=False, index=True)
    name = Column(String(255), nullable=False)
    noun_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False)
    __table_args__ = (("actor", "name"),)  # Unique constraint

class SOWDatabase(Base, IDMixin):
    """Multi-DB federation mapping"""
    __tablename__ = "sow_databases"

    sow_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, unique=True)
    database_url = Column(String(500), nullable=False)
    region = Column(String(100), nullable=True)

class Instruction(Base, IDMixin, TimestampMixin):
    """Operational instructions (global or per-SOW)"""
    __tablename__ = "instructions"

    scope = Column(String(50), nullable=False)  # global or sow
    sow_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=True)
    category = Column(String(100), nullable=False)
    title = Column(String(255), nullable=False)
    content = Column(String(4000), nullable=False)
    applies_to = Column(String(255), nullable=True)
    created_by = Column(String(255), nullable=False)

class UserPreferences(Base, IDMixin, TimestampMixin):
    """User preferences and settings"""
    __tablename__ = "user_preferences"

    actor = Column(String(255), nullable=False, unique=True, index=True)
    default_sow = Column(UUID(as_uuid=True), nullable=True)
    timezone = Column(String(100), default="UTC", nullable=False)
    theme = Column(String(50), default="light", nullable=False)
    date_format = Column(String(50), default="ISO", nullable=False)
    default_view = Column(String(50), default="timeline", nullable=False)
    notifications = Column(String(255), nullable=True)  # JSON stringified
