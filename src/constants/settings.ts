import type { 
  SidebarPosition, 
  Language, 
  Theme, 
  NotificationPosition, 
  TaskBarBehavior, 
  StartMenuBehavior 
} from "~/interfaces";
import type { SatisfiesArray } from "~/utils/constants";

export const SIDEBAR_POSITIONS = ["right", "left"] as const satisfies SatisfiesArray<SidebarPosition>;

export const LANGUAGES = ["it", "en"] as const satisfies SatisfiesArray<Language>;

export const THEMES = ["dark", "light", "system"] as const satisfies SatisfiesArray<Theme>;

export const NOTIFICATION_POSITIONS = [
  "top",
  "top-left",
  "top-right",
  "bottom",
  "bottom-left",
  "bottom-right",
] as const satisfies SatisfiesArray<NotificationPosition>;

export const TASKBAR_BEHAVIORS = ["hide", "show"] as const satisfies SatisfiesArray<TaskBarBehavior>;

export const STARTMENU_BEHAVIORS = [
  "disable-ctrl-esc",
  "disable-win",
  "disable-both",
  "windows-default",
] as const satisfies SatisfiesArray<StartMenuBehavior>;