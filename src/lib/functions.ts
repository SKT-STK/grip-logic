import { invoke } from "@tauri-apps/api/core";
import { type Store } from "@tauri-apps/plugin-store";

export async function dataStoreSaveAndNotify(store?: Store | null) {
  await store?.save()
  await invoke('data_store_saved')
}
