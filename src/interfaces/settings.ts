
/** Possible positions for notifications on screen. */
export type NotificationPosition =
  | "top"
  | "top-left"
  | "top-right"
  | "bottom"
  | "bottom-left"
  | "bottom-right";

/** Configuration for notifications display. */
export interface Notifications {
  /** Position on screen where notifications appear. */
  position: NotificationPosition; // optional because of #[serde(default)]
}

/** Top-level application settings. */
export interface Settings {
  /** The JSON Schema version or URI for this Settings definition. */
  $schema?: string; // optional because of #[serde(default), rename = "$schema"]

  /** Notification settings. */
  notifications: Notifications; // optional because of #[serde(default)]
}
