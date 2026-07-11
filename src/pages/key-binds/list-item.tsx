import IonIcon from "@/components/ion-icon"
import { Store } from "@tauri-apps/plugin-store"
import { useCallback, useEffect, useState } from "react"
import { motion, AnimatePresence, useAnimation } from 'framer-motion'

type ListItemProps = {
  children: string
  idx: number
  bindsArr: (string | null)[]
  store: Store | null
}

export default function ListItem({ children, idx, bindsArr, store }: ListItemProps) {
  const [isHovering, setIsHovering] = useState(false)
  const motionButtonControls = useAnimation()
  const [isListening, setIsListening] = useState(false)
  const setBindsInStore = useCallback(async (binds: (string | null)[]) => {
    await store?.set('binds', binds)
    await store?.save()
  }, [bindsArr, idx, store])
  const keyPressCallback = useCallback((e: KeyboardEvent) => {
    if (e.code === 'Escape') {
      setIsListening(false)
      return
    }

    const binds = [...bindsArr]
    binds[idx] = e.code
    setBindsInStore(binds)
      .then(() => setIsListening(false))
  }, [setBindsInStore])

  useEffect(() => {
    if (isListening) {
      window.addEventListener('keydown', keyPressCallback, { once: true })
    }

    return () => {
      window.removeEventListener('keydown', keyPressCallback)
    }
  }, [isListening, keyPressCallback])

  useEffect(() => {
    isHovering ? 
      motionButtonControls.start({
        width: 'auto',
        opacity: 1,
      })
    :
      motionButtonControls.start({
        width: 0,
        opacity: 0
      })
  }, [isHovering])
  
  function deleteBind(e: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    e.stopPropagation()
    const binds = bindsArr.map((v, i) => i === idx ? null : v)
    setBindsInStore(binds)
  }

  return (
    <div
      className='h-[15vh] px-20 flex justify-between items-center text-white text-xl font-semibold not-last:border-b hover:bg-stone-925'
      onMouseOver={() => setIsHovering(true)}
      onMouseOut={() => setIsHovering(false)}
    >
      <p>{ children }</p>
      <button type="button" className='cursor-pointer hover:text-redorange duration-200 outline-none' onClick={() => setIsListening(true)}>
        {
          isListening ?
            <IonIcon key={`listening--${idx}`} name='ellipsis-horizontal' variant="outline" className="scale-200 -translate-x-1/2 text-stone-900 hover:text-stone-900 duration-0" />
          :
            ( bindsArr[idx] ? 
              <AnimatePresence>
                <div className='flex'>
                  <motion.p layout className='text-white text-xl font-semibold px-2 hover:text-redorange duration-200'>{ bindsArr[idx] }</motion.p>
                  <motion.button type="button" onClick={deleteBind} className='border-l border-white'
                    initial={{
                      width: 0,
                      opacity: 0
                    }}
                    animate={motionButtonControls}
                    transition={{
                      duration: .2
                    }}
                  >
                    <IonIcon name="trash" variant="filled" className='text-red-500 pl-2 scale-150 hover:scale-175 duration-200 cursor-pointer' />
                  </motion.button>
                </div>
              </AnimatePresence>
            :
              <IonIcon key={`empty--${idx}`} name='ellipsis-horizontal' variant="filled" className="scale-200 -translate-x-1/2" /> )
        }
      </button>
    </div>
  )
}
