import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export interface DiscoveryBackgroundServiceState {
    code: number;
    message: Object | {
        port: number;
        pid: number;
    }
}

export function useDiscoveryBackgroundService() {
    const [state, setState] = useState<DiscoveryBackgroundServiceState | null>(null);

    useEffect(() => {
        let unlisten: (() => void) | undefined;

        async function init() {

            const initial = await invoke<DiscoveryBackgroundServiceState>(
                "discover_port_winforge_background_service"
            );

            setState(initial);

            // Mises à jour
            unlisten = await listen<DiscoveryBackgroundServiceState>(
                "discover_port_winforge_background_service",
                event => {
                    console.log("discover_port_winforge_background_service", event.payload);
                    setState(event.payload);
                }
            );
        }

        init();

        return () => {
            unlisten?.();
        };
    }, []);

    return state;
}