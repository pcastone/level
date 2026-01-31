"""Transaction model for event sourcing"""

from typing import Optional, Dict, Any
from sqlalchemy import Column, String, Integer, JSON, ForeignKey
from sqlalchemy.dialects.postgresql import UUID, JSONB
from .base import Base, IDMixin, TimestampMixin

class Transaction(Base, IDMixin, TimestampMixin):
    """Event log for all noun changes (event sourcing)"""
    __tablename__ = "transactions"

    sequence = Column(Integer, nullable=False)
    sow_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, index=True)
    noun_id = Column(UUID(as_uuid=True), ForeignKey("nouns.id"), nullable=False, index=True)
    verb = Column(String(50), nullable=False, index=True)  # Open, Complete, Update, etc.
    actor = Column(String(255), nullable=False)  # +person, @group, !vendor, *ai
    before_snapshot = Column(JSONB, nullable=False)  # State before change
    after_snapshot = Column(JSONB, nullable=False)   # State after change
    context = Column(JSONB, nullable=True)  # Verb-specific context

    def __repr__(self):
        return f"<Transaction seq={self.sequence} verb={self.verb} actor={self.actor}>"
