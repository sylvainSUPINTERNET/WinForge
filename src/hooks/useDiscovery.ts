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

export interface UseDiscoveryBackgroundServiceResult {
    state: DiscoveryBackgroundServiceState | null;
    loading: boolean;
    error: Error | null;
}

export function useDiscoveryBackgroundService(): UseDiscoveryBackgroundServiceResult {
    const [state, setState] =
        useState<DiscoveryBackgroundServiceState | null>(null);

    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<Error | null>(null);

    useEffect(() => {
        let unlisten: (() => void) | undefined;

        async function init() {
            try {
                const initial = await invoke<DiscoveryBackgroundServiceState>(
                    "discover_port_winforge_background_service"
                );

                setState(initial);

                unlisten = await listen<DiscoveryBackgroundServiceState>(
                    "discover_port_winforge_background_service",
                    (event) => {
                        setState(event.payload);
                    }
                );
            } catch (e) {
                setError(e as Error);
            } finally {
                setLoading(false);
            }
        }

        init();

        return () => {
            unlisten?.();
        };
    }, []);

    return {
        state,
        loading,
        error,
    };
}