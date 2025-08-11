use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Top-level application settings.")]
pub struct Settings {
    #[serde(default = "default_schema", rename = "$schema", skip)]
    #[schemars(description = "The JSON Schema version or URI for this Settings definition.")]
    pub schema: String,

    #[serde(default)]
    #[schemars(description = "Notification settings.")]
    pub notifications: Notifications,

    #[serde(default)]
    #[schemars(description = "TaskBar settings.")]
    pub taskbar: TaskBar
}

fn default_schema() -> String { "../.schemas/settings.schema.json".to_string() }

impl Default for Settings {
    fn default() -> Self {
        Self {
            schema: default_schema(),
            notifications: Notifications::default(),
            taskbar: TaskBar::default()
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

    #[schemars(description = "Notifications appear at the top left corner.") ]
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

/* TaskBar */

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Configuration for windows taskbar.")]
pub struct TaskBar {
    #[serde(default)]
    pub behavior: TaskBarBehavior,
}

impl Default for TaskBar {
    fn default() -> Self {
        TaskBar {
            behavior: TaskBarBehavior::default()
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Taskbar behavior.")]
pub enum TaskBarBehavior {
    #[schemars(description = "Taskbar is hidden but the reserved space remains.")]
    #[serde(rename = "hide")]
    Hide,

    #[schemars(description = "Taskbar is hidden and the space is used by apps and bricks.")]
    #[serde(rename = "hide-and-fill")]
    HideAndFill,

    #[schemars(description = "Use Windows default taskbar behavior.")]
    #[serde(rename = "windows-default")]
    WindowsDefault,
}

impl Default for TaskBarBehavior {
    fn default() -> Self {
        TaskBarBehavior::WindowsDefault
    }
}

