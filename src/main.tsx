import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import { TildeBar } from './components/TildeBar.tsx'
import { Toaster } from 'react-hot-toast'
import "./utils/promiseExtension.ts"

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <Toaster/>
    <TildeBar/>
    <App />
  </React.StrictMode>,
)
