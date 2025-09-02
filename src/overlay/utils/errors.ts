import { getCurrentInstance } from "vue";
import { app } from "../loader";

export function catchBrickError(error: Error, brickName?: string) {
    const instance = getCurrentInstance();

    if (!instance) {
        app.config.errorHandler?.(error, app as any, brickName ? brickName : '');
        return;
    }

    instance?.appContext.config.errorHandler?.(error, instance as any, brickName ? brickName : instance.vnode.key.toString());
}

export function catchBrickWarning(warn: Error) {
    const instance = getCurrentInstance().appContext ?? app;

    instance.config.warnHandler?.(warn.message, instance as any, warn.stack);
}