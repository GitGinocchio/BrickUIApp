use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Top-level application settings.")]
pub struct Settings {
    #[serde(default = "default_schema", rename = "$schema", skip)]
    #[schemars(description = "The JSON Schema version or URI for this Settings definition.")]
    pub schema: String,

    #[serde(default = "default_theme")]
    pub theme: Theme,

    #[serde(default = "default_language")]
    pub language: Language,

    #[serde(default)]
    pub sidebar: Sidebar,

    #[serde(default)]
    #[schemars(description = "Notification settings.")]
    pub notifications: Notifications,

    #[serde(default)]
    #[schemars(description = "TaskBar settings.")]
    pub taskbar: TaskBar,

    #[serde(default)]
    #[schemars(description = "Start Menu settings.")]
    pub startmenu: StartMenu,

    #[serde(default)]
    #[schemars(
        description = "Determines whether the application should automatically start when the system boots."
    )]
    pub autostart: bool,

    #[serde(default)]
    #[schemars(description = "Determines whether enable or not the system tray icon")]
    pub systemtray: SystemTray,
}

fn default_theme() -> Theme {
    Theme::Light
}
fn default_language() -> Language {
    Language::EN
}
fn default_schema() -> String {
    "./.schemas/settings.schema.json".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    EN,
    IT,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: default_language(),
            theme: default_theme(),
            schema: default_schema(),
            notifications: Notifications::default(),
            taskbar: TaskBar::default(),
            startmenu: StartMenu::default(),
            autostart: false,
            systemtray: SystemTray::default(),
            sidebar: Sidebar::default()
        }
    }
}

/* Notifications */

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Configuration for notifications display.")]
pub struct Notifications {
    #[serde(default)]
    #[schemars(description = "Position on screen where notifications appear.")]
    pub position: NotificationPosition,
}

impl Default for Notifications {
    fn default() -> Self {
        Self {
            position: NotificationPosition::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Possible positions for notifications on screen.")]
pub enum NotificationPosition {
    #[schemars(description = "Notifications appear at the top center.")]
    #[serde(rename = "top")]
    Top,

    #[schemars(description = "Notifications appear at the top left corner.")]
    #[serde(rename = "top-left")]
    TopLeft,

    #[schemars(description = "Notifications appear at the top right corner.")]
    #[serde(rename = "top-right")]
    TopRight,

    #[schemars(description = "Notifications appear at the bottom center.")]
    #[serde(rename = "bottom")]
    Bottom,

    #[schemars(description = "Notifications appear at the bottom left corner.")]
    #[serde(rename = "bottom-left")]
    BottomLeft,

    #[schemars(description = "Notifications appear at the bottom right corner.")]
    #[serde(rename = "bottom-right")]
    BottomRight,
}
impl Default for NotificationPosition {
    fn default() -> Self {
        NotificationPosition::TopRight
    }
}

/* Sidebar */

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Sidebar settings")]
pub struct Sidebar {
    position: SidebarPosition
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            position: SidebarPosition::Left
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Position of the app sidebar")]
pub enum SidebarPosition {
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "left")]
    Left
}

/* System Tray */

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Configuration for the system tray icon.")]
pub struct SystemTray {
    #[schemars(description = "Determines whether enable or not the system tray icon")]
    #[serde(default)]
    pub enabled: bool,

    #[serde(default)]
    pub hidetaskbaricon: bool,
}

impl Default for SystemTray {
    fn default() -> Self {
        Self {
            enabled: false,
            hidetaskbaricon: false,
        }
    }
}

/* TaskBar */

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Configuration for windows taskbar.")]
pub struct TaskBar {
    #[serde(default)]
    pub behavior: TaskBarBehavior,
}

impl Default for TaskBar {
    fn default() -> Self {
        Self {
            behavior: TaskBarBehavior::default(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Taskbar behavior.")]
pub enum TaskBarBehavior {
    #[schemars(description = "Taskbar is hidden and the space is used by apps and bricks.")]
    #[serde(rename = "hide")]
    Hide,

    #[schemars(description = "Use Windows default taskbar behavior.")]
    #[serde(rename = "show")]
    Show,
}

impl Default for TaskBarBehavior {
    fn default() -> Self {
        TaskBarBehavior::Show
    }
}

/* Start Menu */
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Configuration for windows start menu.")]
pub struct StartMenu {
    #[serde(default)]
    pub behavior: StartMenuBehavior,
}

impl Default for StartMenu {
    fn default() -> Self {
        Self {
            behavior: StartMenuBehavior::default(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum StartMenuBehavior {
    #[schemars(description = "Disables Ctrl+Esc from opening the Start menu.")]
    DisableCtrlEsc,

    #[schemars(description = "Disables the Windows key.")]
    DisableWin,

    #[schemars(description = "Disables both Ctrl+Esc and the Windows key.")]
    DisableBoth,

    #[schemars(description = "Uses the default Windows behavior.")]
    WindowsDefault,
}

impl Default for StartMenuBehavior {
    fn default() -> Self {
        StartMenuBehavior::WindowsDefault
    }
}
