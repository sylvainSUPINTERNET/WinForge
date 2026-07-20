import { Pagination } from "../types/api/Pagination";
import { getApiUrlWindowsBackgroundService } from "./config";

export const getFolders = async (pagination: Pagination, port: number) => {
    const resp = await fetch(`${getApiUrlWindowsBackgroundService(port)}/folders?last_id=${pagination.last_id}&per_page=${pagination.per_page}`);
    if (!resp.ok) {
        throw new Error(`HTTP error! status: ${resp.status}`);
    }
    return await resp.json();
}

export const updateFolderPrompt = async (
    folderId: number,
    prompt: string | null,
    port: number,
) => {
    const resp = await fetch(`${getApiUrlWindowsBackgroundService(port)}/folders/${folderId}/prompt`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ prompt }),
    });

    if (!resp.ok) {
        throw new Error(`Impossible d’enregistrer le prompt (${resp.status})`);
    }

    return await resp.json() as { prompt: string | null };
};
