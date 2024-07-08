<script lang="ts">
  import { onMount, getContext } from 'svelte'
  import '@material/mwc-circular-progress'
  import type {
    Link,
    ActionHash,
    EntryHash,
    AppClient,
    Record,
    AgentPubKey,
    NewEntryAction,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Comment, BlogSignal } from './types'
  import CommentDetail from './CommentDetail.svelte'

  export let postHash: ActionHash

  let client: AppClient = (getContext(clientContext) as any).getClient()

  let hashes: Array<ActionHash> | undefined = []

  let loading: boolean
  let error: any = undefined

  $: hashes, loading, error

  onMount(async () => {
    if (postHash === undefined) {
      throw new Error(
        `The postHash input is required for the CommentsForPost element`
      )
    }

    await fetchComments()

    client.on('signal', async (signal) => {
      if (signal.zome_name !== 'blog') return
      const payload = signal.payload as BlogSignal
      if (
        !(
          payload.type === 'EntryCreated' &&
          payload.app_entry.type === 'Comment'
        )
      )
        return
      await fetchComments()
    })
  })

  async function fetchComments() {
    loading = true
    try {
      const links: Array<Link> = await client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'get_comments_for_post',
        payload: postHash,
      })
      hashes = links.map((l) => l.target)
    } catch (e) {
      error = e
    }
    loading = false
  }
</script>

{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{:else if error}
  <span>Error fetching comments: ${error}.</span>
{:else}
  <div style="display: flex; flex-direction: column">
    {#each hashes as hash}
      <div style="margin-bottom: 8px;">
        <CommentDetail commentHash={hash} on:comment-deleted={fetchComments}
        ></CommentDetail>
      </div>
    {/each}
  </div>
{/if}
