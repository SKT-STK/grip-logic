import React from "react"
import ReactDOM from "react-dom/client"
import App from "@/App"
import '@/styles/.css'
import SetWindowVisible from "./components/set-window-visible"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
    <SetWindowVisible />
  </React.StrictMode>
)
