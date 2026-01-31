"""Role and permission models"""

from sqlalchemy import Column, String, JSON, ForeignKey
from sqlalchemy.dialects.postgresql import JSONB, UUID
from src.models.base import Base, IDMixin, TimestampMixin

class Role(Base, IDMixin):
    """Roles with inherited permissions"""
    __tablename__ = "roles"

    name = Column(String(100), unique=True, nullable=False)
    inherits_from = Column(String(100), nullable=True)
    verbs = Column(JSONB, default=[], nullable=False)  # Array of allowed verbs

class RoleMapping(Base, IDMixin):
    """Actor to role mappings"""
    __tablename__ = "role_mappings"

    actor = Column(String(255), nullable=False, index=True)
    role_name = Column(String(100), ForeignKey("roles.name"), nullable=False)
    sow_id = Column(UUID(as_uuid=True), nullable=True)  # SOW-specific if set
