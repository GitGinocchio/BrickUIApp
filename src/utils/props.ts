import type { CollectionValueTypes, GradientStop, NotNullProp, Prop, ValidProp } from "~/interfaces";
import type { AllPropsType } from "~/interfaces";

export function isKnownProp<T extends { prop_type: string }>(prop: T): prop is Exclude<T, { prop_type: 'Unknown' }> {
  return prop.prop_type !== 'Unknown';
}

export function isValidProp<T extends { prop_type: string }>(prop: T): prop is Exclude<T, { prop_type: 'Unknown' | 'Deprecated' }> {
  return prop.prop_type !== 'Unknown' && prop.prop_type !== 'Deprecated';
}

export function isNotNullProp(prop: ValidProp): prop is NotNullProp {
  return prop.prop_type !== 'Null';
}

export function isCollectionProp<T extends ValidProp>(prop: T): prop is Extract<T, { prop_type: 'Select' | 'Array' }> {
  return prop.prop_type === 'Select' || prop.prop_type === 'Array';
}

export function isNumericProp<T extends { prop_type: string }>(
  prop: T
): prop is Extract<T, { prop_type: 'Int' | 'Float' }> {
  return prop.prop_type === 'Int' || prop.prop_type === 'Float';
}

function createBaseMeta(name: string, description: string | null) {
  return {
    prop_name: name,
    description: description ?? undefined,
  };
}

function createBaseProp<T>(type: string, name: string, description: string | null) {
  return {
    ...createBaseMeta(name, description),
    prop_type: type as any,
    value: undefined as T | undefined,
    default: undefined as T | undefined,
  };
}

export function createProp(
  type: AllPropsType,
  name: string,
  description: string | null,
  value_type: CollectionValueTypes = 'String'
): Prop {

  switch (type) {
    case "String":
    case "Text":
      return createBaseProp(type, name, description);

    case "Bool":
      return { ...createBaseProp<boolean>('Bool', name, description), default: false };

    case "Int":
    case "Float":
      return {
        ...createBaseProp<number>(type, name, description),
        min: null, max: null, step: null
      };

    case "Select":
      return {
        ...createBaseProp<any[]>('Select', name, description),
        options: [],
        value_type,
        min: null, max: null, min_value: null, max_value: null
      };

    case "Array":
      return {
        ...createBaseProp<any[]>('Array', name, description),
        value_type,
        min: null, max: null, min_value: null, max_value: null
      };

    case "Color":
      return {
        ...createBaseProp<string>('Color', name, description),
        skip_alpha: false,
        swatches: [],
        saved: []
      };

    case "Gradient":
      return {
          ...createBaseProp<GradientStop[]>('Gradient', name, description),
        skip_alpha: false,
        type: "Linear",
        default: [],
        value: []
      };

    case "Date":
    case "Datetime":
    case "Time":
      return {
        ...createBaseProp<bigint>(type, name, description),
        allow_future: true,
        allow_past: true
      };

    case "Null":
      return { prop_type: "Null", prop_name: name, description: description };

    case "Deprecated":
      return {
        ...createBaseProp<any>('Deprecated', name, description),
        deprecated_type: "Unknown"
      };

    default:
      return {
        prop_type: "Unknown"
      };
  }
}