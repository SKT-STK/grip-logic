import { useState } from "react"
import Index from "./pages/index/page"

declare global {
  interface Window {
    setPage: (_: React.JSX.Element) => void
  }
}

export default function App() {
  const [page, setPage] = useState<React.JSX.Element>(<Index />)
  window.setPage = setPage

  return (
    <>{ page }</>
  )
}
