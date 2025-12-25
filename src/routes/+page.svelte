<script lang="ts">
  import SecretList from "$lib/SecretList.svelte";
  import SecretForm from "$lib/SecretForm.svelte";

  let showModal = $state(false);
  let editSecretId = $state<string | null>(null);
  let secretListRef: { loadSecrets: () => Promise<void> } | undefined = $state();
  let showInstructions = $state(true);

  function handleAdd() {
    editSecretId = null;
    showModal = true;
  }

  function handleEdit(id: string) {
    editSecretId = id;
    showModal = true;
  }

  function handleClose() {
    showModal = false;
    editSecretId = null;
  }

  function handleSaved() {
    secretListRef?.loadSecrets();
  }
</script>

<main>
  <div class="instructions">
    <button class="toggle-btn" onclick={() => showInstructions = !showInstructions}>
      {showInstructions ? "Hide" : "Show"} MCP Setup
    </button>

    {#if showInstructions}
      <div class="instructions-content">
        <h3>MCP Server Setup</h3>
        <p>Add to your MCP client config:</p>
        <pre><code>{`"secret-mcp": {
  "command": "npx",
  "args": ["secret-mcp"]
}`}</code></pre>
      </div>
    {/if}
  </div>

  <SecretList bind:this={secretListRef} onAdd={handleAdd} onEdit={handleEdit} />

  {#if showModal}
    <SecretForm secretId={editSecretId} onClose={handleClose} onSaved={handleSaved} />
  {/if}
</main>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
    font-size: 14px;
    line-height: 1.5;
    color: #1a1a1a;
    background-color: #ffffff;
  }

  main {
    min-height: 100vh;
  }

  .instructions {
    padding: 12px 16px;
    background: #f5f5f5;
    border-bottom: 1px solid #e0e0e0;
  }

  .toggle-btn {
    background: none;
    border: 1px solid #ccc;
    padding: 4px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    color: #666;
  }

  .toggle-btn:hover {
    background: #e8e8e8;
  }

  .instructions-content {
    margin-top: 12px;
  }

  .instructions-content h3 {
    margin: 0 0 8px 0;
    font-size: 14px;
    font-weight: 600;
  }

  .instructions-content p {
    margin: 0 0 8px 0;
    font-size: 13px;
    color: #555;
  }

  .instructions-content code {
    background: #e8e8e8;
    padding: 2px 6px;
    border-radius: 3px;
    font-size: 12px;
  }

  .instructions-content pre {
    background: #2d2d2d;
    color: #f8f8f2;
    padding: 12px;
    border-radius: 6px;
    overflow-x: auto;
    margin: 8px 0;
  }

  .instructions-content pre code {
    background: none;
    padding: 0;
    color: inherit;
  }

  .instructions-content .note {
    font-size: 12px;
    color: #777;
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      color: #f0f0f0;
      background-color: #1a1a1a;
    }

    .instructions {
      background: #252525;
      border-bottom-color: #333;
    }

    .toggle-btn {
      border-color: #444;
      color: #aaa;
    }

    .toggle-btn:hover {
      background: #333;
    }

    .instructions-content p {
      color: #bbb;
    }

    .instructions-content code {
      background: #333;
    }

    .instructions-content .note {
      color: #888;
    }
  }
</style>
