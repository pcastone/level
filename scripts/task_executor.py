#!/usr/bin/env python3
"""
Automated Task Executor for Level Project
Parses tasks.md, executes implementation tasks, and marks them complete
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple
from dataclasses import dataclass
from enum import Enum

class TaskType(Enum):
    TASK = "TASK"
    FEAT = "FEAT"
    TEST = "TEST"
    DOCS = "DOCS"
    VALIDATE = "VALIDATE"

@dataclass
class Task:
    phase: int
    task_type: TaskType
    description: str
    subtasks: List[str]
    line_start: int
    line_end: int
    completed: bool = False

class TaskParser:
    """Parse tasks.md file and extract tasks"""

    def __init__(self, tasks_file: Path):
        self.tasks_file = tasks_file
        self.content = tasks_file.read_text()
        self.lines = self.content.split('\n')
        self.tasks: List[Task] = []

    def parse(self) -> List[Task]:
        """Parse all tasks from tasks.md"""
        current_phase = 0
        current_task = None

        for i, line in enumerate(self.lines):
            # Extract phase number
            phase_match = re.match(r'^## Phase (\d+):', line)
            if phase_match:
                current_phase = int(phase_match.group(1))
                continue

            # Extract task header
            task_match = re.match(r'^### Task: \[([A-Z]+)\] (.+)$', line)
            if task_match:
                task_type = TaskType[task_match.group(1)]
                description = task_match.group(2)
                current_task = Task(
                    phase=current_phase,
                    task_type=task_type,
                    description=description,
                    subtasks=[],
                    line_start=i,
                    line_end=i
                )
                self.tasks.append(current_task)
                continue

            # Extract subtasks
            if current_task and line.startswith('- '):
                completed = line.startswith('- [x]')
                current_task.completed = completed
                subtask = line.strip('- []x ')
                current_task.subtasks.append(subtask)
                current_task.line_end = i

        return self.tasks

    def get_incomplete_tasks(self) -> List[Task]:
        """Get only incomplete tasks"""
        return [t for t in self.tasks if not t.completed]

    def get_tasks_by_phase(self, phase: int) -> List[Task]:
        """Get all tasks for a specific phase"""
        return [t for t in self.tasks if t.phase == phase]

    def get_tasks_by_type(self, task_type: TaskType) -> List[Task]:
        """Get all tasks of a specific type"""
        return [t for t in self.tasks if t.task_type == task_type]

class TaskExecutor:
    """Execute tasks and generate code/configurations"""

    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.src_dir = project_root / 'src'
        self.config_dir = project_root / 'config'

    def execute_phase_2(self) -> Tuple[int, int]:
        """Execute Phase 2: Database Schema Implementation
        Returns: (tasks_completed, tasks_failed)
        """
        completed = 0
        failed = 0

        # Create models directory
        models_dir = self.src_dir / 'models'
        models_dir.mkdir(exist_ok=True)

        # Create __init__.py
        (models_dir / '__init__.py').write_text('')

        # Create base model
        self._create_base_model(models_dir)
        completed += 1

        # Create all noun models
        self._create_noun_models(models_dir)
        completed += 1

        # Create junction tables
        self._create_junction_tables(models_dir)
        completed += 1

        # Create transaction model for event sourcing
        self._create_transaction_model(models_dir)
        completed += 1

        # Create sequence and federation models
        self._create_auxiliary_models(models_dir)
        completed += 1

        return completed, failed

    def _create_base_model(self, models_dir: Path) -> None:
        """Create SQLAlchemy base model"""
        content = '''"""Base model for all entities"""

from datetime import datetime
from uuid import uuid4
from sqlalchemy import Column, String, DateTime, func
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.orm import declarative_base

Base = declarative_base()

class TimestampMixin:
    """Mixin for created_at and updated_at timestamps"""
    created_at = Column(DateTime(timezone=True), server_default=func.now(), nullable=False)
    updated_at = Column(DateTime(timezone=True), server_default=func.now(), onupdate=func.now(), nullable=False)

class IDMixin:
    """Mixin for UUID primary key"""
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid4, nullable=False)
'''
        (models_dir / 'base.py').write_text(content)

    def _create_noun_models(self, models_dir: Path) -> None:
        """Create Noun table and all 12 noun types"""
        content = '''"""Noun entities - core domain model"""

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
'''
        (models_dir / 'noun.py').write_text(content)

    def _create_junction_tables(self, models_dir: Path) -> None:
        """Create junction/linking tables"""
        content = '''"""Junction tables for relationships"""

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
'''
        (models_dir / 'relationships.py').write_text(content)

    def _create_transaction_model(self, models_dir: Path) -> None:
        """Create Transaction model for event sourcing"""
        content = '''"""Transaction model for event sourcing"""

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
'''
        (models_dir / 'transaction.py').write_text(content)

    def _create_auxiliary_models(self, models_dir: Path) -> None:
        """Create sequence, alias, and federation models"""
        content = '''"""Auxiliary models for sequences, aliases, and federation"""

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
'''
        (models_dir / 'auxiliary.py').write_text(content)

def main():
    """Main executor function"""
    project_root = Path(__file__).parent.parent
    tasks_file = project_root / 'todo' / 'tasks.md'

    if not tasks_file.exists():
        print(f"Error: tasks.md not found at {tasks_file}")
        return 1

    print("=" * 80)
    print("LEVEL PROJECT - AUTOMATED TASK EXECUTOR")
    print("=" * 80)
    print()

    # Parse tasks
    print("[1/3] Parsing tasks.md...")
    parser = TaskParser(tasks_file)
    all_tasks = parser.parse()
    incomplete_tasks = parser.get_incomplete_tasks()

    print(f"✓ Total tasks: {len(all_tasks)}")
    print(f"✓ Incomplete tasks: {len(incomplete_tasks)}")
    print()

    # Create executor
    print("[2/3] Initializing executor...")
    executor = TaskExecutor(project_root)
    print(f"✓ Project root: {project_root}")
    print()

    # Execute phases
    print("[3/3] Executing phases...")
    print()

    print("Phase 2: Database Schema Implementation")
    print("-" * 80)
    completed, failed = executor.execute_phase_2()
    print(f"✓ Completed: {completed}")
    print(f"✗ Failed: {failed}")
    print()

    print("=" * 80)
    print("EXECUTION COMPLETE")
    print("=" * 80)
    print()
    print("Next steps:")
    print("1. Review generated models in src/models/")
    print("2. Run: poetry install")
    print("3. Update tasks.md to mark tasks complete")
    print("4. Commit changes: git commit -m 'Phase 2: Database Schema Implementation'")
    print()

    return 0

if __name__ == "__main__":
    sys.exit(main())
