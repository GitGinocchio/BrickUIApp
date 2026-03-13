import { reactive } from "vue";
import { BrickError, BrickState } from "./types";
import { Brick } from "interfaces/brick";
import { formatProps } from "../utils/format";

export const bricksState = reactive(new Map<string, BrickState>());

export function setBrickState(brick: Brick) {
  bricksState.set(brick.name, reactive({
    enabled: brick.enabled,
    props: reactive(formatProps(brick.props)),
    errors: reactive([])
  }));
}

export function enableBrick(name: string) {
  console.log(`enabling brick: ${name}`) 
  const state = bricksState.get(name); 
  if (state) state.enabled = true; 
} 

export function disableBrick(name: string) {
  console.log(`disabling brick: ${name}`) 
  const state = bricksState.get(name); 
  
  if (state) state.enabled = false; 
}

export function isBrickInState(name: string) {
  return bricksState.has(name);
}

export function getBrickFromState(name: string) {
  return bricksState.get(name);
}

export function removeBrickFromState(name: string) {
  bricksState.delete(name);
}

export function addErrorToBrickState(error: BrickError) {
  const state = bricksState.get(error.brick.name);

  if (state) {
    state.errors.push(error);
  }
}

export function updateBrickProp(name: string, propKey: string, value: any) {
  const state = bricksState.get(name);
  if (state) {
    state.props.set(propKey, value);
  }
}
