import IonIcon from "@/components/ion-icon";
import { useAsyncInterval } from "@/hooks/use-async-interval";
import { dataStoreSaveAndNotify } from "@/lib/functions";
import Index, { Item } from "@/pages/index/page";
import { invoke } from "@tauri-apps/api/core";
import { load } from "@tauri-apps/plugin-store";
import { useState } from "react";

export default function AddCarTrack() {
  const [carName, setCarName] = useState('')
  const [trackName, setTrackName] = useState('')

  useAsyncInterval(async () => {
    const carAndTrack = await invoke<[string, string] | null>('get_car_and_track_name')
    carAndTrack !== null && setCarName(carAndTrack[0])
    carAndTrack !== null && setTrackName(carAndTrack[1])
    const offset = await invoke('fetch_bb_offset', { carName: (carAndTrack || [''])[0] })
    console.log(offset === null ? 0 : offset)
  }, 1000)

  async function addNewCarTrack() {
    const store = await load('data.json', { autoSave: false, defaults: {} })
    const modifyStore = await store.get<Item[]>('tracks-cars')
    modifyStore?.push({ track: { name: trackName, car: { name: carName, entries: {} } } } as Item)
    await store.set('tracks-cars', modifyStore)
    // await store.save()
    await dataStoreSaveAndNotify(store)
    window.setPage(<Index />)
  }

  return (
    <main className='min-h-screen w-full bg-stone-950 flex items-center justify-center text-center flex-col'>
      <h1 className='text-white text-5xl'>Car Name: <strong>{carName}</strong></h1>
      <h1 className='text-white text-5xl'>Track Name: <strong>{trackName}</strong></h1>
      <button
        type="button"
        className="flex justify-center items-center w-1/2 h-15 outline-none
          border-2 border-white text-white duration-200 rounded-md mt-3
          cursor-pointer hover:border-redorange hover:text-redorange"
        onClick={addNewCarTrack}
      >
        <IonIcon name='add' variant='filled' strokeWidth={92} className='scale-220' />
      </button>

      <button type="button" className='fixed top-0 left-0 text-white ml-2 mt-1 aspect-square rounded-full border-3 w-10 cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={() => window.setPage(<Index />)}>
        <IonIcon name='arrow-back' variant='outline' strokeWidth={64} className='scale-200 mt-1.5' />
      </button>
    </main>
  )
}
