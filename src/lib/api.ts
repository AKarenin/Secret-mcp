import { invoke } from "@tauri-apps/api/core";

export interface SecretInfo {
  id: string;
  name: string;
  description: string | null;
  created_at: number;
  updated_at: number;
}

export interface Secret extends SecretInfo {
  value: string;
}

export interface WriteEnvResult {
  success: boolean;
  written: number;
  missing: string[];
}

export async function listSecrets(): Promise<SecretInfo[]> {
  return await invoke("list_secrets");
}

export async function getSecret(id: string): Promise<Secret | null> {
  return await invoke("get_secret", { id });
}

export async function createSecret(
  name: string,
  description: string | null,
  value: string
): Promise<Secret> {
  return await invoke("create_secret", {
    input: { name, description, value },
  });
}

export async function updateSecret(
  id: string,
  name: string,
  description: string | null,
  value: string
): Promise<Secret> {
  return await invoke("update_secret", {
    input: { id, name, description, value },
  });
}

export async function deleteSecret(id: string): Promise<boolean> {
  return await invoke("delete_secret", { id });
}

export async function getDbPath(): Promise<string> {
  return await invoke("get_db_path");
}
