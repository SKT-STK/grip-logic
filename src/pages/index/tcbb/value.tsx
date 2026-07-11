import IonIcon from "@/components/ion-icon"
import { useAsyncEffect } from "@/hooks/use-async-effect"
import { load } from "@tauri-apps/plugin-store"
import { useState } from "react"
import { ItemEntry, MethodOptions } from "@/pages/index/page"

type ValueProps = {
  children: ItemEntry
  modifyValue: (_: number | null, __: MethodOptions) => void
}

export default function Value({ children, modifyValue }: ValueProps) {
  const [open, setOpen] = useState(false)
  const [listItems, setListItems] = useState<string[]>([])
  const [showBB, setShowBB] = useState(children.method === 'BB')

  useAsyncEffect(async () => {
    const settings = await load('settings.json', { autoSave: false, defaults: {} })
    const tcBinds = (await settings.get<(string | null)[]>('binds'))?.map((v, i) => {
      if (v === null) {
        return null
      }

      if (i < 10) return `TC ${i}`

      return null
    })

    if (tcBinds) {
      setListItems(['-unset-', ...tcBinds.filter(v => v !== null), 'BB'])
    }
  }, [])

  function selectNew(value: string, bb?: number | undefined) {
    setOpen(false)

    if (value.startsWith('TC') || value === '-unset-') {
      let newNumericValue: number | null = parseInt(value.split(' ')[1])
      if (isNaN(newNumericValue)) {
        newNumericValue = null
      }
      modifyValue(newNumericValue, value.split(' ')[0] as MethodOptions)
      setShowBB(false)
    }
    else if (value === 'BB') {
      if (bb !== undefined && !isNaN(bb)) {
        modifyValue(bb, 'BB')
      }
      setShowBB(true)
    }
    else {
      setShowBB(false)
    }
  }

  return (
    <div>
      { showBB &&
        <input type="number" defaultValue={isNaN(parseFloat(`${children.value}`)) ? 0 : children.value!} className='bg-stone-900 mr-5 text-right cursor-text outline-none' onBlur={e => selectNew('BB', parseFloat(parseFloat(e.target.value).toFixed(1)))} />
      }
      <button type='button' className="cursor-pointer" onClick={() => setOpen(o => !o)}>
        <div className="flex justify-center items-center gap-2">
          <p>{ children.method === 'TC' ? `${children.method} ${children.value}` : children.method }</p>
          <IonIcon name={open ? 'chevron-up' : 'chevron-down'} variant="filled" strokeWidth={64} />
        </div>
      </button>
      <div className='w-full h-0 relative'>
        { open && <div className='absolute bg-stone-900 rounded-md w-30 right-0 z-10 flex flex-col justify-center items-center gap-2.5'>
          { listItems.map((v, i) => (
            <div key={i} onClick={() => selectNew(v)} className="cursor-pointer hover:bg-stone-925 w-full text-center">{ v }</div>
          )) }
        </div> }
      </div>
    </div>
  )
}
