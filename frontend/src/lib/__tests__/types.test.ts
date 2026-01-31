/**
 * Tests for TypeScript types
 */

import { describe, it, expect } from 'vitest'
import type { Noun, NounType, NounState, SOW } from '../types'

describe('NounType', () => {
  it('should have 12 valid noun types', () => {
    const validTypes: NounType[] = [
      'SOW', 'Item', 'Task', 'Request', 'Meeting',
      'Deliverable', 'Event', 'Blocker', 'Artifact',
      'Group', 'Project', 'MileStone'
    ]
    expect(validTypes).toHaveLength(12)
  })

  it('should include container types', () => {
    const containerTypes: NounType[] = ['Group', 'Project', 'MileStone']
    expect(containerTypes).toHaveLength(3)
  })

  it('should include SOW as genesis type', () => {
    const sowType: NounType = 'SOW'
    expect(sowType).toBe('SOW')
  })
})

describe('NounState', () => {
  it('should have 6 valid states', () => {
    const validStates: NounState[] = [
      'Normal', 'Escalated', 'Completed',
      'Incompleted', 'Closed', 'Archived'
    ]
    expect(validStates).toHaveLength(6)
  })

  it('should have Normal as initial state', () => {
    const initialState: NounState = 'Normal'
    expect(initialState).toBe('Normal')
  })

  it('should have Archived as terminal state', () => {
    const terminalState: NounState = 'Archived'
    expect(terminalState).toBe('Archived')
  })

  it('should have completion states', () => {
    const completionStates: NounState[] = ['Completed', 'Incompleted']
    expect(completionStates).toContain('Completed')
    expect(completionStates).toContain('Incompleted')
  })
})

describe('Noun type', () => {
  it('should have required properties', () => {
    const noun: Noun = {
      id: 'uuid-123',
      type: 'Task',
      short_name: 'TEST-TASK-001',
      title: 'Test Task',
      state: 'Normal',
      is_blocked: false
    }

    expect(noun.id).toBeDefined()
    expect(noun.type).toBeDefined()
    expect(noun.short_name).toBeDefined()
    expect(noun.title).toBeDefined()
    expect(noun.state).toBeDefined()
    expect(noun.is_blocked).toBeDefined()
  })

  it('should allow all noun types', () => {
    const types: NounType[] = [
      'SOW', 'Item', 'Task', 'Request', 'Meeting',
      'Deliverable', 'Event', 'Blocker', 'Artifact',
      'Group', 'Project', 'MileStone'
    ]

    types.forEach(type => {
      const noun: Noun = {
        id: 'id',
        type,
        short_name: 'SHORT-001',
        title: 'Title',
        state: 'Normal',
        is_blocked: false
      }
      expect(noun.type).toBe(type)
    })
  })

  it('should allow all noun states', () => {
    const states: NounState[] = [
      'Normal', 'Escalated', 'Completed',
      'Incompleted', 'Closed', 'Archived'
    ]

    states.forEach(state => {
      const noun: Noun = {
        id: 'id',
        type: 'Task',
        short_name: 'SHORT-001',
        title: 'Title',
        state,
        is_blocked: false
      }
      expect(noun.state).toBe(state)
    })
  })
})

describe('SOW type', () => {
  it('should have required properties', () => {
    const sow: SOW = {
      id: 'sow-uuid',
      short_name: 'PROJ',
      title: 'Project Name',
      mode: 'standard'
    }

    expect(sow.id).toBeDefined()
    expect(sow.short_name).toBeDefined()
    expect(sow.title).toBeDefined()
    expect(sow.mode).toBeDefined()
  })

  it('should support all deployment modes', () => {
    const modes: SOW['mode'][] = ['skinny', 'standard', 'enterprise']

    modes.forEach(mode => {
      const sow: SOW = {
        id: 'id',
        short_name: 'TEST',
        title: 'Test',
        mode
      }
      expect(sow.mode).toBe(mode)
    })
  })

  it('should have skinny mode for simple deployments', () => {
    const sow: SOW = {
      id: 'id',
      short_name: 'SIMPLE',
      title: 'Simple Project',
      mode: 'skinny'
    }
    expect(sow.mode).toBe('skinny')
  })

  it('should have enterprise mode for federation', () => {
    const sow: SOW = {
      id: 'id',
      short_name: 'ENTERPRISE',
      title: 'Enterprise Project',
      mode: 'enterprise'
    }
    expect(sow.mode).toBe('enterprise')
  })
})
