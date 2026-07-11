import IonIcon from "@/components/ion-icon"
import Keybinds from "@/pages/key-binds/page"
import KeyboardSoftware from "@/pages/keyboard-software/page"
import ListItem from './list-item'
import { load, type Store } from "@tauri-apps/plugin-store"
import { useRef, useState } from "react"
import { useAsyncEffect } from "@/hooks/use-async-effect"
import PercDisplay from "@/pages/perc-display/page"
import AddCarTrack from '@/pages/add-car-track/page'

export type MethodOptions = '-unset-' | 'TC' | 'BB'

export type ItemEntry = {
  minSpeed: number,
  method: MethodOptions,
  value: number | null
}

export type Item = {
  track: {
    name: string
    car: {
      name: string
      entries: Record<string, ItemEntry[]>
    }
  }
}

export default function Index() {
  const store = useRef<Store>(null)
  const [items, setItems] = useState<Item[]>([])

  useAsyncEffect(async () => {
    store.current = await load('data.json', { autoSave: false, defaults: {} })
    const items = await store.current.get<Item[]>('tracks-cars')
    setItems(items || [])
  }, [])

  return (
    <div className='h-full overflow-y-scroll scrollbar scrollbar-track-stone-900 scrollbar-thumb-redorange'>
      <main className='min-h-screen w-full bg-stone-950 flex items-stretch flex-col'>
        <div className='fixed bottom-0 left-0 w-full h-[10vh] bg-stone-925 border-t border-white flex justify-between items-center'>
          <div className='aspect-[2] h-[80%] ml-2 flex justify-center items-center gap-1.5'>
            <button type='button' className='aspect-square h-full border border-white text-white bg-stone-950 rounded-md cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={() => window.setPage(<AddCarTrack />)}>
              <IonIcon name='add' variant='filled' strokeWidth={92} className='scale-220 mt-1' />
            </button>
            <button type='button' className='aspect-square h-full border border-white text-white bg-stone-950 rounded-md cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={() => window.setPage(<PercDisplay />)}>
              <IonIcon name='logo-laravel' variant='filled' className='scale-200 mt-1' />
            </button>
          </div>
          <div className='aspect-[2] h-[80%] mr-2 flex justify-center items-center gap-1.5'>
            <button type='button' className='aspect-square h-full border border-white text-white bg-stone-950 rounded-md cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={() => window.setPage(<KeyboardSoftware />)}>
              <IonIcon name='hardware-chip' variant='filled' className='scale-200 mt-1 ml-0.5' />
            </button>
            <button type='button' className='aspect-square h-full border border-white text-white bg-stone-950 rounded-md cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={() => window.setPage(<Keybinds />)}>
              <IonIcon name='keypad' variant='filled' className='scale-200 mt-2 ml-0.5' />
            </button>
          </div>
        </div>
        { items.map((v, i) => (
          <ListItem key={i} dataStore={store.current} dataItems={items}>{ v }</ListItem>
        )) }
      </main>
    </div>
  )
}
