"""Actor models: Person, Group, Vendor, AI"""

from sqlalchemy import Column, String, Enum as SQLEnum, Boolean
from sqlalchemy.orm import relationship
from enum import Enum
from src.models.base import Base, IDMixin, TimestampMixin

class ActorType(str, Enum):
    PERSON = "Person"
    GROUP = "Group"
    VENDOR = "Vendor"
    AI = "AI"

class Person(Base, IDMixin, TimestampMixin):
    """Internal users or LDAP-sourced persons"""
    __tablename__ = "persons"

    email = Column(String(255), unique=True, nullable=False, index=True)
    name = Column(String(255), nullable=False)
    ldap_source = Column(Boolean, default=False)
    ldap_dn = Column(String(500), nullable=True)

class Group(Base, IDMixin, TimestampMixin):
    """Internal groups (LDAP or manually managed)"""
    __tablename__ = "groups"

    name = Column(String(255), unique=True, nullable=False, index=True)
    description = Column(String(1000), nullable=True)

class Vendor(Base, IDMixin, TimestampMixin):
    """External vendor actors"""
    __tablename__ = "vendors"

    name = Column(String(255), unique=True, nullable=False, index=True)
    contact = Column(String(255), nullable=True)

class AI(Base, IDMixin, TimestampMixin):
    """AI actor registration"""
    __tablename__ = "ais"

    name = Column(String(255), unique=True, nullable=False, index=True)
    model_id = Column(String(255), nullable=False)
    api_key_hash = Column(String(255), nullable=False)

class GroupMember(Base, IDMixin):
    """Group membership"""
    __tablename__ = "group_members"

    group_id = Column(String, nullable=False)
    member = Column(String(255), nullable=False)  # +person, @group, !vendor, *ai
