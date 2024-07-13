import { useEffect } from "react"
import { Main } from "./pages/main/main"
import { Updater } from "./updater";

function App() {

  useEffect(() => {
    Updater();
  }, []);

  return <Main></Main>
}

export default App
