import {
    checkUpdate,
    installUpdate,
    onUpdaterEvent,
} from '@tauri-apps/api/updater'
import toast from 'react-hot-toast';
import { Button } from './pages/main/main';
import { relaunch } from '@tauri-apps/api/process'

export async function Updater() {
    //const unlisten = await onUpdaterEvent(({ error, status }) => {
    //    // This will log all updater events, including status updates and errors.
    //    console.log('Updater event', error, status)
    //})

    try {
        const { shouldUpdate, manifest } = await checkUpdate()

        if (shouldUpdate) {
            // You could show a dialog asking the user if they want to install the update here.
            toast((t) => (
                <span>
                    New version available <b>{manifest?.version}</b>
                    <Button onClick={async () => {
                        await installUpdate()
                        await relaunch()
                        toast.dismiss(t.id);
                    }}>
                        Update
                    </Button>
                </span>
            ), {
                duration: 999
            });

            // Install the update. This will also restart the app on Windows!

            // On macOS and Linux you will need to restart the app manually.
            // You could use this step to display another confirmation dialog.
        }
    } catch (error) {
        console.error(error)
    }

    // you need to call unlisten if your handler goes out of scope, for example if the component is unmounted.
    //unlisten()
}
