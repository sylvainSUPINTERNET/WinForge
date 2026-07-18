import { DiscoveryBackgroundServiceState } from "../hooks/useDiscovery";

function Folders({
        discoveryBackgroundServiceState
    }:{
        discoveryBackgroundServiceState: DiscoveryBackgroundServiceState
    }) {


    return  ( 
        <div>
            {
                discoveryBackgroundServiceState.message && discoveryBackgroundServiceState.message !== null && (
                    <div>
                        {
                            JSON.stringify(discoveryBackgroundServiceState.message)
                        }
                    </div>
                )
            }
        </div>
    )
}

export default Folders;

