import { useEffect, useRef } from "react";

export function useAsyncInterval(effect: () => Promise<(() => (void | Promise<void>)) | void>, timeout: number) {
  const savedEffect = useRef(effect)

  useEffect(() => {
    savedEffect.current = effect
  }, [effect])

  useEffect(() => {
    let active = true
    let timerId: number
    let cleanup: (() => void) | void

    async function runEffect() {
      if (!active) return

      const ret = await savedEffect.current()
      if (active) {
        cleanup = ret
        timerId = setTimeout(runEffect, timeout)
      }
    }

    timerId = setTimeout(runEffect, timeout)

    return () => {
      active = false
      clearTimeout(timerId)
      if (typeof cleanup === 'function') {
        cleanup()
      }
    }
  }, [timeout])
}
