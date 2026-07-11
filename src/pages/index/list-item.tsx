import { useCallback } from "react"
import { type Item } from "./page"
import { capitalize as capitalizeLib } from "@/lib/utils"
import TCBB from '@/pages/index/tcbb/page'
import { Store } from "@tauri-apps/plugin-store"

type ListItemProps = {
  children: Item
  dataItems: Item[]
  dataStore: Store | null
}

export default function ListItem({ children, dataItems, dataStore }: ListItemProps) {
  const capitalize = useCallback((text: string) => capitalizeLib(text, '_'), [])

  return (
    <div
      className="h-[15vh] px-20 flex justify-between items-center text-white text-xl
        font-semibold not-last:border-b hover:bg-stone-925 cursor-pointer
        hover:text-redorange duration-200 border-white"
      onClick={() => window.setPage(<TCBB item={children} dataItems={dataItems} dataStore={dataStore} />)}
    >
      <div>{ capitalize(children.track.name) }</div>
      <div>{ capitalize(children.track.car.name) }</div>
    </div>
  )
}
