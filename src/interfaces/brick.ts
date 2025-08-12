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
  | ({ prop_type: 'string' } & PropType<string>)
  | ({ prop_type: 'int' | 'float' } & NumericPropType<number>)
  | ({ prop_type: 'string-array' } & ArrayPropType<string>)
  | ({ prop_type: 'int-array' | 'float-array' } & ArrayPropType<number>)
  | ({ prop_type: 'string-select' } & SelectablePropType<string>)
  | ({ prop_type: 'int-select' | 'float-select' } & SelectablePropType<number>)
  | ({ prop_type: 'array' } & ArrayPropType<any>)
  | ({ prop_type: 'select' } & SelectablePropType<any>)
  | ({ prop_type: 'any', prop_name: string, description: string } & Record<string, any>);

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

  /** Indicates whether this property can be null. */
  nullable?: boolean;
}

/** Property container for numeric types, including optional bounds. */
export interface NumericPropType<T> extends PropType<T> {
  /** Minimum allowed value (inclusive). */
  min?: T;

  /** Maximum allowed value (inclusive). */
  max?: T;
}

/** Array property container with typed values. */
export interface ArrayPropType<T> {
  /** The unique identifier for this property. Must exactly match the `prop_name` used in the corresponding `brick.vue` file to ensure proper binding and synchronization. */
  prop_name: string;

  /**
   * A brief textual description providing additional details or context about the property.
   * This helps users understand the purpose or usage of the property.
   */
  description: string;

  /** Indicates whether this array property can be null. */
  nullable?: boolean;

  /** Default value of the array property. */
  default?: T[];

  /** Current value of the array property. */
  values?: T[];

  /** Minimum number of items allowed in the array. */
  min_items?: number;

  /** Maximum number of items allowed in the array. */
  max_items?: number;
}

/** Selectable property container with typed options. */
export interface SelectablePropType<T> {
  /** The unique identifier for this property. Must exactly match the `prop_name` used in the corresponding `brick.vue` file to ensure proper binding and synchronization. */
  prop_name: string;

  /**
   * A brief textual description providing additional details or context about the property.
   * This helps users understand the purpose or usage of the property.
   */
  description: string;

  /** List of selectable options. */
  options: T[];

  /** Currently selected option. */
  selected?: T;

  /** Default selected option. */
  default_selected?: T;

  /** Indicates whether this property can be null. */
  nullable?: boolean;

  /** Minimum number of selections allowed. */
  min_selections?: number;

  /** Maximum number of selections allowed. */
  max_selections?: number;
}
