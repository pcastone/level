"""Noun entities - core domain model"""

from enum import Enum
from typing import Optional, List
from uuid import uuid4
from sqlalchemy import Column, String, Enum as SQLEnum, ForeignKey, JSON, Integer, Boolean
from sqlalchemy.dialects.postgresql import UUID, JSONB
from sqlalchemy.orm import relationship
from .base import Base, IDMixin, TimestampMixin

class NounType(str, Enum):
    """12 noun types"""
    SOW = "SOW"
    ITEM = "Item"
    TASK = "Task"
    REQUEST = "Request"
    MEETING = "Meeting"
    DELIVERABLE = "Deliverable"
    EVENT = "Event"
    BLOCKER = "Blocker"
    ARTIFACT = "Artifact"
    GROUP = "Group"
    PROJECT = "Project"
    MILESTONE = "MileStone"

class NounState(str, Enum):
    """Noun state machine"""
    NORMAL = "Normal"
    ESCALATED = "Escalated"
    COMPLETED = "Completed"
    INCOMPLETED = "Incompleted"
    CLOSED = "Closed"
    ARCHIVED = "Archived"

class Noun(Base, IDMixin, TimestampMixin):
    """Core Noun entity with all 12 types"""
    __tablename__ = "nouns"

    type = Column(SQLEnum(NounType), nullable=False, index=True)
    sow_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, index=True)
    parent_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=True)
    short_name = Column(String(255), unique=True, nullable=False, index=True)
    title = Column(String(500), nullable=False)
    description = Column(String(4000), nullable=True)
    state = Column(SQLEnum(NounState), default=NounState.NORMAL, nullable=False, index=True)
    is_blocked = Column(Boolean, default=False, nullable=False, index=True)
    due_date = Column(String, nullable=True)  # ISO 8601 format
    completed_at = Column(String, nullable=True)
    closed_at = Column(String, nullable=True)
    custom_fields = Column(JSONB, default={}, nullable=False)

    # Relationships
    children = relationship(
        "Noun",
        remote_side=[id],
        foreign_keys=[parent_id],
        backref="parent"
    )

    def __repr__(self):
        return f"<Noun {self.short_name} ({self.type.value})>"
