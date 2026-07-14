export interface JobEvent {
  cmd_name: string;
  resource_path: string;
  created_at: string;
  ui_action?: UIAction;
}

export enum UIAction {
  PasswordRequired = "PasswordRequired",
}


export const getJobId = (job: JobEvent) => `${job.cmd_name}@${job.resource_path}`;