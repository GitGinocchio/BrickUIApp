<template></template>

<script setup lang="ts">
import { Window } from '@tauri-apps/api/window';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import { invoke } from '@tauri-apps/api/core';
import { emitTo } from '@tauri-apps/api/event';
import { CheckMenuItem, CheckMenuItemOptions, SubmenuOptions, Menu, Submenu, IconMenuItemOptions, MenuItem, MenuItemOptions, PredefinedMenuItemOptions, PredefinedMenuItem, IconMenuItem } from '@tauri-apps/api/menu';
import { TrayIcon } from '@tauri-apps/api/tray';
import { Brick } from 'interfaces/brick';
import { Language, languages, Settings, Theme, themes } from '../interfaces/settings';
import { inject, onMounted, onUnmounted, ref, Ref, watch } from 'vue';
import { Image } from '@tauri-apps/api/image';
import { readFile } from '@tauri-apps/plugin-fs';
import { BaseDirectory } from '@tauri-apps/api/path';

let tray: TrayIcon = null;

const settings = inject("settings") as Ref<Settings>;
const bricks = inject("bricks") as Ref<Brick[]>;

let iconsLoaded = false;
let paletteIcon: Image;
let languagesIcon: Image;
let logoutIcon: Image;
let settingsIcon: Image;
let layoutDashboardIcon: Image;
let panelBottomCloseIcon: Image;
let checkIcon: Image;

async function buildMenuFromOptions(options: (CheckMenuItemOptions | IconMenuItemOptions | SubmenuOptions | MenuItemOptions | PredefinedMenuItemOptions)[]): Promise<(CheckMenuItem | MenuItem | Submenu | PredefinedMenuItem | IconMenuItem)[]> {
  const items: (CheckMenuItem | MenuItem | Submenu | PredefinedMenuItem | IconMenuItem)[] = [];

  for (const opt of options) {
    if ('checked' in opt) {
        items.push(await CheckMenuItem.new(opt)); // oggetto costruito
    } else if ('items' in opt) {
        const subItems = await buildMenuFromOptions(opt.items as any); // ricorsione
        items.push(await Submenu.new({ ...opt, items: subItems })); // oggetto costruito
    } else if ('item' in opt) {
        items.push(await PredefinedMenuItem.new(opt));
    } else if ('icon' in opt) {
        items.push(await IconMenuItem.new(opt));
    } else {
        // fallback: MenuItem
        items.push(await MenuItem.new(opt));
    }
  }

  return items;
}

async function loadIcon(path: string): Promise<Image> {
    const bytes = await readFile(path, { baseDir: BaseDirectory.Resource });
    return Image.fromBytes(bytes);
}

async function createMenu(){
    if (!tray) return;

    const itemsOptions: (CheckMenuItemOptions|SubmenuOptions|MenuItemOptions|PredefinedMenuItemOptions)[] = [
        {
            id: 'bricks-menu',
            text: 'Bricks',
            icon: layoutDashboardIcon,
            items: bricks.value.map((brick) => {
                return {
                    id: brick.name,
                    text: brick.name,
                    icon: brick.enabled ? checkIcon : null,
                    action: async () => {
                        const b = bricks.value.find(b => b.name === brick.name);
                        if (b) b.enabled = !b.enabled;
                        await emitTo("overlay", "toggle-brick", {brick: b});
                        await invoke("save_brick", { brick: b });
                    },
                }
            })
        },
        {
            id: 'settings-menu',
            text: 'Settings',
            icon: settingsIcon,
            items: [
                {
                    id: 'language',
                    text: 'Language',
                    icon: languagesIcon,
                    items: languages.map((language) => {
                        return {
                            text: language,
                            id: language,
                            icon: language === settings.value.language ? checkIcon : null,
                            action: async () => {
                                settings.value.language = language as Language;
                                await invoke("save_settings", { settings });
                            }
                        }
                    }) as CheckMenuItemOptions[],
                },
                {
                    id: 'theme',
                    text: 'Theme',
                    icon: paletteIcon,
                    items: themes.map((theme) => {
                        return {
                            text: theme,
                            id: theme,
                            icon: theme === settings.value.theme ? checkIcon : null,
                            action: async () => {
                                settings.value.theme = theme as Theme;
                                await invoke("save_settings", { settings });
                            }
                        }
                    })
                },
                {
                    id: 'separator',
                    item: 'Separator'
                },
                {
                    id: 'system-tray',
                    text: 'System Tray',
                    icon: settings.value.systemtray ? checkIcon : null,
                    action: async () => {
                        settings.value.systemtray.enabled = !settings.value.systemtray.enabled;
                        await invoke("save_settings", { settings });
                    }
                },
                {
                    id: 'autostart',
                    text: 'Autostart',
                    icon: settings.value.autostart ? checkIcon : null,
                    action: async () => {
                        settings.value.autostart = !settings.value.autostart;
                        await invoke("save_settings", { settings });
                    }
                }
            ]
        },
        {
            id: 'separator',
            item: 'Separator',
        },
        {
            id: "quit",
            text: 'Quit',
            icon: logoutIcon,
            action: async () => {
                const mainWindow = await Window.getByLabel("main");
                if (mainWindow) {
                    if (mainWindow.isVisible()) {
                        await mainWindow.close();
                    }
                    await mainWindow.close();
                }
            }
        },
    ]

    const items = await buildMenuFromOptions(itemsOptions);

    const menu = await Menu.new({ items: items });
    
    await tray.setMenu(menu);
}

async function showSystemTray() {
    if (tray) { 
        tray.setVisible(true);
        return;
    };

    tray = await TrayIcon.new({
        icon: await defaultWindowIcon(),
        action: async (event) => {
            switch (event.type) {
                case 'DoubleClick':
                    const mainWindow = await Window.getByLabel("main");
                    if (!mainWindow) return;

                    await mainWindow.show();
                    await mainWindow.setFocus();
            }
        },
        showMenuOnLeftClick : false
    });

    await createMenu();
}

async function hideSystemTray() {
    if (!tray) return;
    await tray.setVisible(false);
}

watch(() => bricks.value.map(b => b.enabled), async () => settings.value.systemtray.enabled ? await createMenu() : null);
watch(() => [
    settings.value.systemtray.enabled, 
    settings.value.autostart,
    settings.value.theme, 
    settings.value.language,
], async () => settings.value.systemtray.enabled ? await createMenu() : null);
watch(async () => settings.value.systemtray.enabled,async () => {
    settings.value.systemtray.enabled ? await showSystemTray() : await hideSystemTray();
});
onMounted(async () => {
    if (iconsLoaded) return;

    paletteIcon = await loadIcon('assets/icons/palette.png');
    languagesIcon = await loadIcon('assets/icons/languages.png');
    logoutIcon = await loadIcon('assets/icons/log-out.png');
    settingsIcon = await loadIcon('assets/icons/settings.png');
    layoutDashboardIcon = await loadIcon('assets/icons/layout-dashboard.png');
    panelBottomCloseIcon = await loadIcon('assets/icons/panel-bottom-close.png');
    checkIcon = await loadIcon('assets/icons/check.png');

    settings.value.systemtray.enabled ? await showSystemTray() : null
});
onUnmounted(hideSystemTray);
</script>