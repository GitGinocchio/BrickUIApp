import { getCurrentInstance } from "vue";
import { app } from "../loader";

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