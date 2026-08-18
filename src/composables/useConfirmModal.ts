import { reactive } from 'vue';

export type ConfirmColor = "neutral" | "primary" | "error" | "secondary" | "success" | "info" | "warning";

export type ConfirmInputOptions = {
  title?: string;
  message?: string;
  confirmLabel?: string;
  cancelLabel?: string;
  color?: ConfirmColor;
};

export type ConfirmState = ConfirmInputOptions & {
  open: boolean;
  _resolve?: (value: boolean) => void;
};

const state = reactive<ConfirmState>({
  open: false,
  title: '',
  message: '',
  confirmLabel: '',
  cancelLabel: '',
  color: 'neutral',
  _resolve: undefined,
});

export function useConfirmModalState() {
  return state;
}

export function useConfirmModal() {
  const { t } = useI18n();

  async function confirm(opts: ConfirmInputOptions | string): Promise<boolean> {
    const o: ConfirmInputOptions = typeof opts === 'string' ? { message: opts } : opts || {};

    return new Promise<boolean>((resolve) => {
      state.title = o.title ?? '';
      state.message = o.message ?? '';
      state.confirmLabel = o.confirmLabel ?? t('actions.ok');
      state.cancelLabel = o.cancelLabel ?? t('actions.cancel');
      state.color = o.color ?? 'neutral';
      state._resolve = resolve;
      state.open = true;
    });
  }

  function resolve(value: boolean) {
    if (state._resolve) {
      state._resolve(value);
      state._resolve = undefined;
    }
    state.open = false;
  }

  return { confirm, resolve, state };
}