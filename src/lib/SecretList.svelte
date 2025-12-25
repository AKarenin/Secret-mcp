<script lang="ts">
  import { listSecrets, deleteSecret, type SecretInfo } from "./api";

  interface Props {
    onAdd: () => void;
    onEdit: (id: string) => void;
  }

  let { onAdd, onEdit }: Props = $props();

  let secrets = $state<SecretInfo[]>([]);
  let searchQuery = $state("");
  let loading = $state(true);
  let error = $state("");

  let filteredSecrets = $derived(
    secrets.filter(
      (s) =>
        s.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (s.description || "").toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  $effect(() => {
    loadSecrets();
  });

  export async function loadSecrets() {
    loading = true;
    error = "";
    try {
      secrets = await listSecrets();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleDelete(id: string, name: string) {
    if (!confirm(`Delete secret "${name}"?`)) return;

    try {
      await deleteSecret(id);
      await loadSecrets();
    } catch (e) {
      error = String(e);
    }
  }

  function formatDate(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleDateString();
  }
</script>

<div class="container">
  <div class="header">
    <h1>Secrets</h1>
    <button class="add-btn" onclick={onAdd}>+ Add Secret</button>
  </div>

  <div class="search-bar">
    <input
      type="text"
      placeholder="Search secrets..."
      bind:value={searchQuery}
    />
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if loading}
    <div class="loading">Loading...</div>
  {:else if filteredSecrets.length === 0}
    <div class="empty">
      {#if searchQuery}
        No secrets match your search.
      {:else}
        No secrets yet. Click "Add Secret" to create one.
      {/if}
    </div>
  {:else}
    <div class="table-container">
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Description</th>
            <th>Value</th>
            <th>Updated</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredSecrets as secret (secret.id)}
            <tr>
              <td class="name">{secret.name}</td>
              <td class="description">{secret.description || "-"}</td>
              <td class="value">••••••••</td>
              <td class="date">{formatDate(secret.updated_at)}</td>
              <td class="actions">
                <button onclick={() => onEdit(secret.id)}>Edit</button>
                <button
                  class="delete"
                  onclick={() => handleDelete(secret.id, secret.name)}
                >
                  Delete
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<style>
  .container {
    padding: 16px;
    max-width: 100%;
    overflow: hidden;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  h1 {
    margin: 0;
    font-size: 1.5rem;
  }

  .add-btn {
    background: #4a90d9;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .add-btn:hover {
    background: #3a80c9;
  }

  .search-bar {
    margin-bottom: 16px;
  }

  .search-bar input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 1rem;
    box-sizing: border-box;
  }

  .error {
    background: #fee;
    color: #c00;
    padding: 8px 12px;
    border-radius: 4px;
    margin-bottom: 16px;
  }

  .loading,
  .empty {
    text-align: center;
    padding: 40px 20px;
    color: #666;
  }

  .table-container {
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
  }

  th,
  td {
    text-align: left;
    padding: 10px 12px;
    border-bottom: 1px solid #eee;
  }

  th {
    background: #f5f5f5;
    font-weight: 600;
    white-space: nowrap;
  }

  .name {
    font-family: monospace;
    font-weight: 500;
  }

  .description {
    color: #666;
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .value {
    font-family: monospace;
    color: #999;
  }

  .date {
    white-space: nowrap;
    color: #666;
  }

  .actions {
    white-space: nowrap;
  }

  .actions button {
    padding: 4px 8px;
    margin-right: 4px;
    border: 1px solid #ccc;
    border-radius: 4px;
    background: #fff;
    cursor: pointer;
    font-size: 0.75rem;
  }

  .actions button:hover {
    background: #f0f0f0;
  }

  .actions button.delete {
    color: #c00;
    border-color: #c00;
  }

  .actions button.delete:hover {
    background: #fee;
  }

  @media (prefers-color-scheme: dark) {
    .search-bar input {
      background: #1a1a1a;
      border-color: #444;
      color: #f0f0f0;
    }

    th {
      background: #333;
    }

    th,
    td {
      border-color: #444;
    }

    .description,
    .date {
      color: #999;
    }

    .error {
      background: #4a1a1a;
      color: #faa;
    }

    .loading,
    .empty {
      color: #888;
    }

    .actions button {
      background: #2a2a2a;
      border-color: #555;
      color: #f0f0f0;
    }

    .actions button:hover {
      background: #3a3a3a;
    }

    .actions button.delete {
      color: #f88;
      border-color: #f88;
    }

    .actions button.delete:hover {
      background: #4a2a2a;
    }
  }
</style>
