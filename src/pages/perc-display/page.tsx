import IonIcon from "@/components/ion-icon";
import { useAsyncInterval } from "@/hooks/use-async-interval";
import Index from "@/pages/index/page";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

export default function PercDisplay() {
  const [trackPerc, setTrackPerc] = useState('0')

  useAsyncInterval(async () => {
    const perc = await invoke<number | null>('get_curr_track_perc')
    perc !== null && setTrackPerc((perc * 100).toFixed(1))
  }, 33)

  return (
    <main className='min-h-screen w-full bg-stone-950 flex items-center justify-center text-center flex-col'>
      <h1 className='text-white text-5xl'>{ `Track %: ${trackPerc}` }</h1>

      <button type="button" className='fixed top-0 left-0 text-white ml-2 mt-1 aspect-square rounded-full border-3 w-10 cursor-pointer hover:border-redorange hover:text-redorange duration-200' onClick={() => window.setPage(<Index />)}>
        <IonIcon name='arrow-back' variant='outline' strokeWidth={64} className='scale-200 mt-1.5' />
      </button>
    </main>
  )
}
