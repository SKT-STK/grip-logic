import IonIcon from "@/components/ion-icon"
import Index from "@/pages/index/page"
import { load, type Store } from '@tauri-apps/plugin-store'
import { useAsyncEffect } from "@/hooks/use-async-effect"
import { useRef, useState } from "react"
import ListItem from "./list-item"

export default function Keybinds() {
  const [binds, setBinds] = useState<(string | null)[]>([])
  const store = useRef<Store>(null)

  useAsyncEffect(async () => {
    store.current = await load('settings.json', { autoSave: false, defaults: {} })
    const binds = await store.current.get<(string | null)[]>('binds')
    setBinds(binds || [])
  })

  return (
    <div className='h-full overflow-y-scroll scrollbar scrollbar-track-stone-900 scrollbar-thumb-redorange scrollbar-buttons-none'>
      <main className='min-h-screen w-full bg-stone-950 flex items-stretch flex-col'>
        <button type="button" className='fixed top-0 left-0 text-white ml-2 mt-1 aspect-square rounded-full border-3 w-10 cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={() => window.setPage(<Index />)}>
          <IonIcon name='arrow-back' variant='outline' strokeWidth={64} className='scale-200 mt-1.5' />
        </button>

        <ListItem idx={0} bindsArr={binds} store={store.current}>TRACTION CONTROL 0</ListItem>
        <ListItem idx={1} bindsArr={binds} store={store.current}>TRACTION CONTROL 1</ListItem>
        <ListItem idx={2} bindsArr={binds} store={store.current}>TRACTION CONTROL 2</ListItem>
        <ListItem idx={3} bindsArr={binds} store={store.current}>TRACTION CONTROL 3</ListItem>
        <ListItem idx={4} bindsArr={binds} store={store.current}>TRACTION CONTROL 4</ListItem>
        <ListItem idx={5} bindsArr={binds} store={store.current}>TRACTION CONTROL 5</ListItem>
        <ListItem idx={6} bindsArr={binds} store={store.current}>TRACTION CONTROL 6</ListItem>
        <ListItem idx={7} bindsArr={binds} store={store.current}>TRACTION CONTROL 7</ListItem>
        <ListItem idx={8} bindsArr={binds} store={store.current}>TRACTION CONTROL 8</ListItem>
        <ListItem idx={9} bindsArr={binds} store={store.current}>TRACTION CONTROL 9</ListItem>
        <ListItem idx={10} bindsArr={binds} store={store.current}>BRAKE BIAS INCREASE</ListItem>
        <ListItem idx={11} bindsArr={binds} store={store.current}>BRAKE BIAS DECREASE</ListItem>
      </main>
    </div>
  )
}
