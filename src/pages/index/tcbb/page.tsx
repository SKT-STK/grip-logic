import IonIcon from "@/components/ion-icon"
import Index, { Item } from "../page"
import { useCallback, useState } from "react"
import { capitalize as capitalizeLib } from '@/lib/utils'
import ListItem from "./list-item"
import { Store } from "@tauri-apps/plugin-store"
import { useAsyncEffect } from "@/hooks/use-async-effect"
import { type ItemEntry } from "@/pages/index/page"
import { dataStoreSaveAndNotify } from "@/lib/functions"

type TCBBProps = {
  item: Item
  dataItems: Item[]
  dataStore: Store | null
}

export default function TCBB({ item, dataItems, dataStore }: TCBBProps) {
  const capitalize = useCallback((text: string) => capitalizeLib(text, '_'), [])
  const [entries, setEntries] = useState<Record<string, ItemEntry[]>>(item.track.car.entries)

  useAsyncEffect(async () => {
    const newData = dataItems.map(item_ => {
      if (item.track.name === item_.track.name && item.track.car.name === item_.track.car.name) {
        return {
          ...item_,
          track: {
            ...item_.track,
            car: {
              ...item_.track.car,
              entries
            }
          }
        } as Item
      }
      else {
        return item_
      }
    })
    await dataStore?.set('tracks-cars', newData)
    // await dataStore?.save()
    await dataStoreSaveAndNotify(dataStore)
  }, [entries, dataStore])

  function addUnset() {
    let newData = entries['0'] || []
    newData.push({ minSpeed: 0, method: '-unset-', value: null })
    const newEntries = {...entries}
    newEntries['0'] = newData
    setEntries(newEntries)
  }

  async function deleteThis() {
    const newData = dataItems.filter(item_ => item.track.name !== item_.track.name || item.track.car.name !== item_.track.car.name)
    await dataStore?.set('tracks-cars', newData)
    // await dataStore?.save()
    await dataStoreSaveAndNotify(dataStore)

    window.setPage(<Index />)
  }

  return (
    <div className='h-full overflow-y-scroll scrollbar scrollbar-track-stone-900 scrollbar-thumb-redorange scrollbar-buttons-none'>
      <main className='min-h-screen w-full bg-stone-950 flex items-stretch flex-col'>
        <button type="button" className='fixed top-0 left-0 text-white ml-2 mt-1 aspect-square rounded-full border-3 w-10 cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={() => window.setPage(<Index />)}>
          <IonIcon name='arrow-back' variant='outline' strokeWidth={64} className='scale-200 mt-1.5' />
        </button>
        <button type="button" className='fixed top-0 right-4 text-white ml-2 mt-1 aspect-square rounded-full border-3 w-10 cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={deleteThis}>
          <IonIcon name='trash' variant='filled' className='scale-150 mt-1.5' />
        </button>
        <div className="h-[15vh] flex justify-around items-center text-white text-3xl font-semibold border-b">
          <p>{ capitalize(item.track.name) }</p>
          <p>{ capitalize(item.track.car.name) }</p>
        </div>

        { Object.entries(entries).map(([k, values], i) => values.map((v, ii) => (
          <ListItem key={`${i}--${ii}`} entry={{ [`${k}`]: v }} allEntries={entries} setEntries={setEntries} />
        ))) }
        <button type='button' className='h-[15vh] flex justify-center items-center text-white hover:bg-stone-925 hover:text-redorange duration-200 cursor-pointer' onClick={addUnset}>
          <IonIcon name='add' variant='filled' strokeWidth={64} className='scale-300' />
        </button>
      </main>
    </div>
  )
}
