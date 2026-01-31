# Unit Test Infrastructure Build Plan
Generated: 2026-01-30
Completed: 2026-01-30

## Overview
Build unit test infrastructure for Backend (Python/FastAPI), CLI Client (Python/Typer), and Frontend (SvelteKit/TypeScript).

---

## Phase 1: Backend Test Infrastructure (pytest)

### Task 1.1: Configure pytest
- [x] Add pytest configuration to pyproject.toml
- [x] Create tests/conftest.py with fixtures
- [x] Set up async test support with pytest-asyncio

### Task 1.2: Create test fixtures
- [x] Database session fixture with rollback
- [x] Factory functions for Noun, SOW, Actor test data
- [x] Mock API client fixture

### Task 1.3: Model unit tests
- [x] tests/unit/test_models.py - Noun, Transaction, Actor models
- [x] Test NounType and NounState enums
- [x] Test JSONB custom_fields

### Task 1.4: Service unit tests
- [x] tests/unit/test_services.py - StateTransition, BlockingService
- [x] Test Short Name generation
- [x] Test Transaction/Event sourcing

### Task 1.5: Verb handler unit tests
- [x] tests/unit/test_verbs.py - All 12 verb handlers
- [x] Test state guards and transitions

### Task 1.6: API endpoint tests
- [x] tests/integration/test_api.py - SOW, Noun, Transaction, View endpoints

---

## Phase 2: CLI Client Tests (pytest)

### Task 2.1: CLI test setup
- [x] tests/cli/conftest.py - CLI fixtures
- [x] Mock HTTP client fixture

### Task 2.2: Reference resolution tests
- [x] tests/cli/test_references.py - UUID, ShortName, Alias, Title resolution

### Task 2.3: Command tests
- [x] tests/cli/test_commands.py - SOW, Noun, Verb commands

---

## Phase 3: MCP Server Tests (pytest)

### Task 3.1: MCP test setup
- [x] tests/mcp/conftest.py - MCP fixtures

### Task 3.2: Tool tests
- [x] tests/mcp/test_tools.py - All MCP tools

---

## Phase 4: Frontend Tests (Vitest)

### Task 4.1: Configure Vitest
- [x] Create vitest.config.ts
- [x] Set up jsdom environment
- [x] Configure SvelteKit testing

### Task 4.2: API client tests
- [x] frontend/src/lib/__tests__/api.test.ts

### Task 4.3: Store tests
- [x] frontend/src/lib/__tests__/stores.test.ts

### Task 4.4: Type tests
- [x] frontend/src/lib/__tests__/types.test.ts

---

## Summary

| Phase | Tests | Status |
|-------|-------|--------|
| Backend Infrastructure | 6 tasks | ✅ Complete |
| CLI Client Tests | 3 tasks | ✅ Complete |
| MCP Server Tests | 2 tasks | ✅ Complete |
| Frontend Tests | 4 tasks | ✅ Complete |
| **Total** | **15 tasks** | **✅ ALL COMPLETE** |

---

## Files Created

### Backend Tests (pytest)
- `tests/__init__.py`
- `tests/conftest.py` - Shared fixtures, factories, mocks
- `tests/unit/__init__.py`
- `tests/unit/test_models.py` - Noun, NounType, NounState tests
- `tests/unit/test_services.py` - StateTransition, Blocking, Transaction, ShortName tests
- `tests/unit/test_verbs.py` - All 12 verb handler tests
- `tests/integration/__init__.py`
- `tests/integration/test_api.py` - API endpoint tests

### CLI Tests (pytest)
- `tests/cli/__init__.py`
- `tests/cli/conftest.py` - CLI fixtures
- `tests/cli/test_references.py` - Reference resolution chain tests
- `tests/cli/test_commands.py` - Command tests

### MCP Tests (pytest)
- `tests/mcp/__init__.py`
- `tests/mcp/conftest.py` - MCP fixtures
- `tests/mcp/test_tools.py` - MCP tool tests

### Frontend Tests (Vitest)
- `frontend/vitest.config.ts` - Vitest configuration
- `frontend/src/lib/__tests__/setup.ts` - Test setup
- `frontend/src/lib/__tests__/api.test.ts` - API client tests
- `frontend/src/lib/__tests__/stores.test.ts` - Svelte store tests
- `frontend/src/lib/__tests__/types.test.ts` - TypeScript type tests

---

## Running Tests

### Backend Tests
```bash
# Run all backend tests
poetry run pytest

# Run with coverage
poetry run pytest --cov=src --cov-report=html

# Run specific test file
poetry run pytest tests/unit/test_models.py -v

# Run specific test class
poetry run pytest tests/unit/test_services.py::TestStateTransition -v
```

### Frontend Tests
```bash
cd frontend

# Run all tests
npm test

# Run tests once
npm run test:run

# Run with coverage
npm run test:coverage
```
