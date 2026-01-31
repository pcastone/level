import { env } from '$env/dynamic/public'

export class APIClient {
  private baseUrl: string
  private apiKey: string

  constructor() {
    this.baseUrl = env.PUBLIC_API_URL || 'http://localhost:8000'
    this.apiKey = localStorage.getItem('api_key') || ''
  }

  async get(endpoint: string) {
    return fetch(`${this.baseUrl}${endpoint}`, {
      headers: { 'X-API-Key': this.apiKey }
    }).then(r => r.json())
  }

  async post(endpoint: string, data: any) {
    return fetch(`${this.baseUrl}${endpoint}`, {
      method: 'POST',
      headers: { 'X-API-Key': this.apiKey, 'Content-Type': 'application/json' },
      body: JSON.stringify(data)
    }).then(r => r.json())
  }
}

export const apiClient = new APIClient()
