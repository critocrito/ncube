export interface ConfigSettingReq {
  name: string;
  value: string;
}

export interface ConfigSetting {
  name: string;
  value?: string;
  required: boolean;
  restricted: boolean;
  description: string;
}

export type HostConfig = ConfigSetting[];
