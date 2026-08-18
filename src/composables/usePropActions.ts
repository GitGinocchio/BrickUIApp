import type { Brick, Prop, ValidProp } from "~/interfaces";
import { isValidProp } from "~/utils/props";

export const usePropActions = () => {
  const { confirm } = useConfirmModal();
  const { saveBrick } = useBrickActions();
  const toast = useToast();
  const { t } = useI18n();

  const deleteProp = (brick: Brick, prop: Prop) => {
    brick.props = brick.props.filter(p => p !== prop);
    saveBrick(brick);
  }

  const duplicateProp = (brick: Brick, prop: ValidProp) => {
    const copyName = `${prop.prop_name}Copy`;
    if (brick.props.some(p => isValidProp(p) && p.prop_name === copyName)) {
      toast.add({ title: t('errors.copy_exists'), description: '', color: 'error' });
      return;
    }

    const index = brick.props.findIndex(p => isValidProp(p) && p.prop_name === prop.prop_name);
    brick.props.splice(index + 1, 0, { ...prop, prop_name: copyName });
    saveBrick(brick);
  }

  const changePropOrder = (brick: Brick, oldIndex: number, newIndex: number) => {
    if (oldIndex !== newIndex) {
      const newList = [...brick.props];
      const [movedItem] = newList.splice(oldIndex, 1);
      newList.splice(newIndex, 0, movedItem);
      brick.props = newList;
      saveBrick(brick);
    }
  }

  const openDeletePropModal = async (brick: Brick, prop: Prop) => {
    const ok = await confirm({
      title: t('modals.delete_prop_title', 'Delete prop'),
      message: t('modals.delete_prop_message', { name: isValidProp(prop) ? prop.prop_name : 'Invalid Prop' }),
      confirmLabel: t('actions.delete', 'Delete'),
      cancelLabel: t('actions.cancel', 'Cancel'),
      color: 'error'
    });

    if (ok) {
      deleteProp(brick, prop);
    }
  }

  return {
    duplicateProp,
    deleteProp,
    changePropOrder,
    
    openDeletePropModal
  };
}