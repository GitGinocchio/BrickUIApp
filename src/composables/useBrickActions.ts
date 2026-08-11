import { invoke } from "@tauri-apps/api/core";
import { emitTo } from "@tauri-apps/api/event";
import { save as openSaveDialog } from "@tauri-apps/plugin-dialog";
import { BRICK_FILE_FILTERS } from "~/constants/brick";
import type { Brick, Prop } from "~/interfaces";

let lastInvoicedState;

/* 
TODO: Sostituire cose come:
  const { bricks } = useAppState();
  bricks.value = await invoke("save_brick", { brick: brick });
In qualcosa come:
  const { bricks } = useAppState();
  bricks[brick.name] = brick;
  await invoke("save_brick", { brick: brick });
In modo che prima avvengano le modifiche in ram e poi le modifiche sul filesystem
(Per fare questo e' necessario trasformare OVUNQUE Vec<Brick> in HashMap<String, Brick>)
*/

export const useBrickActions = () => {
  const { t } = useI18n();
  const toast = useToast();

  const shareBrick = debounce(async (brick: Brick) => {
    const path = await openSaveDialog({
      title: t('modals.save_brick_title', "Save your brick!"),
      defaultPath: `${brick.name}.brk`,
      filters: BRICK_FILE_FILTERS
    });

    if (!path) return;

    toast.add({
      title: t('notifications.packing_title', 'Packing...'),
      description: t('notifications.packing_description', `Preparing '${brick.name}'`),
      color: 'info',
      progress: true,
      duration: 10000
    });

    try {
      await invoke("pack_brick", { brickName: brick.name, outputPath: path });

      toast.clear();

      toast.add({
        title: t('notifications.pack_success_title', 'Ready'),
        description: t('notifications.pack_success_description'),
        color: 'success',
        type: 'foreground',
        actions: [{
          label: t('actions.open_explorer'),
          variant: 'solid',
          color: 'success',
          onClick: async () => await invoke("open_file_folder", { path })
        }],
        progress: false,
        duration: 0
      });
    } catch (error) {
      toast.clear();

      toast.add({
        title: t('notifications.pack_error_title', 'Packing failed'),
        description: String(error),
        color: 'error'
      });
    }
  }, 500);
  
  const toggleBrick = debounce(async (brick: Brick) => {
    await emitTo("overlay", "toggle-brick", { brick });
    saveBrick(brick);
  }, 150);

  const duplicateBrick = debounce(async (brick: Brick, afterDuplicate?: () => any) => {
    try {
      const { bricks } = useAppState();
      bricks.value = await invoke("duplicate_brick", { brick });
      if (afterDuplicate) afterDuplicate();
    } catch (error) {
      toast.add({ title: "Could not duplicate", description: String(error), color: 'error' });
    }
  }, 150);

  const renameBrick = async (oldName: string, newName: string) => {
    const { bricks } = useAppState();
    bricks.value = await invoke("rename_brick", { oldName, newName });
  };

  const importBrick = debounce(async (brickPath: string, brickName: string) => {
    const { bricks } = useAppState();
    bricks.value = await invoke("unpack_brick", { brickPath: brickPath, brickName: brickName });
  }, 150);

  const newBrick = async (brick: Brick) => {
    await invoke("new_brick", { brick: brick });
    const { bricks } = useAppState();
    bricks.value.push(brick);
  };

  const saveBrick = debounce(async (brick: Brick) => {
    const currentState = JSON.stringify(brick);

    if (currentState === lastInvoicedState) {
      console.log("No real changes, skipping invoke.");
      return;
    }

    try {
      const { bricks } = useAppState();
      bricks.value = await invoke("save_brick", { brick: brick });
      lastInvoicedState = currentState;
      console.log("Brick saved to disk via Rust!");
    } catch (err) {
      console.error("Failed to save brick:", err);
    }
  }, 2000);

  const deleteBrick = debounce(async (brick: Brick) => {
    await emitTo("overlay", "delete-brick", { brick: brick });
    const { bricks } = useAppState();
    bricks.value = await invoke("delete_brick", { brick: brick });
  }, 150);

  const updateBrickProp = debounce(async (brick_name: string, prop: Prop) => {
    await emitTo("overlay", "update-brick", { name: brick_name, prop });
    try {
      const { bricks } = useAppState();
      const target = bricks.value.find(b => b.name === brick_name);
      if (target) {
        saveBrick(target);
      }
    } catch (err) {
      console.error('Failed to persist updated prop for', brick_name, err);
    }
  }, 150);

  const openRenameBrickModal = async (brick: Brick, redirect: boolean) => {
    await emitTo<{ brick: Brick, redirect: boolean }>("main", "rename_brick", { brick, redirect });
  };

  const openImportBrickModal = async (brick_path: string, brick_name: string) => {
    await emitTo<[string, string]>("main", "import_brick", [brick_path, brick_name]);
  };

  const openNewBrickModal = async () => {
    await emitTo("main", "new_brick");
  };

  const openDeleteBrickModal = async (brick: Brick) => {
    // Use the global confirm modal for in-app flows; fall back to emitting to main if needed
    const { confirm } = useConfirmModal();
    const ok = await confirm({
      title: t('modals.delete_brick_title', 'Delete brick'),
      message: t('modals.delete_brick_message', `Are you sure you want to delete '${brick.name}'? This action cannot be undone.`),
      confirmLabel: t('actions.delete', 'Delete'),
      cancelLabel: t('actions.cancel', 'Cancel'),
      color: 'error'
    });

    if (ok) {
      deleteBrick(brick);
    } else {
      // No-op on cancel. The existing Tauri flow still listens for main events and opens the old modal if triggered from main.
    }
  };

  return { 
    duplicateBrick, 
    toggleBrick, 
    shareBrick,
    renameBrick,
    importBrick,
    newBrick,
    saveBrick,
    deleteBrick,

    updateBrickProp,
    
    openRenameBrickModal,
    openImportBrickModal, 
    openNewBrickModal, 
    openDeleteBrickModal 
  };
};