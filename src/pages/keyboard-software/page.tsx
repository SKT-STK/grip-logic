import IonIcon from "@/components/ion-icon"
import Index from "@/pages/index/page"
import { load, type Store } from '@tauri-apps/plugin-store'
import { useRef, useState } from "react"
import { useAsyncEffect } from "@/hooks/use-async-effect"

type KbSoftOptions = 'ghub' | 'razer' | undefined

const REDORANGE = 'oklch(0.6845 0.2095 41.2)'

export default function KeyboardSoftware() {
  const [kbSoft, setKbSoft] = useState<KbSoftOptions>(undefined)
  const store = useRef<Store>(null)

  useAsyncEffect(async () => {
    store.current = await load('settings.json', { autoSave: false, defaults: {} })
    const software = await store.current.get<KbSoftOptions>('keyboard-software')
    setKbSoft(software)
  }, [])

  useAsyncEffect(async () => {
    await store.current?.set('keyboard-software', kbSoft)
    store.current?.save()
  }, [kbSoft])

  return (
    <main className='min-h-screen w-full bg-stone-950 flex items-center justify-center text-center flex-col'>
      <h1 className='text-white text-center text-2xl font-semibold'>Pick whichever you have installed.</h1>
      <h1 className='text-white text-center text-2xl font-semibold'>If you do not have either of them installed, do so.</h1>
      <br />
      <button
        type="button"
        className='text-white text-center text-xl font-bold border-2 rounded-t-md
          border-b py-3 w-[30%] hover:bg-stone-900 cursor-pointer'
        style={{
          borderTopColor: kbSoft === 'ghub' ? REDORANGE : 'white',
          borderLeftColor: kbSoft === 'ghub' ? REDORANGE : 'white',
          borderRightColor: kbSoft === 'ghub' ? REDORANGE : 'white',
          borderBottomColor: kbSoft === undefined ? 'white' : REDORANGE
        }}
        onClick={() => setKbSoft('ghub')}
      >
        Logitech G-Hub
      </button>
      <button
        type="button"
        className='text-white text-center text-xl font-bold border-2 rounded-b-md
          border-t py-3 w-[30%] hover:bg-stone-900 cursor-pointer'
        style={{
          borderBottomColor: kbSoft === 'razer' ? REDORANGE : 'white',
          borderLeftColor: kbSoft === 'razer' ? REDORANGE : 'white',
          borderRightColor: kbSoft === 'razer' ? REDORANGE : 'white',
          borderTopColor: kbSoft === undefined ? 'white' : REDORANGE
        }}
        onClick={() => setKbSoft('razer')}
      >
        Razer Synapse
      </button>
      <button type="button" className='fixed top-0 left-0 text-white ml-2 mt-1 aspect-square rounded-full border-3 w-10 cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={() => window.setPage(<Index />)}>
        <IonIcon name='arrow-back' variant='outline' strokeWidth={64} className='scale-200 mt-1.5' />
      </button>
    </main>
  )
}
