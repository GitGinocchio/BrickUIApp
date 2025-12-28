/** Top-level application settings. */
export interface Settings {
  /** The JSON Schema version or URI for this Settings definition. */
  $schema?: string; // optional because of #[serde(default), rename = "$schema"]

  language: Language;

  theme: Theme;

  sidebar: Sidebar;

  /** Notification settings. */
  notifications: Notifications; // optional because of #[serde(default)]

  taskbar: Taskbar;

  startmenu: StartMenu;

  /** Determines whether the application should automatically start when the system boots. */
  autostart: boolean;

  /** System tray icon settings */
  systemtray: SystemTray;
}

export interface Sidebar {
  position: SidebarPosition
}

export const sidebarPositions = ["right", "left"] as const;

export type SidebarPosition = (typeof sidebarPositions)[number]

export const languages = ["it", "en"] as const;

export type Language = (typeof languages)[number];

export const themes = ["dark", "light", "system"] as const;

export type Theme = (typeof themes)[number];

/** Possible positions for notifications on screen. */
export const notificationPositions = [
  "top",
  "top-left",
  "top-right",
  "bottom",
  "bottom-left",
  "bottom-right",
] as const;

// 👉 il tipo si ricava automaticamente dall'array
export type NotificationPosition = (typeof notificationPositions)[number];

/** Configuration for notifications display. */
export interface Notifications {
  /** Position on screen where notifications appear. */
  position: NotificationPosition; // optional because of #[serde(default)]
}

export interface SystemTray {
  /** Determines whether enable or not the system tray icon */
  enabled: boolean;

  /** The main window taskbar icon will be hidden when minimizing the window (only available if system tray icon is enabled) */
  hidetaskbaricon: boolean;
}

export const taskBarBehaviors = ["hide", "show"] as const;

export type TaskBarBehavior = (typeof taskBarBehaviors)[number];

export interface Taskbar {
  behavior: TaskBarBehavior;
}

export const startMenuBehaviors = [
  "disable-ctrl-esc",
  "disable-win",
  "disable-both",
  "windows-default",
] as const;

export type StartMenuBehavior = (typeof startMenuBehaviors)[number];

export interface StartMenu {
  behavior: StartMenuBehavior;
}
