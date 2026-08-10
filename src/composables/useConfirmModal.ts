import { reactive } from 'vue';

type ConfirmOptions = {
  title?: string;
  message?: string;
  confirmLabel?: string;
  cancelLabel?: string;
  color?: string;
};

const state = reactive({
  open: false,
  title: '' as string | null,
  message: '' as string | null,
  confirmLabel: '' as string | null,
  cancelLabel: '' as string | null,
  color: 'neutral',
  // internal
  _resolve: undefined as ((value: boolean) => void) | undefined,
});

export function useConfirmModalState() {
  return state;
}

export function useConfirmModal() {
  async function confirm(opts: ConfirmOptions | string): Promise<boolean> {
    const o = typeof opts === 'string' ? { message: opts } : opts || {};

    const { t } = useI18n();

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
