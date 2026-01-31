<script lang="ts">
  import { onMount } from 'svelte'
  import { activeSow, nouns } from '$lib/stores'
  import { apiClient } from '$lib/api'

  onMount(async () => {
    const sows = await apiClient.get('/sows')
    if (sows.sows.length > 0) {
      activeSow.set(sows.sows[0])
      const nounList = await apiClient.get(`/nouns/${sows.sows[0].id}/nouns`)
      nouns.set(nounList.nouns)
    }
  })
</script>

<div class="container mx-auto p-4">
  <h1 class="text-3xl font-bold">Level - Project Management</h1>
  <p class="text-gray-600 mt-2">Noun/Verb/Container Grammar</p>

  {#if $activeSow}
    <div class="mt-8">
      <h2 class="text-2xl font-bold">{$activeSow.title}</h2>
      <p class="text-sm text-gray-500">{$activeSow.short_name}</p>

      <div class="mt-4 grid grid-cols-1 md:grid-cols-3 gap-4">
        {#each $nouns as noun}
          <div class="p-4 border rounded">
            <h3 class="font-bold">{noun.title}</h3>
            <p class="text-sm text-gray-600">{noun.short_name}</p>
            <span class="inline-block mt-2 px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded">
              {noun.state}
            </span>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  :global(body) {
    @apply bg-white;
  }
</style>
