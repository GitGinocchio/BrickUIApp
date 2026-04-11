/** A Brick represents a modular component with metadata, configuration, and properties. */
export interface Brick {
  /** The JSON Schema version or URI for this Brick definition. */
  $schema?: string;

  /** The unique name identifier for the Brick. */
  name: string;

  /** A brief textual description of the Brick. */
  description: string;

  /** A list of tags for categorizing or labeling the Brick. */
  tags: string[];

  /** List of dependencies required by this Brick. */
  dependencies: string[];

  /** Optional license information for the Brick. */
  license?: string;

  /** Optional icon path or URL representing the Brick. */
  icon?: string;

  /** Author or creator of the Brick. */
  author: string;

  /** Brick banner */
  banner?: string;

  /** Semantic version of the Brick as [major, minor, patch]. */
  version: [number, number, number];

  /** Flag indicating whether the Brick is enabled (true) or disabled (false). */
  enabled: boolean;

  /** Custom properties defined for the Brick. */
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

// Tipo unione inferito automaticamente
export type PropTypeValue = typeof propTypeValues[number];

/*
// I metadati base che ogni proprietà possiede
interface PropMeta {
  prop_name: string;
  description?: string;
}

// Il tipo KnownProp (quello con tag obbligatorio)
type KnownProp = 
  | { prop_type: 'Bool'; data: { meta: PropMeta; value?: boolean; default?: boolean } }
  | { prop_type: 'String'; data: { meta: PropMeta; value?: string; default?: string } }
  | { prop_type: 'Int'; data: { meta: PropMeta; value?: number; default?: number } }
  // ... aggiungi gli altri tipi (Color, Date, ecc.)
  | { prop_type: 'Null'; data: PropMeta };

// Il tipo finale Prop (l'enum untagged)
export type Prop =
  | KnownProp
  // Caso Null Implicito: ha i metadati ma NON ha prop_type
  | (PropMeta & { prop_type?: never })
  // Caso Deprecated: ha un prop_type sconosciuto e i metadati
  | (PropMeta & { prop_type: string })
  // Caso Unknown/Fallback: accetta qualunque chiave stringa (il tuo "skibidi")
  | { [key: string]: any };

TODO: Da sostituire con una struttura del genere...
      o semplicemente passare a zod ._.
*/
/** Enumeration of supported property types for a Brick (flatten respected). */
export type Prop =
  | ({ prop_type: 'String' } & PropType<string>)
  | ({ prop_type: 'Text' } & PropType<string>)
  | ({ prop_type: 'Bool' } & PropType<boolean>)
  | ({ prop_type: 'Int' | 'Float' } & NumericPropType<number>)
  | ({ prop_type: 'Array', value_type: 'String' } & ArrayPropType<string>)
  | ({ prop_type: 'Array', value_type: 'Integer' | 'Float' } & ArrayPropType<number>)
  | ({ prop_type: 'Select', value_type: 'String' } & SelectablePropType<string>)
  | ({ prop_type: 'Select', value_type: 'Integer' | 'Float' } & SelectablePropType<number>)
  | ({ prop_type: 'Color'} & ColorPropType)
  | ({ prop_type: 'Gradient'} & GradientPropType)
  | ({ prop_type: 'Date'} & DatePropType)
  | ({ prop_type: 'Datetime'} & DatePropType)
  | ({ prop_type: 'Time'} & DatePropType)

  | ({ prop_type: 'Null' } & PropType<any>)
  | ({ prop_type: 'Deprecated', deprecated_type: String } & PropType<any>)
  | ({ prop_type: 'Unknown', [key: string]: any })
;

export interface PropMeta {
  /** The unique identifier for this property. Must exactly match the `prop_name` used in the corresponding `brick.vue` file to ensure proper binding and synchronization. */
  prop_name: string;

  /**
   * A brief textual description providing additional details or context about the property.
   * This helps users understand the purpose or usage of the property.
   */
  description?: string | null;
}

/** Generic property container. */
export interface PropType<T> extends PropMeta {
  /** Current value of the property. May be null if `nullable` is true. */
  value?: T | null;

  /** Default value of the property. */
  default?: T | null;
}

export interface DatePropType extends PropType<number> {
  /** If past dates are allowed */
  allow_past: boolean,

  /** If future dates are allowed */
  allow_future: boolean
}

/** Property container for numeric types, including optional bounds. */
export interface NumericPropType<T> extends PropType<T> {
  /** Minimum allowed value (inclusive). */
  min?: T;

  /** Maximum allowed value (inclusive). */
  max?: T;

  /** Step value. */
  step?: T;
}

/** Color property container. */
export interface ColorPropType extends PropType<string> {
  /** If true, the alpha channel is ignored. */
  skip_alpha? : boolean;

  /** Default swatches to show to the user. */
  swatches? : string[]

  /** Color saved by the user */
  saved? : string[]
}

export interface GradientStop {
  /** The actual color of the stop color. */
  color: string,
  /** The actual position of the stop color from 0 to 100. */
  position: number
} 

export enum GradientType {
  LINEAR = 'Linear',
  RADIAL = 'Radial',
  CONIC = 'Conic'
}

export interface GradientPropType extends PropType<Array<GradientStop>> {
  /** the gradient type */
  type?: GradientType;

  /** If true, the alpha channel is ignored. */
  skip_alpha? : boolean;
}

export const CollectionValueTypes = [
  'String',
  'Integer',
  'Float'
] as const;

// Tipo unione inferito automaticamente
export type CollectionValueType = typeof CollectionValueTypes[number];

/** Array property container with typed values. */
export interface ArrayPropType<T> extends PropType<Array<T>> {
  /** Minimum number of items allowed in the array. */
  min?: number;

  /** Maximum number of items allowed in the array. */
  max?: number;

  /** Options type */
  value_type?: any,

  /** Minimum value allowed in the array. */
  min_value?: number;

  /** Maximum value allowed in the array. */
  max_value?: number;
}

/** Selectable property container with typed options. */
export interface SelectablePropType<T> extends PropType<Array<T>> {
  /** List of selectable options. */
  options: T[];

  /** Selectable options type */
  value_type?: any,

  /** Minimum number of selections allowed. */
  min?: number;

  /** Maximum number of selections allowed. */
  max?: number;
  
  /** Minimum value allowed in the array. */
  min_value?: number;

  /** Maximum value allowed in the array. */
  max_value?: number;
}

function createBaseProp<T>(name: string, description = null): PropType<T> {
  return {
    prop_name: name,
    description,
    value: null,
    default: null
  }
}

function createNumericProp(name: string, description = null): NumericPropType<number> {
  return {
    ...createBaseProp<number>(name, description),
    min: null,
    max: null
  }
}

function createSelectableProp<T>(name: string, options: T[], description = null): SelectablePropType<T> {
  return {
    ...createBaseProp<Array<T>>(name, description),
    options,
    min: null,
    max: null,
    min_value: null,
    max_value: null
  }
}

function createArrayProp<T>(name: string, description = null): ArrayPropType<T> {
  return {
    ...createBaseProp<Array<T>>(name, description),
    min: null,
    max: null,
    min_value: null,
    max_value: null
  }
}

export function createProp(type: PropTypeValue, name: string, description: string, value_type?: CollectionValueType): Prop {
  switch (type) {
    case "String":
      return { prop_type: "String", ...createBaseProp<string>(name, description) }
    case "Text":
      return { prop_type: "Text", ...createBaseProp<string>(name, description) }
    case "Bool":
      return { prop_type: "Bool", default: false, ...createBaseProp<boolean>(name, description) }
    case "Int":
      return { prop_type: "Int", ...createNumericProp(name, description) }
    case "Float":
      return { prop_type: "Float", ...createNumericProp(name, description) }
    
    case "Select":
      switch (value_type) {
        case 'Float':
        case 'Integer':
          return { prop_type: "Select", value_type: value_type,...createSelectableProp<number>(name, [], description) }
        default:
          return { prop_type: "Select", value_type: value_type, ...createSelectableProp<string>(name, [], description) }
      }
    
    case "Array":
      switch (value_type) {
        case 'Float':
        case 'Integer':
          return { prop_type: "Array", value_type: value_type, ...createArrayProp<number>(name, description) }
        default:
          return { prop_type: "Array", value_type: value_type, ...createArrayProp<string>(name, description) }
      }

    case "Color":
      return { prop_type: "Color", ...createBaseProp<string>(name, description), skip_alpha: false, swatches: [], saved: [] }
    case "Gradient":
      return { prop_type: "Gradient", ...createBaseProp<Array<GradientStop>>(name, description), skip_alpha: false, type: GradientType.LINEAR }
    
    case "Date":
      return { prop_type: "Date", ...createBaseProp<number>(name, description), allow_future: true, allow_past: true }
    case "Datetime":
      return { prop_type: "Datetime", ...createBaseProp<number>(name, description), allow_future: true, allow_past: true }
    case "Time":
      return { prop_type: "Time", ...createBaseProp<number>(name, description), allow_future: true, allow_past: true }
    
    case "Null":
      return { prop_type: "Null", prop_name: name }

    default:
      throw new Error(`Invalid prop type ${type}`);
  }
}
