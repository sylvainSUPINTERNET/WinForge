import { create } from "zustand";
import { getJobId, JobEvent } from "../types/JobEvent";

interface PdfToJpegStore {
  jobs: Record<string, JobEvent>;

  setJob: (job: JobEvent) => void;
  removeJob: (job: JobEvent) => void;
  clear: () => void;
}

export const usePdfToJpegStore = create<PdfToJpegStore>((set) => ({
  jobs: {},

  setJob: (job) =>
    set((state) => ({
      jobs: {
        ...state.jobs,
        [getJobId(job)]: job,
      },
    })),

  removeJob: (job) =>
    set((state) => {
      const jobs = { ...state.jobs };
      delete jobs[getJobId(job)];
      return { jobs };
    }),

  clear: () => set({ jobs: {} }),
}));