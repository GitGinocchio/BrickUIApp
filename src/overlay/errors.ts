import { NButton, NotificationApi } from "naive-ui";
import { ComponentPublicInstance, h } from "vue";
import { invoke } from "@tauri-apps/api/core";

export async function onBrickError(
  notification: NotificationApi,
  error: Error,
  instance: ComponentPublicInstance | { name: string; author: string },
  info: string
) {
  console.log(error, instance, info);
  if (!instance) return;
  if ("type" in instance) return;
  let brickName: string;
  let brickAuthor: string;

  if ("$options" in instance) {
    while ("$options" in instance && !("__brickContext" in instance.$options)) {
      instance = instance.$parent;
    }
    brickName = instance.$options?.__brickContext?.name;
    brickAuthor = instance.$options?.__brickContext?.author;
  } else if ("name" in instance && "author" in instance) {
    brickName = instance.name;
    brickAuthor = instance.author;
  }

  notification.error({
    // @ts-ignore
    title: `Error ${info ? `in ${info}` : ""} — brick "${brickName}"${
      brickAuthor !== "undefined" ? ` by ${brickAuthor}` : ""
    }`,
    content: () => {
      return h(
        "code",
        {
          style: {
            whiteSpace: "pre-wrap",
            fontSize: "12px",
            color: "#FFFFFF85",
          },
        },
        `${error.message ?? error}\n${error.cause ?? ""}`
      );
    },
    action: () => {
      return h(
        NButton,
        {
          style: { color: "white" },
          onClick: async () => {
            // @ts-ignore
            await invoke("open_brick", { brickName: brickName });
          },
        },
        {
          default: () => "Open Brick",
        }
      );
    },
    keepAliveOnHover: true,
    duration: 10000,
  });
}

export async function onBrickWarn(
  notification: NotificationApi,
  message: string,
  instance: ComponentPublicInstance | { name: string; author: string },
  _trace: string
) {
  console.log(message, instance, _trace);
  let brickName: string;
  let brickAuthor: string;
  if (!instance) return;
  if ("$options" in instance) {
    brickName = instance.$options.__brickContext.name;
    brickAuthor = instance.$options.__brickContext.author;
  } else if ("name" in instance && "author" in instance) {
    brickName = instance.name;
    brickAuthor = instance.author;
  }

  notification.error({
    title: `Warn in brick "${brickName}"${
      brickAuthor !== "undefined" ? ` by ${brickAuthor}` : ""
    }`,
    content: () => {
      return h(
        "code",
        {
          style: {
            whiteSpace: "pre-wrap",
            fontSize: "12px",
            color: "#FFFFFF85",
          },
        },
        `${message}`
      );
    },
    action: () => {
      return h(
        NButton,
        {
          style: { color: "white" },
          onClick: async () => {
            // @ts-ignore
            await invoke("open_brick", { brickName: brickName });
          },
        },
        {
          default: () => "Open Brick",
        }
      );
    },
    keepAliveOnHover: true,
    duration: 10000,
  });
}
