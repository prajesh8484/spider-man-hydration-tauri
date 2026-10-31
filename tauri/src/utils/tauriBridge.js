import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Cache active listeners to prevent duplicate listener accumulation on React re-renders
const activeListeners = new Map();

function registerListener(eventName, callback) {
  if (activeListeners.has(eventName)) {
    activeListeners.get(eventName).then((unlisten) => {
      if (typeof unlisten === 'function') unlisten();
    });
  }
  const unlistenPromise = listen(eventName, (event) => callback(event.payload));
  activeListeners.set(eventName, unlistenPromise);
  return unlistenPromise;
}

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

  recordDrink: async () => {
    try {
      return await invoke('record_drink');
    } catch (e) {
      console.warn('Failed to invoke record_drink via Tower service', e);
      return null;
    }
  },

  logServiceEvent: async (message) => {
    try {
      return await invoke('log_service_event', { message });
    } catch (e) {
      console.warn('Failed to invoke log_service_event', e);
      return null;
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

  // System Tray & Lifecycle Event Listeners (Safe single-registration)
  onTriggerReminder: (callback) => registerListener('trigger-reminder', callback),
  onOpenSettings: (callback) => registerListener('open-settings', callback),
  onDrinkWater: (callback) => registerListener('action-drink', callback),
  onSnoozeReminder: (callback) => registerListener('action-snooze', callback),
  onIdleStatus: (callback) => registerListener('idle-status', callback),
  onPowerSuspend: (callback) => registerListener('power-suspend', callback),
  onPowerResume: (callback) => registerListener('power-resume', callback),
  onTogglePause: (callback) => registerListener('toggle-pause', callback),
};

// Expose on window.electronAPI as well so all existing React components work out of the box
if (typeof window !== 'undefined') {
  window.electronAPI = tauriAPI;
  window.hydrationAPI = tauriAPI;
}
