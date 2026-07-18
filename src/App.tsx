import Folders from "./components/folders.component";
import { useDiscoveryBackgroundService } from "./hooks/useDiscovery";

function App() {
  const {
    state,
    loading,
    error
  } = useDiscoveryBackgroundService()

  return (
    <main className="container">

      <div>
        {
          loading && (
            <div>
              Loading...
            </div>
          )
        }

        {
          error && (
            <div>
              Error: {error.message}
            </div>
          )
        }

        {
          !error && !loading && state && state != null && state.code === 200 && state.message && state.message !== null && (
              <div>
                <Folders discoveryBackgroundServiceState={state}></Folders>
              </div>
          )
        }
  
      </div>

    </main>
  )
}

export default App;
