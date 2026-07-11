import { invoke } from "@tauri-apps/api/core"

export default function SetWindowVisible() {
  invoke('set_window_visible')

  return <></>
}
