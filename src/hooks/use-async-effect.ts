import { useEffect } from "react";

export function useAsyncEffect(effect: () => Promise<(() => (void | Promise<void>)) | void>, deps?: React.DependencyList) {
  useEffect(() => {
    let active = true
    let cleanup: (() => void) | void

    effect().then(ret => {
      if (active) {
        cleanup = ret
      }
    })

    return () => {
      active = false
      if (typeof cleanup === 'function') {
        cleanup()
      }
    }
  }, deps)
}
