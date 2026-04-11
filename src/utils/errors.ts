import { getCurrentInstance } from "vue";
import { app } from "../loader";
import { NButton, type NotificationApi } from "naive-ui";

export function catchBrickError(error: Error, brickContext?: { name: string, author?: string}, info: string = '') {
    const instance = getCurrentInstance();

    (error as any).timestamp = new Date().toISOString();

    // ottieni la linea e colonna dall’Error stack
    const stack = error.stack;
    if (stack) {
      // esempio: estrai l’ultima riga dello stack
      const match = stack.match(/:(\d+):(\d+)\)?$/);
      if (match) {
        (error as any).line = parseInt(match[1], 10);
        (error as any).column = parseInt(match[2], 10);
      }
    }

    if (!instance) {
        app.config.errorHandler?.(error, brickContext as any, info);
        return;
    }

    instance?.appContext.config.errorHandler?.(error, instance as any, info);
}

export function catchBrickWarning(warn: Error) {
    const instance = getCurrentInstance().appContext ?? app;

    instance.config.warnHandler?.(warn.message, instance as any, warn.stack);
}

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
