import { Pagination } from "../types/api/Pagination";
import { getApiUrlWindowsBackgroundService } from "./config";

export const getFolders = async (pagination:Pagination, port:number) => {
    const resp = await fetch(`${getApiUrlWindowsBackgroundService(port)}/folders?last_id=${pagination.last_id}&per_page=${pagination.per_page}`);
    if (!resp.ok) {
        throw new Error(`HTTP error! status: ${resp.status}`);
    }
    console.log(resp);
    return await resp.json();
}
