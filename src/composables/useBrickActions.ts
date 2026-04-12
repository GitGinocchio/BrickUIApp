import { invoke } from "@tauri-apps/api/core";
import { emitTo } from "@tauri-apps/api/event";
import { save as openSaveDialog } from "@tauri-apps/plugin-dialog";
import { Pencil } from "lucide-vue-next";
import { NAlert, NButton, NFormItem, NInput, NSpace, useDialog, useNotification } from "naive-ui";
import type { DialogApiInjection } from "naive-ui/es/dialog/src/DialogProvider";
import type { NotificationApiInjection } from "naive-ui/es/notification/src/NotificationProvider";
import type { BuiltInGlobalTheme } from "naive-ui/es/themes/interface";
import { BRICK_FILE_FILTERS, BRICK_NAME_REGEX } from "~/constants/brick";
import type { Brick } from "~/interfaces/brick";

export const useBrickActions = (
  bricks: Ref<Brick[]>, 
  theme: Ref<BuiltInGlobalTheme>, 
  d?: DialogApiInjection,
  n?: NotificationApiInjection
) => {
  const { t } = useI18n();
  const dialog = d ?? useDialog();
  const notification = n ?? useNotification();

  const openBrick = async (brick: [string, string]) => {
    if (bricks.value.some((b) => b.name === brick[1])) {
      dialog.error({
        style: theme.value,
        title: "Brick already imported",
        content: "There is already a brick with this name",
        positiveText: 'Ok',
        draggable: true,
        maskClosable: false,
        closable: true,
      });
      return;
    }

    dialog.warning({
      title: `Are you sure you want to import brick "${brick[1]}"`,
      content: () => h(NSpace, { vertical: true, size: 'medium' }, {
        default: () => [
          // Primo Alert: Warning
          h(NAlert, { 
            title: t('modals.warning', 'Warning'), 
            type: 'warning' 
          }, {
            default: () => t('modals.untrusted_source_msg', 'Careful when importing bricks from untrusted sources')
          }),
          
          // Secondo Alert: Info
          h(NAlert, { 
            title: t('modals.info', 'Info'), 
            type: 'info' 
          }, {
            default: () => [
              t('modals.verify_msg', 'You can verify the brick by yourself by changing the file extension from '),
              h('code', '.brick'),
              t('modals.to_msg', ' to '),
              h('code', '.zip'),
              t('modals.look_inside_msg', ' and look to the code inside')
            ]
          })
        ]
      }),
      positiveText: 'Confirm',
      negativeText: 'Cancel',
      onPositiveClick: async () => {
        bricks.value = await invoke("unpack_brick", { 
          brickPath: brick[0], 
          brickName: brick[1] 
        });

        navigateTo(`/bricks/${brick[1]}`);
      }
    });
  };

  const shareBrick = async (brick: Brick) => {
    const path = await openSaveDialog({
      title: t('modals.save_brick_title', "Save your brick!"),
      defaultPath: `${brick.name}.brk`,
      filters: BRICK_FILE_FILTERS
    });

    if (!path) return;

    const n = notification.create({
      type: 'info',
      title: t('notifications.packing_title', 'Packing...'),
      content: t('notifications.packing_description', `Preparing brick '${brick.name}' for export.`),
      closable: false,
      duration: 0
    });

    try {
      await invoke("pack_brick", { brickName: brick.name, outputPath: path });

      n.type = 'success';
      n.title = t('notifications.pack_success_title', `Brick '${brick.name}' ready`);
      n.content = t('notifications.pack_success_description', `Export completed successfully.`);
      n.closable = true;
      n.duration = 5000;

      n.action = () =>
        h(
          NButton,
          {
            type: 'primary',
            size: 'small',
            onClick: async () => {
              try {
                await invoke("open_file_folder", { path })
              } catch (err) {
                console.error("Failed to open folder:", err);
              }
            }
          },
          { default: () => t('actions.open_explorer', 'Open in Explorer') }
        );

    } catch (error) {
      n.type = 'error';
      n.title = t('notifications.pack_error_title', 'Packing failed');
      n.content = String(error);
      n.closable = true;
      n.duration = 10000;
    }
  };

  const renameBrick = async (brick: Brick, afterRename?: (newName: string) => any | Promise<any>) => {
    const newName = ref(brick.name);
    const feedback = ref<string | null>(null);

    const onEditBrick = async () => {
      if (brick.name == newName.value) return;
      bricks.value = await invoke("rename_brick", { oldName: brick.name, newName: newName.value });
      if (afterRename) await afterRename(newName.value);
    }

    const d = dialog.create({
      title: t('modals.edit_brick_title', 'Edit Brick'),
      icon: () => h(Pencil),
      content: () => 
        h(NFormItem, {
          label: t('fields.name', 'Name'),
          validationStatus: feedback.value ? 'error' : undefined,
          feedback: feedback.value,
        }, {
          default: () => h(NInput, {
            value: newName.value,
            placeholder: t('placeholders.brick_name', "Type your brick's name"),
            onUpdateValue: (v: string) => {
              newName.value = v;
              
              if (!v) {
                feedback.value = null;
              } else if (!BRICK_NAME_REGEX.test(v)) {
                feedback.value = t('errors.invalid_name', "Must start with a letter. Only letters, numbers, and underscores allowed.");
              } else if (newName.value != brick.name && bricks.value.some(b => b.name === v)) {
                feedback.value = t('errors.duplicate_name', "A brick with this name already exists.");
              } else {
                feedback.value = null;
              }

              d.positiveButtonProps = {
                disabled: !newName.value || feedback.value !== null
              };
            },
            onKeyup: async (e: KeyboardEvent) => {
              if (e.key === 'Enter' && newName.value && !feedback.value) {
                d.destroy();
                await onEditBrick();
              }
            }
          })
        }),
      positiveText: t('actions.create', 'Create'),
      negativeText: t('actions.cancel', 'Cancel'),
      positiveButtonProps: {
        disabled: true 
      },
      onPositiveClick: onEditBrick
    });
  };

  const toggleBrick = debounce(async (brick: Brick) => {
    await emitTo("overlay", "toggle-brick", { brick: brick });
    bricks.value = await invoke("save_brick", { brick: brick });
  }, 150);

  const deleteBrick = async (brick: Brick, afterDelete?: () => any | Promise<any>) => {
    dialog.warning({
      title: "Confirm deletion",
      content: `Are you sure you want to delete "${brick.name}"?`,
      positiveText: 'Delete',
      negativeText: 'Cancel',
      onPositiveClick: async () => {
        let title = t('notifications.delete_success_title', "Brick deleted");
        let description = t('notifications.delete_success_description', { name: brick.name }, 
          `Brick '${brick.name}' has been successfully removed.`
        );
        let type: 'success' | 'error' = 'success';

        try {
          await emitTo("overlay", "delete-brick", { brick: brick });
          bricks.value = await invoke("delete_brick", { brick: brick });
          if (afterDelete) await afterDelete();
        } catch (error) {
          type = 'error';
          title = t('notifications.delete_failure_title', "Could not delete Brick");
          description = t('notifications.delete_failure_description', { name: brick.name }, 
            `Brick '${brick.name}' has not been removed.`
          );
        }

        notification.create({
          type: type,
          title: title,
          description: description,
          duration: 5000,
          closable: true
        });
      }
    });
  };

  const duplicateBrick = debounce(async (brick: Brick, afterDuplicate?: () => any | Promise<any>) => {
    try {
      bricks.value = await invoke("duplicate_brick", { brick: brick });
      if (afterDuplicate) afterDuplicate();
    } catch (error) {
      notification.error({
        title: `Could not duplicate this brick`,
        description: error as string
      });
    }
  }, 150);

  return { openBrick, shareBrick, renameBrick, duplicateBrick, deleteBrick, toggleBrick };
}