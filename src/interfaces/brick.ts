export type { Brick, Prop } from "./generated/Brick";
export type { GradientType } from "./generated/GradientType";
import type { GradientStop } from "./generated/GradientStop";

export const propTypeValues = [
  "String",
  "Text",
  "Int",
  "Float",
  "Bool",
  "Select",
  "Array",
  "Color",
  "Gradient",
  "Date",
  "Datetime",
  "Time",
  "Null",
  "Deprecated",
  "Unknown",
] as const;

export type PropTypeValue = typeof propTypeValues[number];

export const CollectionValueTypes = ["String", "Integer", "Float"] as const;
export type CollectionValueType = typeof CollectionValueTypes[number];

// --- Factory Functions ---

type BaseProp<TValue> = {
  prop_type: string;
  prop_name: string;
  description?: string | null;
  value?: TValue | null;
  default?: TValue | null;
};

function createBaseProp<TValue>(
  type: string,
  name: string,
  description: string | null = null
): BaseProp<TValue> {
  return {
    prop_type: type,
    prop_name: name,
    description,
    value: null,
    default: null,
  };
}

export function createProp(
  type: PropTypeValue,
  name: string,
  description: string,
  value_type: CollectionValueType = 'String'
): Prop {
  const desc = description || null;

  switch (type) {
    case "String":
    case "Text":
      return createBaseProp<string, typeof type>(type, name, desc);

    case "Bool":
      return { ...createBaseProp<boolean, 'Bool'>('Bool', name, desc), default: false };

    case "Int":
    case "Float":
      return {
        ...createBaseProp<number, typeof type>(type, name, desc),
        min: null, max: null, step: null
      };

    case "Select":
      return {
        ...createBaseProp<any[], 'Select'>('Select', name, desc),
        options: [],
        value_type,
        min: null, max: null, min_value: null, max_value: null
      };

    case "Array":
      return {
        ...createBaseProp<any[], 'Array'>('Array', name, desc),
        value_type,
        min: null, max: null, min_value: null, max_value: null
      };

    case "Color":
      return {
        ...createBaseProp<string, 'Color'>('Color', name, desc),
        skip_alpha: false,
        swatches: [],
        saved: []
      };

    case "Gradient":
      return {
          ...createBaseProp<GradientStop[], 'Gradient'>('Gradient', name, desc),
        skip_alpha: false,
          type: "Linear",
        default: [],
        value: []
      };

    case "Date":
    case "Datetime":
    case "Time":
      return {
        ...createBaseProp<number, typeof type>(type, name, desc),
        allow_future: true,
        allow_past: true
      };

    case "Null":
      return { prop_type: "Null", prop_name: name, description: desc };

    case "Deprecated":
      return {
        ...createBaseProp<any, 'Deprecated'>('Deprecated', name, desc),
        deprecated_type: "Unknown"
      };

    default:
      return {
        prop_type: "Unknown",
        prop_name: name,
        description: desc
      };
  }
}