#!/usr/bin/env python3
"""Mark a phase's tasks as complete in tasks.md"""

import sys
from pathlib import Path

def mark_phase_complete(phase_num: int):
    """Mark all tasks in a phase as complete"""
    tasks_file = Path(__file__).parent.parent / 'todo' / 'tasks.md'
    content = tasks_file.read_text()
    lines = content.split('\n')
    
    in_phase = False
    in_next_phase = False
    modified_lines = []
    task_count = 0
    
    for i, line in enumerate(lines):
        # Check if we're entering the target phase
        if f"## Phase {phase_num}:" in line:
            in_phase = True
            in_next_phase = False
            modified_lines.append(line)
            continue
        
        # Check if we're entering the next phase
        if in_phase and line.startswith("## Phase") and f"Phase {phase_num}" not in line:
            in_phase = False
            in_next_phase = True
        
        # Mark uncompleted subtasks as complete in current phase
        if in_phase and line.startswith("- [ ]"):
            modified_line = line.replace("- [ ]", "- [x]", 1)
            # Add completion date before the newline
            if " (done:" not in modified_line:
                modified_line = modified_line.rstrip() + " (done: 2026-01-30)"
            modified_lines.append(modified_line)
            task_count += 1
            continue
        
        modified_lines.append(line)
    
    # Write updated content
    tasks_file.write_text('\n'.join(modified_lines))
    return task_count

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: mark_phase_complete.py <phase_number>")
        sys.exit(1)
    
    phase = int(sys.argv[1])
    count = mark_phase_complete(phase)
    print(f"✓ Marked {count} tasks in Phase {phase} as complete")
