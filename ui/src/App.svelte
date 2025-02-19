<script lang="ts">
import type { ActionHash, AppClient, HolochainError } from "@holochain/client";
import { AppWebsocket } from "@holochain/client";
import { onMount, setContext } from "svelte";

import { type ClientContext, clientContext } from "./contexts";

import Banner from './Banner.svelte'
import CreatePost from './blog/blog/CreatePost.svelte'
import AllPosts from './blog/blog/AllPosts.svelte'

let client: AppClient | undefined;
let error: HolochainError | undefined;
let loading = false;

const appClientContext = {
  getClient: async () => {
    if (!client) {
      client = await AppWebsocket.connect();
    }
    return client;
  },
};

onMount(async () => {
  try {
    loading = true;
    client = await appClientContext.getClient();
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
});

setContext<ClientContext>(clientContext, appClientContext);
</script>

<Banner challengeName={'Links and Collections'} challengeNumber={2}>
  <div>
    {#if loading}
      Loading
    {:else if client}
      <!-- Add your CreatePost and AllPosts components here -->
    {/if}

  </div>
</Banner>
