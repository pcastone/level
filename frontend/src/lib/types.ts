export type Noun = {
  id: string
  type: NounType
  short_name: string
  title: string
  state: NounState
  is_blocked: boolean
}

export type NounType = 'SOW' | 'Item' | 'Task' | 'Request' | 'Meeting' | 'Deliverable' | 'Event' | 'Blocker' | 'Artifact' | 'Group' | 'Project' | 'MileStone'

export type NounState = 'Normal' | 'Escalated' | 'Completed' | 'Incompleted' | 'Closed' | 'Archived'

export type SOW = {
  id: string
  short_name: string
  title: string
  mode: 'skinny' | 'standard' | 'enterprise'
}
