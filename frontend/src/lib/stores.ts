import { writable } from 'svelte/store'
import type { Noun, SOW } from './types'

export const currentUser = writable<string | null>(null)
export const activeSow = writable<SOW | null>(null)
export const nouns = writable<Noun[]>([])
export const loading = writable(false)

export const preferences = writable({
  theme: 'light',
  timezone: 'UTC',
  default_view: 'timeline'
})
