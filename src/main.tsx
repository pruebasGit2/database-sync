import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import './tilde-bar.css'

import { appWindow } from '@tauri-apps/api/window'
document
  ?.getElementById('titlebar-minimize')
  ?.addEventListener('click', () => appWindow.minimize())
document
  ?.getElementById('titlebar-maximize')
  ?.addEventListener('click', () => appWindow.toggleMaximize())
document
  ?.getElementById('titlebar-close')
  ?.addEventListener('click', () => appWindow.close())

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
