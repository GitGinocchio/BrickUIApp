export type {
  Settings,
  Sidebar,
  Notifications,
  SystemTray,
  Taskbar,
  StartMenu,
  NotificationPosition,
  SidebarPosition,
  Language,
  Theme,
  TaskBarBehavior,
  StartMenuBehavior,
} from "./generated/Settings";

export const sidebarPositions = ["right", "left"] as const;

export const languages = ["it", "en"] as const;

export const themes = ["dark", "light", "system"] as const;

/** Possible positions for notifications on screen. */
export const notificationPositions = [
  "top",
  "top-left",
  "top-right",
  "bottom",
  "bottom-left",
  "bottom-right",
] as const;

export const taskBarBehaviors = ["hide", "show"] as const;

export const startMenuBehaviors = [
  "disable-ctrl-esc",
  "disable-win",
  "disable-both",
  "windows-default",
] as const;
