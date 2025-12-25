<script lang="ts">
  import { createSecret, updateSecret, getSecret, type Secret } from "./api";

  interface Props {
    secretId: string | null;
    onClose: () => void;
    onSaved: () => void;
  }

  let { secretId, onClose, onSaved }: Props = $props();

  let name = $state("");
  let description = $state("");
  let value = $state("");
  let showValue = $state(false);
  let loading = $state(false);
  let error = $state("");

  let isEdit = $derived(secretId !== null);

  $effect(() => {
    if (secretId) {
      loadSecret(secretId);
    }
  });

  async function loadSecret(id: string) {
    loading = true;
    error = "";
    try {
      const secret = await getSecret(id);
      if (secret) {
        name = secret.name;
        description = secret.description || "";
        value = secret.value;
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleSubmit(event: Event) {
    event.preventDefault();
    loading = true;
    error = "";

    try {
      if (isEdit && secretId) {
        await updateSecret(secretId, name, description || null, value);
      } else {
        await createSecret(name, description || null, value);
      }
      onSaved();
      onClose();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onClose();
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={handleBackdropClick}>
  <div class="modal">
    <div class="modal-header">
      <h2>{isEdit ? "Edit Secret" : "Add Secret"}</h2>
      <button class="close-btn" onclick={onClose} disabled={loading}>&times;</button>
    </div>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    <form onsubmit={handleSubmit}>
      <div class="form-group">
        <label for="name">Name *</label>
        <input
          id="name"
          type="text"
          bind:value={name}
          placeholder="e.g., DATABASE_URL"
          required
          disabled={loading}
        />
      </div>

      <div class="form-group">
        <label for="description">Description</label>
        <input
          id="description"
          type="text"
          bind:value={description}
          placeholder="Optional description"
          disabled={loading}
        />
      </div>

      <div class="form-group">
        <label for="value">Value *</label>
        <div class="value-input">
          <input
            id="value"
            type={showValue ? "text" : "password"}
            bind:value={value}
            placeholder="Secret value"
            required
            disabled={loading}
          />
          <button
            type="button"
            class="toggle-btn"
            onclick={() => (showValue = !showValue)}
            disabled={loading}
          >
            {showValue ? "Hide" : "Show"}
          </button>
        </div>
      </div>

      <div class="form-actions">
        <button type="button" onclick={onClose} disabled={loading}>Cancel</button>
        <button type="submit" class="primary" disabled={loading}>
          {loading ? "Saving..." : "Save"}
        </button>
      </div>
    </form>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: var(--bg-color, #fff);
    border-radius: 8px;
    padding: 20px;
    width: 90%;
    max-width: 400px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.25rem;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    line-height: 1;
    color: inherit;
  }

  .error {
    background: #fee;
    color: #c00;
    padding: 8px 12px;
    border-radius: 4px;
    margin-bottom: 16px;
    font-size: 0.875rem;
  }

  .form-group {
    margin-bottom: 16px;
  }

  .form-group label {
    display: block;
    margin-bottom: 4px;
    font-weight: 500;
    font-size: 0.875rem;
  }

  .form-group input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 1rem;
    box-sizing: border-box;
  }

  .value-input {
    display: flex;
    gap: 8px;
  }

  .value-input input {
    flex: 1;
  }

  .toggle-btn {
    padding: 8px 12px;
    white-space: nowrap;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 20px;
  }

  button {
    padding: 8px 16px;
    border-radius: 4px;
    border: 1px solid #ccc;
    background: #f5f5f5;
    cursor: pointer;
    font-size: 0.875rem;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  button.primary {
    background: #4a90d9;
    color: white;
    border-color: #3a80c9;
  }

  button.primary:hover:not(:disabled) {
    background: #3a80c9;
  }

  @media (prefers-color-scheme: dark) {
    .modal {
      --bg-color: #2a2a2a;
    }

    .form-group input {
      background: #1a1a1a;
      border-color: #444;
      color: #f0f0f0;
    }

    .error {
      background: #4a1a1a;
      color: #faa;
    }

    button {
      background: #3a3a3a;
      border-color: #555;
      color: #f0f0f0;
    }
  }
</style>
