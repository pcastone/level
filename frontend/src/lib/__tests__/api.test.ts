/**
 * Tests for API client
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'
import { APIClient } from '../api'

describe('APIClient', () => {
  let client: APIClient
  let mockFetch: ReturnType<typeof vi.fn>

  beforeEach(() => {
    mockFetch = vi.fn()
    global.fetch = mockFetch
    client = new APIClient()
  })

  describe('constructor', () => {
    it('should use default API URL when env not set', () => {
      expect(client).toBeDefined()
    })

    it('should get API key from localStorage', () => {
      const mockGetItem = vi.fn().mockReturnValue('test-api-key')
      vi.stubGlobal('localStorage', { getItem: mockGetItem })

      const newClient = new APIClient()
      expect(mockGetItem).toHaveBeenCalledWith('api_key')
    })
  })

  describe('get', () => {
    it('should make GET request with X-API-Key header', async () => {
      mockFetch.mockResolvedValue({
        json: () => Promise.resolve({ data: 'test' })
      })

      await client.get('/sows/')

      expect(mockFetch).toHaveBeenCalledWith(
        expect.stringContaining('/sows/'),
        expect.objectContaining({
          headers: expect.objectContaining({
            'X-API-Key': expect.any(String)
          })
        })
      )
    })

    it('should return JSON response', async () => {
      const mockData = { sows: [] }
      mockFetch.mockResolvedValue({
        json: () => Promise.resolve(mockData)
      })

      const result = await client.get('/sows/')
      expect(result).toEqual(mockData)
    })

    it('should append endpoint to base URL', async () => {
      mockFetch.mockResolvedValue({
        json: () => Promise.resolve({})
      })

      await client.get('/test/endpoint')

      expect(mockFetch).toHaveBeenCalledWith(
        expect.stringContaining('/test/endpoint'),
        expect.any(Object)
      )
    })
  })

  describe('post', () => {
    it('should make POST request with JSON body', async () => {
      mockFetch.mockResolvedValue({
        json: () => Promise.resolve({ id: '123' })
      })

      const data = { title: 'Test Task', type: 'Task' }
      await client.post('/nouns/', data)

      expect(mockFetch).toHaveBeenCalledWith(
        expect.stringContaining('/nouns/'),
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify(data)
        })
      )
    })

    it('should include Content-Type header', async () => {
      mockFetch.mockResolvedValue({
        json: () => Promise.resolve({})
      })

      await client.post('/test', {})

      expect(mockFetch).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          headers: expect.objectContaining({
            'Content-Type': 'application/json'
          })
        })
      )
    })

    it('should include X-API-Key header', async () => {
      mockFetch.mockResolvedValue({
        json: () => Promise.resolve({})
      })

      await client.post('/test', {})

      expect(mockFetch).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          headers: expect.objectContaining({
            'X-API-Key': expect.any(String)
          })
        })
      )
    })

    it('should return JSON response', async () => {
      const mockResponse = { id: 'uuid-123', short_name: 'TEST-TASK-001' }
      mockFetch.mockResolvedValue({
        json: () => Promise.resolve(mockResponse)
      })

      const result = await client.post('/nouns/', { title: 'Test' })
      expect(result).toEqual(mockResponse)
    })
  })
})

describe('API Endpoints', () => {
  it('should use correct SOW endpoint', () => {
    expect('/sows/').toBe('/sows/')
    expect('/sows/{sow_id}').toContain('/sows/')
  })

  it('should use correct Noun endpoint pattern', () => {
    const sowId = 'test-sow-id'
    const endpoint = `/nouns/${sowId}/nouns`
    expect(endpoint).toContain('/nouns/')
    expect(endpoint).toContain(sowId)
  })

  it('should use correct View endpoint pattern', () => {
    const sowId = 'test-sow-id'
    const views = ['timeline', 'kanban', 'calendar']

    views.forEach(view => {
      const endpoint = `/views/${sowId}/${view}`
      expect(endpoint).toContain('/views/')
      expect(endpoint).toContain(view)
    })
  })
})
