import { useQuery } from "@tanstack/react-query";
import { DiscoveryBackgroundServiceState } from "../hooks/useDiscovery";
import { getFolders } from "../api/folders";
import { Pagination } from "../types/api/Pagination";

interface Folder {
    id: number;
    uid: string;
    resource_path: string;
    prompt: string | null;
    created_at: string;
}

function Folders({
        discoveryBackgroundServiceState: _discoveryBackgroundServiceState
    }:{
        discoveryBackgroundServiceState: DiscoveryBackgroundServiceState
    }) {


    const pagination:Pagination = {
        last_id: 0,
        per_page: 20
    }



    const {
        data: folders,
        isPending,
        isError,
        error,
    } = useQuery<Folder[]>({
        queryKey: ["background-service", pagination, (_discoveryBackgroundServiceState.message as any)?.port],
        queryFn: () => getFolders(pagination, (_discoveryBackgroundServiceState.message as any)?.port),

        // Pas de cache "utile"
        staleTime: 0,
        gcTime: 0,

        // Ne jamais relancer tout seul
        // refetchOnWindowFocus: false,
        // refetchOnR   econnect: false,
        // refetchOnMount: true,
        // retry: false,
    });
    if (isPending) {
        return <p>Chargement des dossiers...</p>;
    }

    if (isError) {
        return <p>Impossible de charger les dossiers : {error.message}</p>;
    }

    if (folders.length === 0) {
        return <p>Aucun dossier trouvé.</p>;
    }

    return (
        <section>
            <h1>Dossiers</h1>

            <ul>
                {folders.map((folder) => (
                    <li key={folder.id}>
                        <h2>{folder.resource_path}</h2>
                        <p>Identifiant : {folder.uid}</p>
                        {folder.prompt && <p>Prompt : {folder.prompt}</p>}
                        <p>Créé le : {folder.created_at}</p>
                    </li>
                ))}
            </ul>
        </section>
    );
}

export default Folders;

