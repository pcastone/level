/**
 * Tests for Svelte stores
 */

import { describe, it, expect, beforeEach } from 'vitest'
import { get } from 'svelte/store'
import { currentUser, activeSow, nouns, loading, preferences } from '../stores'
import type { Noun, SOW } from '../types'

describe('currentUser store', () => {
  beforeEach(() => {
    currentUser.set(null)
  })

  it('should initialize as null', () => {
    expect(get(currentUser)).toBeNull()
  })

  it('should update user', () => {
    currentUser.set('john.doe')
    expect(get(currentUser)).toBe('john.doe')
  })

  it('should clear user', () => {
    currentUser.set('john.doe')
    currentUser.set(null)
    expect(get(currentUser)).toBeNull()
  })
})

describe('activeSow store', () => {
  beforeEach(() => {
    activeSow.set(null)
  })

  it('should initialize as null', () => {
    expect(get(activeSow)).toBeNull()
  })

  it('should update active SOW', () => {
    const sow: SOW = {
      id: 'sow-123',
      short_name: 'TEST',
      title: 'Test Project',
      mode: 'standard'
    }
    activeSow.set(sow)
    expect(get(activeSow)).toEqual(sow)
  })

  it('should support all deployment modes', () => {
    const modes: SOW['mode'][] = ['skinny', 'standard', 'enterprise']

    modes.forEach(mode => {
      const sow: SOW = {
        id: 'sow-123',
        short_name: 'TEST',
        title: 'Test',
        mode
      }
      activeSow.set(sow)
      expect(get(activeSow)?.mode).toBe(mode)
    })
  })
})

describe('nouns store', () => {
  beforeEach(() => {
    nouns.set([])
  })

  it('should initialize as empty array', () => {
    expect(get(nouns)).toEqual([])
  })

  it('should store noun list', () => {
    const nounList: Noun[] = [
      {
        id: 'noun-1',
        type: 'Task',
        short_name: 'TEST-TASK-001',
        title: 'First Task',
        state: 'Normal',
        is_blocked: false
      },
      {
        id: 'noun-2',
        type: 'Task',
        short_name: 'TEST-TASK-002',
        title: 'Second Task',
        state: 'Escalated',
        is_blocked: false
      }
    ]
    nouns.set(nounList)
    expect(get(nouns)).toHaveLength(2)
  })

  it('should support all noun types', () => {
    const types: Noun['type'][] = [
      'SOW', 'Item', 'Task', 'Request', 'Meeting',
      'Deliverable', 'Event', 'Blocker', 'Artifact',
      'Group', 'Project', 'MileStone'
    ]

    const nounList: Noun[] = types.map((type, i) => ({
      id: `noun-${i}`,
      type,
      short_name: `TEST-${type.toUpperCase().slice(0, 4)}-001`,
      title: `Test ${type}`,
      state: 'Normal',
      is_blocked: false
    }))

    nouns.set(nounList)
    expect(get(nouns)).toHaveLength(12)
  })

  it('should support all noun states', () => {
    const states: Noun['state'][] = [
      'Normal', 'Escalated', 'Completed',
      'Incompleted', 'Closed', 'Archived'
    ]

    const nounList: Noun[] = states.map((state, i) => ({
      id: `noun-${i}`,
      type: 'Task',
      short_name: `TEST-TASK-00${i + 1}`,
      title: `Task in ${state}`,
      state,
      is_blocked: false
    }))

    nouns.set(nounList)
    expect(get(nouns)).toHaveLength(6)
  })

  it('should support blocked nouns', () => {
    const noun: Noun = {
      id: 'blocked-noun',
      type: 'Task',
      short_name: 'TEST-TASK-001',
      title: 'Blocked Task',
      state: 'Normal',
      is_blocked: true
    }
    nouns.set([noun])
    expect(get(nouns)[0].is_blocked).toBe(true)
  })
})

describe('loading store', () => {
  beforeEach(() => {
    loading.set(false)
  })

  it('should initialize as false', () => {
    expect(get(loading)).toBe(false)
  })

  it('should toggle loading state', () => {
    loading.set(true)
    expect(get(loading)).toBe(true)
    loading.set(false)
    expect(get(loading)).toBe(false)
  })
})

describe('preferences store', () => {
  it('should have default values', () => {
    const prefs = get(preferences)
    expect(prefs.theme).toBe('light')
    expect(prefs.timezone).toBe('UTC')
    expect(prefs.default_view).toBe('timeline')
  })

  it('should update theme preference', () => {
    preferences.update(p => ({ ...p, theme: 'dark' }))
    expect(get(preferences).theme).toBe('dark')
  })

  it('should update timezone preference', () => {
    preferences.update(p => ({ ...p, timezone: 'America/New_York' }))
    expect(get(preferences).timezone).toBe('America/New_York')
  })

  it('should update default_view preference', () => {
    const views = ['timeline', 'kanban', 'calendar']

    views.forEach(view => {
      preferences.update(p => ({ ...p, default_view: view }))
      expect(get(preferences).default_view).toBe(view)
    })
  })

  it('should preserve other preferences when updating one', () => {
    preferences.update(p => ({ ...p, theme: 'dark' }))

    const prefs = get(preferences)
    expect(prefs.theme).toBe('dark')
    expect(prefs.timezone).toBe('UTC')
    expect(prefs.default_view).toBe('timeline')
  })
})
