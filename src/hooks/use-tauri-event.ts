import { type EventName, listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useEffect } from 'react'

export function useTauriEvent(callback: (payload: unknown, eventName?: EventName, id?: number) => void, eventName?: string, ) {
  useEffect(() => {
    let unlisten: UnlistenFn = () => {}
    let isMounted = true;

    (async () => {
      if (eventName === undefined) return
      unlisten = await listen(eventName, e => {
        if (!isMounted) return

        callback(e.payload, e.event, e.id)
      })
    })()

    return () => {
      isMounted = false
      unlisten()
    }
  }, [])
}
