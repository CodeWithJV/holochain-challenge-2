<script lang="ts">
  import { onMount, getContext } from 'svelte'
  import '@material/mwc-circular-progress'
  import type { Record, ActionHash, AppClient } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import PostDetail from './PostDetail.svelte'
  import type { BlogSignal } from './types'

  let client: AppClient = (getContext(clientContext) as any).getClient()

  let hashes: Array<ActionHash> | undefined
  let loading = true
  let error: any = undefined

  $: hashes, loading, error

  onMount(async () => {
    // Delay by a bit to allow backend to bootup
    setTimeout(async () => {
      await fetchPosts()
    }, 300)
    client.on('signal', (signal) => {
      if (signal.zome_name !== 'blog') return
      const payload = signal.payload as BlogSignal
      if (payload.type !== 'EntryCreated') return
      if (payload.app_entry.type !== 'Post') return
      hashes = [...hashes, payload.action.hashed.hash]
    })
  })

  async function fetchPosts() {
    try {
      const links = await client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'get_all_posts',
        payload: null,
      })
      hashes = links.map((l) => l.target)
    } catch (e) {
      error = e
    }
    loading = false
  }
</script>

{#if loading}
  <mwc-circular-progress indeterminate></mwc-circular-progress>
{:else if error}
  <span>Error fetching the posts: {error}.</span>
{:else if hashes.length === 0}
  <span>No posts found.</span>
{:else}
  <div style="display: flex; flex-direction: column; width: 100%;">
    {#each hashes as hash}
      <div style="margin-bottom: 8px;">
        <PostDetail postHash={hash} on:post-deleted={() => fetchPosts()}
        ></PostDetail>
      </div>
    {/each}
  </div>
{/if}
