import { useState } from "react";

type Setting = {
  apiKey: string | null;
};

const defaultSetting: Setting = {
  apiKey: null,
};

export function getSetting(): Setting {
  const settingString = localStorage.getItem("setting");
  return settingString ? JSON.parse(settingString) : defaultSetting;
}

export function setSetting(setting: Setting) {
  localStorage.setItem("setting", JSON.stringify(setting));
}

export function useSetting() {
  const [setting, setSettingState] = useState(getSetting);

  return {
    setting,
    setSetting(setting: Setting) {
      setSettingState(setting);
      setSetting(setting);
    },
  };
}
