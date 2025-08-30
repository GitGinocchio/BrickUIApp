import { TrayIcon, TrayIconEvent } from "@tauri-apps/api/tray";
import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Menu, Submenu, CheckMenuItem } from "@tauri-apps/api/menu";
import type { Brick } from "interfaces/brick";
import { invoke } from "@tauri-apps/api/core";
import { emitTo } from "@tauri-apps/api/event";

const tray = await TrayIcon.new({
  icon: await defaultWindowIcon(),
  action: (event) => onTrayAction(event)
});

let brickItems: CheckMenuItem[];

async function createMenu(bricks: Brick[]){
  brickItems = await Promise.all(bricks.map(createBrickCheckMenuItem));

  const bricksMenu = await Submenu.new({ 
    id: "brick-menu", 
    text: "Bricks", 
    items: brickItems
  });

  const menu = await Menu.new({
    items: [
      bricksMenu,
      {
        id: "quit",
        text: "Quit",
        action: () => console.log("Quitting..."),
      },
    ],
  });
  
  return menu;
}

async function createBrickCheckMenuItem(brick: Brick) {
  return CheckMenuItem.new({
    id: brick.name,
    text: brick.name,
    checked: brick.enabled,
    action: async () => await emitTo("overlay", "toggle-brick", {brick: brick})
  });
}


async function onTrayAction(event: TrayIconEvent){
  switch(event.type){
    case "Click":
      const bricks = await invoke<Brick[]>("get_bricks");
      brickItems.forEach((item) => {
        const brick = bricks.find((brick) => brick.name === item.id)
        item.setChecked(brick.enabled)
      });
      break;
    default:
  }
}

export async function initTrayIcon(bricks: Brick[]) {
  const menu = await createMenu(bricks);
  await tray.setMenu(menu);
}