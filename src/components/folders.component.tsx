import { useEffect, useMemo, useState } from "react";
import { createPortal } from "react-dom";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { getFolders, updateFolderPrompt } from "../api/folders";
import { DiscoveryBackgroundServiceState } from "../hooks/useDiscovery";

interface Folder {
  id: number;
  uid: string;
  resource_path: string;
  prompt: string | null;
  created_at: string;
}

interface FoldersProps {
  discoveryBackgroundServiceState: DiscoveryBackgroundServiceState;
}

const PAGE_SIZES = [6, 12, 24];

function FolderIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path d="M3.75 6.75A2.25 2.25 0 0 1 6 4.5h3.15c.6 0 1.17.24 1.59.66l1.1 1.09H18A2.25 2.25 0 0 1 20.25 8.5v7.75A2.25 2.25 0 0 1 18 18.5H6a2.25 2.25 0 0 1-2.25-2.25v-9.5Z" />
    </svg>
  );
}

function ChevronIcon({ direction }: { direction: "left" | "right" }) {
  return (
    <svg
      viewBox="0 0 20 20"
      aria-hidden="true"
      className={direction === "right" ? "chevron-right" : undefined}
    >
      <path d="m12.5 15-5-5 5-5" />
    </svg>
  );
}

function RefreshIcon() {
  return (
    <svg viewBox="0 0 20 20" aria-hidden="true">
      <path d="M15.3 6.1A6.5 6.5 0 1 0 16.5 10M15.3 6.1V2.8m0 3.3H12" />
    </svg>
  );
}

function CopyIcon({ copied }: { copied: boolean }) {
  return (
    <svg viewBox="0 0 20 20" aria-hidden="true">
      {copied ? (
        <path d="m4.5 10.2 3.4 3.4 7.6-7.7" />
      ) : (
        <>
          <rect x="6.5" y="6.5" width="9" height="9" rx="2" />
          <path d="M13.5 6.5v-2a2 2 0 0 0-2-2h-7a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h2" />
        </>
      )}
    </svg>
  );
}

function ExplorerIcon() {
  return (
    <svg viewBox="0 0 20 20" aria-hidden="true">
      <path d="M3 6.5A1.5 1.5 0 0 1 4.5 5h3l1.4 1.5h6.6A1.5 1.5 0 0 1 17 8v6.5a1.5 1.5 0 0 1-1.5 1.5h-11A1.5 1.5 0 0 1 3 14.5v-8Z" />
      <path d="m11 9 2 2-2 2m2-2H8" />
    </svg>
  );
}

function EditIcon() {
  return (
    <svg viewBox="0 0 20 20" aria-hidden="true">
      <path d="m12.8 4.2 3 3M4 16l.8-3.8L13.6 3.4a1.4 1.4 0 0 1 2 0l1 1a1.4 1.4 0 0 1 0 2l-8.8 8.8L4 16Z" />
    </svg>
  );
}

function CloseIcon() {
  return (
    <svg viewBox="0 0 20 20" aria-hidden="true">
      <path d="m5 5 10 10M15 5 5 15" />
    </svg>
  );
}

function formatDate(value: string) {
  const date = new Date(value);

  if (Number.isNaN(date.getTime())) return value;

  return new Intl.DateTimeFormat("fr-FR", {
    day: "2-digit",
    month: "short",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
}

function getFolderName(path: string) {
  const parts = path.replace(/[\\/]+$/, "").split(/[\\/]/);
  return parts[parts.length - 1] || path;
}

function Folders({ discoveryBackgroundServiceState }: FoldersProps) {
  const queryClient = useQueryClient();
  const port = (discoveryBackgroundServiceState.message as { port?: number } | null)?.port;
  const [pageSize, setPageSize] = useState(PAGE_SIZES[0]);
  const [cursorHistory, setCursorHistory] = useState([0]);
  const [copiedFolderId, setCopiedFolderId] = useState<number | null>(null);
  const [openingFolderId, setOpeningFolderId] = useState<number | null>(null);
  const [openErrorFolderId, setOpenErrorFolderId] = useState<number | null>(null);
  const [editedFolder, setEditedFolder] = useState<Folder | null>(null);
  const [promptDraft, setPromptDraft] = useState("");
  const pageIndex = cursorHistory.length - 1;
  const currentCursor = cursorHistory[pageIndex];

  useEffect(() => {
    setCursorHistory([0]);
  }, [pageSize, port]);

  const {
    data: response = [],
    isPending,
    isFetching,
    isError,
    error,
    refetch,
  } = useQuery<Folder[]>({
    queryKey: ["background-service", "folders", currentCursor, pageSize, port],
    queryFn: () => getFolders({ last_id: currentCursor, per_page: pageSize + 1 }, port as number),
    enabled: typeof port === "number",
    placeholderData: (previousData) => previousData,
    staleTime: 0,
    gcTime: 0,
    retry: 1,
  });

  const folders = useMemo(() => response.slice(0, pageSize), [response, pageSize]);
  const hasNextPage = response.length > pageSize;

  const promptMutation = useMutation({
    mutationFn: ({ folderId, prompt }: { folderId: number; prompt: string | null }) =>
      updateFolderPrompt(folderId, prompt, port as number),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ["background-service", "folders"] });
      setEditedFolder(null);
    },
  });

  useEffect(() => {
    if (!editedFolder) return;

    const closeOnEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape" && !promptMutation.isPending) setEditedFolder(null);
    };

    document.addEventListener("keydown", closeOnEscape);
    return () => document.removeEventListener("keydown", closeOnEscape);
  }, [editedFolder, promptMutation.isPending]);

  const goToNextPage = () => {
    const lastFolder = folders[folders.length - 1];
    if (!lastFolder || !hasNextPage) return;
    setCursorHistory((history) => [...history, lastFolder.id]);
  };

  const goToPreviousPage = () => {
    setCursorHistory((history) => (history.length > 1 ? history.slice(0, -1) : history));
  };

  const copyPath = async (folder: Folder) => {
    try {
      await navigator.clipboard.writeText(folder.resource_path);
      setCopiedFolderId(folder.id);
      window.setTimeout(() => {
        setCopiedFolderId((currentId) => (currentId === folder.id ? null : currentId));
      }, 1800);
    } catch {
      setCopiedFolderId(null);
    }
  };

  const openInExplorer = async (folder: Folder) => {
    setOpeningFolderId(folder.id);
    setOpenErrorFolderId(null);

    try {
      await invoke("open_folder_in_explorer", { path: folder.resource_path });
    } catch {
      setOpenErrorFolderId(folder.id);
    } finally {
      setOpeningFolderId(null);
    }
  };

  const openPromptEditor = (folder: Folder) => {
    promptMutation.reset();
    setPromptDraft(folder.prompt ?? "");
    setEditedFolder(folder);
  };

  const savePrompt = (event: React.FormEvent) => {
    event.preventDefault();
    if (!editedFolder) return;
    const prompt = promptDraft.trim();
    promptMutation.mutate({ folderId: editedFolder.id, prompt: prompt || null });
  };

  const content = (() => {
    if (isPending) {
      return (
        <ul className="folders-grid" aria-label="Chargement des dossiers" aria-busy="true">
          {Array.from({ length: pageSize }).map((_, index) => (
            <li className="folder-card folder-card-skeleton" key={index}>
              <span className="skeleton skeleton-icon" />
              <span className="skeleton skeleton-title" />
              <span className="skeleton skeleton-line" />
              <span className="skeleton skeleton-line skeleton-line-short" />
            </li>
          ))}
        </ul>
      );
    }

    if (isError) {
      return (
        <div className="folders-state folders-state-error" role="alert">
          <span className="state-icon">!</span>
          <h2>Impossible de charger les dossiers</h2>
          <p>{error instanceof Error ? error.message : "Une erreur inattendue est survenue."}</p>
          <button className="primary-button" type="button" onClick={() => refetch()}>
            Réessayer
          </button>
        </div>
      );
    }

    if (folders.length === 0) {
      return (
        <div className="folders-state">
          <span className="empty-folder-icon"><FolderIcon /></span>
          <h2>{pageIndex === 0 ? "Aucun dossier surveillé" : "Vous êtes arrivé au bout"}</h2>
          <p>
            {pageIndex === 0
              ? "Les dossiers ajoutés à WinForge apparaîtront ici."
              : "Revenez à la page précédente pour continuer la navigation."}
          </p>
          {pageIndex > 0 && (
            <button className="primary-button" type="button" onClick={goToPreviousPage}>
              Page précédente
            </button>
          )}
        </div>
      );
    }

    return (
      <ul className="folders-grid">
        {folders.map((folder) => (
          <li className="folder-card" key={folder.id}>
            <div className="folder-card-topline">
              <span className="folder-icon"><FolderIcon /></span>
              <span className="folder-status"><i /> Surveillé</span>
            </div>

            <div className="folder-card-copy">
              <h2 title={folder.resource_path}>{getFolderName(folder.resource_path)}</h2>
              <div className="folder-path-row">
                <p className="folder-path" title={folder.resource_path}>{folder.resource_path}</p>
                <div className="folder-path-actions">
                  <button
                    className={`path-action-button${copiedFolderId === folder.id ? " is-copied" : ""}`}
                    type="button"
                    onClick={() => copyPath(folder)}
                    aria-label={copiedFolderId === folder.id ? "Chemin copié" : "Copier le chemin"}
                    title={copiedFolderId === folder.id ? "Copié" : "Copier le chemin"}
                  >
                    <CopyIcon copied={copiedFolderId === folder.id} />
                    <span>{copiedFolderId === folder.id ? "Copié" : "Copier"}</span>
                  </button>
                  <button
                    className={`path-action-button${openErrorFolderId === folder.id ? " has-error" : ""}`}
                    type="button"
                    onClick={() => openInExplorer(folder)}
                    disabled={openingFolderId === folder.id}
                    aria-label="Ouvrir dans l’Explorateur Windows"
                    title={openErrorFolderId === folder.id ? "Impossible d’ouvrir ce dossier" : "Ouvrir dans l’Explorateur"}
                  >
                    <ExplorerIcon />
                    <span>{openErrorFolderId === folder.id ? "Erreur" : "Ouvrir"}</span>
                  </button>
                </div>
              </div>
            </div>

            <div className="folder-prompt-block">
              <div className="folder-prompt-heading">
                <span>Prompt LLM</span>
                <button type="button" onClick={() => openPromptEditor(folder)}>
                  <EditIcon />
                  {folder.prompt ? "Modifier" : "Ajouter"}
                </button>
              </div>
              <p className={folder.prompt ? "folder-prompt" : "folder-prompt is-empty"} title={folder.prompt ?? undefined}>
                {folder.prompt ?? "Aucune instruction personnalisée"}
              </p>
            </div>

            <div className="folder-meta">
              <time dateTime={folder.created_at}>{formatDate(folder.created_at)}</time>
              <span className="folder-uid" title={`Identifiant : ${folder.uid}`}>
                #{folder.uid.slice(0, 8)}
              </span>
            </div>
          </li>
        ))}
      </ul>
    );
  })();

  return (
    <section className="folders-panel">
      <header className="folders-header">
        <div>
          <span className="eyebrow">Espace de travail</span>
          <h1>Vos dossiers</h1>
          <p>Retrouvez les dossiers surveillés par WinForge.</p>
        </div>
        <button
          className="icon-button"
          type="button"
          onClick={() => refetch()}
          disabled={isFetching}
          aria-label="Actualiser les dossiers"
          title="Actualiser"
        >
          <RefreshIcon />
        </button>
      </header>

      {content}

      {!isPending && !isError && folders.length > 0 && (
        <nav className="pagination" aria-label="Pagination des dossiers">
          <div className="pagination-size">
            <label htmlFor="folders-page-size">Afficher</label>
            <select
              id="folders-page-size"
              value={pageSize}
              onChange={(event) => setPageSize(Number(event.target.value))}
            >
              {PAGE_SIZES.map((size) => <option value={size} key={size}>{size}</option>)}
            </select>
            <span>par page</span>
          </div>

          <div className="pagination-controls">
            <button
              type="button"
              onClick={goToPreviousPage}
              disabled={pageIndex === 0 || isFetching}
              aria-label="Page précédente"
            >
              <ChevronIcon direction="left" />
              <span>Précédent</span>
            </button>
            <span className="page-number" aria-current="page">{pageIndex + 1}</span>
            <button
              type="button"
              onClick={goToNextPage}
              disabled={!hasNextPage || isFetching}
              aria-label="Page suivante"
            >
              <span>Suivant</span>
              <ChevronIcon direction="right" />
            </button>
          </div>
        </nav>
      )}

      {editedFolder && createPortal(
        <div className="prompt-modal-backdrop" role="presentation" onMouseDown={() => !promptMutation.isPending && setEditedFolder(null)}>
          <div
            className="prompt-modal"
            role="dialog"
            aria-modal="true"
            aria-labelledby="prompt-modal-title"
            onMouseDown={(event) => event.stopPropagation()}
          >
            <div className="prompt-modal-header">
              <div>
                <span className="eyebrow">Instructions du dossier</span>
                <h2 id="prompt-modal-title">Prompt LLM</h2>
                <p title={editedFolder.resource_path}>{getFolderName(editedFolder.resource_path)}</p>
              </div>
              <button
                className="modal-close-button"
                type="button"
                onClick={() => setEditedFolder(null)}
                disabled={promptMutation.isPending}
                aria-label="Fermer"
              >
                <CloseIcon />
              </button>
            </div>

            <form onSubmit={savePrompt}>
              <label htmlFor="folder-prompt">Instructions à appliquer aux fichiers de ce dossier</label>
              <textarea
                id="folder-prompt"
                value={promptDraft}
                onChange={(event) => setPromptDraft(event.target.value)}
                placeholder="Ex. Résume chaque document en français, relève les points importants et conserve les termes techniques…"
                rows={9}
                autoFocus
              />
              <div className="prompt-editor-meta">
                <span>Ce prompt sera utilisé pour tous les fichiers de ce dossier.</span>
              </div>

              {promptMutation.isError && (
                <p className="prompt-save-error" role="alert">
                  {promptMutation.error instanceof Error ? promptMutation.error.message : "La sauvegarde a échoué."}
                </p>
              )}

              <div className="prompt-modal-actions">
                {editedFolder.prompt && (
                  <button
                    className="delete-prompt-button"
                    type="button"
                    onClick={() => promptMutation.mutate({ folderId: editedFolder.id, prompt: null })}
                    disabled={promptMutation.isPending}
                  >
                    Supprimer le prompt
                  </button>
                )}
                <span />
                <button className="secondary-button" type="button" onClick={() => setEditedFolder(null)} disabled={promptMutation.isPending}>
                  Annuler
                </button>
                <button className="save-prompt-button" type="submit" disabled={promptMutation.isPending}>
                  {promptMutation.isPending ? "Enregistrement…" : "Enregistrer"}
                </button>
              </div>
            </form>
          </div>
        </div>,
        document.body,
      )}
    </section>
  );
}

export default Folders;
