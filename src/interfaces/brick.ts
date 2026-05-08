/** A Brick represents a modular component with metadata, configuration, and properties. */
export interface Brick {
  $schema?: string;
  name: string;
  description: string;
  tags: string[];
  dependencies: string[];
  license?: string;
  icon?: string;
  author: string;
  banner?: string;
  version: [number, number, number];
  enabled: boolean;
  props: Prop[];
}

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
  "Unknown"
] as const;

export type PropTypeValue = typeof propTypeValues[number];

export const CollectionValueTypes = ["String", "Integer", "Float"] as const;
export type CollectionValueType = typeof CollectionValueTypes[number];

// --- Metadata & Base Types ---

export interface PropMeta<TType extends string = PropTypeValue> {
  /** The unique identifier for this property. */
  prop_name: string;
  /** The discriminator for the property type. */
  prop_type: TType;
  /** A brief textual description providing additional details. */
  description?: string | null;
}

/** Generic property container. */
export interface PropType<TValue, TType extends string = PropTypeValue> extends PropMeta<TType> {
  /** Current value of the property. */
  value?: TValue | null;
  /** Default value of the property. */
  default?: TValue | null;
}

// --- Specific Property Interfaces ---

export interface NumericPropType<TType extends 'Int' | 'Float'> extends PropType<number, TType> {
  min?: number | null;
  max?: number | null;
  step?: number | null;
}

export interface DatePropType<TType extends 'Date' | 'Datetime' | 'Time'> extends PropType<number, TType> {
  allow_past: boolean;
  allow_future: boolean;
}

export interface ColorPropType extends PropType<string, 'Color'> {
  skip_alpha?: boolean;
  swatches?: string[];
  saved?: string[];
}

export interface GradientStop {
  color: string;
  position: number;
}

export enum GradientType {
  LINEAR = 'Linear',
  RADIAL = 'Radial',
  CONIC = 'Conic'
}

export interface GradientPropType extends PropType<GradientStop[], 'Gradient'> {
  type?: GradientType;
  skip_alpha?: boolean;
}

export interface ArrayPropType<T> extends PropType<T[], 'Array'> {
  min?: number | null;
  max?: number | null;
  value_type?: CollectionValueType;
  min_value?: number | null;
  max_value?: number | null;
}

export interface SelectablePropType<T> extends PropType<T[], 'Select'> {
  options: T[];
  value_type?: CollectionValueType;
  min?: number | null;
  max?: number | null;
  min_value?: number | null;
  max_value?: number | null;
}

// --- The Final Prop Union ---

export type Prop =
  | PropType<string, 'String'>
  | PropType<string, 'Text'>
  | PropType<boolean, 'Bool'>
  | NumericPropType<'Int'>
  | NumericPropType<'Float'>
  | ArrayPropType<string>
  | ArrayPropType<number>
  | SelectablePropType<string>
  | SelectablePropType<number>
  | ColorPropType
  | GradientPropType
  | DatePropType<'Date'>
  | DatePropType<'Datetime'>
  | DatePropType<'Time'>
  | (PropMeta<'Null'> & { value?: never; default?: never })
  | (PropMeta<'Deprecated'> & { deprecated_type: string } & PropType<any, 'Deprecated'>)
  | (PropMeta<'Unknown'> & { [key: string]: any; value?: any; default?: any });

// --- Factory Functions ---

function createBaseProp<TValue, TType extends string>(
  type: TType,
  name: string,
  description: string | null = null
): PropType<TValue, TType> {
  return {
    prop_type: type,
    prop_name: name,
    description,
    value: null,
    default: null
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
        type: GradientType.LINEAR,
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