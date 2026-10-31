import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

/**
 * Tauri Bridge providing 1:1 API parity with Electron's preload window.electronAPI.
 * This ensures the exact same React components run seamlessly in both environments.
 */
export const tauriAPI = {
  // Settings & Storage
  getSettings: async () => {
    try {
      return await invoke('get_settings');
    } catch (e) {
      console.warn('Failed to invoke get_settings, falling back to localStorage', e);
      const raw = localStorage.getItem('spiderman_hydration_settings');
      return raw ? JSON.parse(raw) : null;
    }
  },

  saveSettings: async (settings) => {
    try {
      return await invoke('save_settings', { settings });
    } catch (e) {
      console.warn('Failed to invoke save_settings', e);
      localStorage.setItem('spiderman_hydration_settings', JSON.stringify(settings));
      return settings;
    }
  },

  getLoginItemSettings: async () => {
    try {
      return await invoke('get_login_item_settings');
    } catch (e) {
      return false;
    }
  },

  setLoginItemSettings: async (enabled) => {
    try {
      return await invoke('set_login_item_settings', { enabled });
    } catch (e) {
      return enabled;
    }
  },

  saveCustomAudio: async (soundType, base64Data, extension) => {
    try {
      return await invoke('save_custom_audio', { soundType, base64Data, extension });
    } catch (e) {
      console.warn('Failed to save audio via Tauri', e);
      return null;
    }
  },

  // Window & Mouse Passthrough
  setIgnoreMouseEvents: async (ignore, options = {}) => {
    try {
      await invoke('set_ignore_mouse_events', { ignore });
    } catch (e) {
      console.warn('Failed to set mouse passthrough', e);
    }
  },

  closeApp: async () => {
    try {
      await invoke('close_app');
    } catch (e) {
      window.close();
    }
  },

  // System Tray & Lifecycle Event Listeners
  onTriggerReminder: (callback) => {
    return listen('trigger-reminder', (event) => callback(event.payload));
  },

  onOpenSettings: (callback) => {
    return listen('open-settings', () => callback());
  },

  onDrinkWater: (callback) => {
    return listen('action-drink', () => callback());
  },

  onSnoozeReminder: (callback) => {
    return listen('action-snooze', () => callback());
  },

  onIdleStatus: (callback) => {
    return listen('idle-status', (event) => callback(event.payload));
  },

  onPowerSuspend: (callback) => {
    return listen('power-suspend', () => callback());
  },

  onPowerResume: (callback) => {
    return listen('power-resume', () => callback());
  },

  onTogglePause: (callback) => {
    return listen('toggle-pause', (event) => callback(event.payload));
  },
};

// Expose on window.electronAPI as well so all existing React components work out of the box
if (typeof window !== 'undefined') {
  window.electronAPI = tauriAPI;
  window.hydrationAPI = tauriAPI;
}
