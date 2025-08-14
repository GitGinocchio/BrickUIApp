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

  /** Semantic version of the Brick as [major, minor, patch]. */
  version: [number, number, number];

  /** Flag indicating whether the Brick is enabled (true) or disabled (false). */
  enabled: boolean;

  /** Custom properties defined for the Brick. */
  props: Prop[];
}

/** Enumeration of supported property types for a Brick (flatten respected). */
export type Prop =
  | ({ prop_type: 'String' } & PropType<string>)
  | ({ prop_type: 'Int' | 'Float' } & NumericPropType<number>)
  | ({ prop_type: 'StringArray' } & ArrayPropType<string>)
  | ({ prop_type: 'IntArray' | 'FloatArray' } & ArrayPropType<number>)
  | ({ prop_type: 'StringSelect' } & SelectablePropType<string>)
  | ({ prop_type: 'IntSelect' | 'FloatSelect' } & SelectablePropType<number>)
  | ({ prop_type: 'Array' } & ArrayPropType<any>)
  | ({ prop_type: 'Select' } & SelectablePropType<any>)
  | ({ prop_type: 'Any' } & PropType<any>)
  | ({ prop_type: 'Color'} & ColorPropType);

/** Generic property container. */
export interface PropType<T> {
  /** The unique identifier for this property. Must exactly match the `prop_name` used in the corresponding `brick.vue` file to ensure proper binding and synchronization. */
  prop_name: string;

  /**
   * A brief textual description providing additional details or context about the property.
   * This helps users understand the purpose or usage of the property.
   */
  description: string;

  /** Current value of the property. May be null if `nullable` is true. */
  value?: T | null;

  /** Default value of the property. */
  default?: T | null;
}

/** Property container for numeric types, including optional bounds. */
export interface NumericPropType<T> extends PropType<T> {
  /** Minimum allowed value (inclusive). */
  min?: T;

  /** Maximum allowed value (inclusive). */
  max?: T;
}

/** Color property container. */
export interface ColorPropType extends PropType<[number, number, number, number]> {
  /** If true, the alpha channel is ignored. */
  skip_alpha? : boolean;

  /** Default swatches to show to the user (0-255 for each channel). */
  swatches? : [number, number, number, number][]
}

/** Array property container with typed values. */
export interface ArrayPropType<T> extends PropType<T> {
  /** Minimum number of items allowed in the array. */
  min?: number;

  /** Maximum number of items allowed in the array. */
  max?: number;
}

/** Selectable property container with typed options. */
export interface SelectablePropType<T> extends PropType<T> {
  /** List of selectable options. */
  options: T[];

  /** Minimum number of selections allowed. */
  min?: number;

  /** Maximum number of selections allowed. */
  max?: number;
}
