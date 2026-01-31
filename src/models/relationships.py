"""Junction tables for relationships"""

from typing import Optional
from sqlalchemy import Column, String, ForeignKey, Integer, Enum as SQLEnum
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.orm import relationship
from enum import Enum
from .base import Base, IDMixin, TimestampMixin

class ActorRole(str, Enum):
    """Actor roles on Nouns"""
    OWNER = "owner"
    ASSIGNEE = "assignee"
    SME = "sme"
    RESOURCE = "resource"
    STAKEHOLDER = "stakeholder"
    AWARENESS = "awareness"

class NounActor(Base, IDMixin, TimestampMixin):
    """M:N relationship between Nouns and Actors with roles"""
    __tablename__ = "noun_actors"

    noun_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, index=True)
    actor = Column(String(255), nullable=False)  # +person, @group, !vendor, *ai
    role = Column(SQLEnum(ActorRole), nullable=False)

class NounAssignment(Base, IDMixin, TimestampMixin):
    """Container assignments with Kanban sort order"""
    __tablename__ = "noun_assignments"

    noun_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, index=True)
    container_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, index=True)
    sort_order = Column(Integer, nullable=False, default=0)

class NounBlock(Base, IDMixin, TimestampMixin):
    """Blocker relationships"""
    __tablename__ = "noun_blocks"

    blocker_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, index=True)
    target_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, index=True)

class NounHashTag(Base, IDMixin, TimestampMixin):
    """Global tagging system"""
    __tablename__ = "noun_hashtags"

    noun_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, index=True)
    tag = Column(String(255), nullable=False, index=True)
