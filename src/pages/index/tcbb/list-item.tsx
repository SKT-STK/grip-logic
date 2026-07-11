import IonIcon from "@/components/ion-icon"
import { useState } from "react"
import Value from "./value"
import { ItemEntry, MethodOptions } from "@/pages/index/page"

type ListItemProps = {
  entry: Record<string, ItemEntry>
  allEntries: Record<string, ItemEntry[]>
  setEntries: (_: Record<string, ItemEntry[]>) => void
}

export default function ListItem({ entry, allEntries, setEntries }: ListItemProps) {
  const [key, value] = Object.entries(entry)[0]
  const [perc, setPerc] = useState(Number.parseFloat(Number.parseFloat(key).toFixed(1)))

  function deleteEntry() {
    const newData = {
      ...allEntries,
      [`${key}`]: allEntries[`${key}`].filter(v => JSON.stringify(v) !== JSON.stringify(value))
    }
    setEntries(newData)
  }

  function updatePerc(perc: number) {
    if (perc.toFixed(1) === key) {
      return
    }

    const fixedPerc = Number.parseFloat(perc.toFixed(1))
    setPerc(fixedPerc)

    const newValue = allEntries[`${perc.toFixed(1)}`] || []
    newValue.push(value)

    const newData = {
      ...allEntries,
      [`${perc.toFixed(1)}`]: newValue,
      [`${key}`]: allEntries[`${key}`].filter(v => JSON.stringify(v) !== JSON.stringify(value))
    }
    setEntries(newData)
  }

  function modifyValue(newValue: number | null, newMethod: MethodOptions) {
    if (newValue === value.value) {
      return
    }

    const newData = {
      ...allEntries,
      [`${key}`]: allEntries[`${key}`].map(v => v === value ? { ...v, value: newValue, method: newMethod } : v)
    }
    setEntries(newData)
  }

  function modifyMinSpeed(newSpeed: number) {
    if (newSpeed === value.minSpeed) {
      return
    }

    const newData = {
      ...allEntries,
      [`${key}`]: allEntries[`${key}`].map(v => v === value ? { ...v, minSpeed: newSpeed } : v)
    }
    setEntries(newData)
  }

  return (
    <div className='h-[15vh] px-20 flex justify-between items-center text-white text-xl font-semibold border-b hover:bg-stone-925 relative'>
      <div className="flex flex-row w-[40%] gap-2">
        <input type="number" defaultValue={perc} onBlur={e => updatePerc(parseFloat(e.target.value))} className="bg-stone-900 cursor-text outline-none w-1/2" />
        <input type="number" defaultValue={value.minSpeed} onBlur={e => modifyMinSpeed(parseInt(e.target.value))} className="bg-stone-900 w-1/2 cursor-text outline-none" />
      </div>
      <Value modifyValue={modifyValue}>{ value }</Value>
      <button type="button" className='absolute right-0 pr-5 cursor-pointer' onClick={deleteEntry}>
        <IonIcon name='trash' variant="filled" className='scale-150 text-red-500 hover:scale-175 duration-200' />
      </button>
    </div>
  )
}
