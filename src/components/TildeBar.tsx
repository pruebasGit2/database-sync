import { appWindow } from '@tauri-apps/api/window'

export const TildeBar = () => {
    return (
        <div
            data-tauri-drag-region 
            className="w-screen h-7 bg-primary select-none flex justify-end fixed t-0 l-0 r-0"
        >
            <div className='text-sm flex items-center mr-auto ml-2'>
                Database sync
            </div>
            <div
                className="inline-flex justify-center items-center w-7 h-7 hover:cursor-pointer hover:bg-gray-300"
                onClick={() => appWindow.minimize()}
            >
                <img
                    src="https://api.iconify.design/mdi:window-minimize.svg"
                    alt="minimize"
                />
            </div>
            <div
                className="inline-flex justify-center items-center w-7 h-7 hover:cursor-pointer hover:bg-red-700"
                onClick={() => appWindow.close()}
            >
                <img
                    src="https://api.iconify.design/mdi:close.svg"
                    alt="close"
                />
            </div>
        </div>
    )
}