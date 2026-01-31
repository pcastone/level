/**
 * Vitest setup file for frontend tests
 */

import { vi } from 'vitest'

// Mock localStorage
const localStorageMock = {
  getItem: vi.fn(() => null),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
}
vi.stubGlobal('localStorage', localStorageMock)

// Mock fetch
global.fetch = vi.fn()

// Mock SvelteKit environment
vi.mock('$env/dynamic/public', () => ({
  env: {
    PUBLIC_API_URL: 'http://localhost:8000'
  }
}))

// Reset mocks before each test
beforeEach(() => {
  vi.clearAllMocks()
  localStorageMock.getItem.mockReturnValue(null)
})
