export type Prop =
  | {
      prop_type: "string";
      /** Current string value of the property. */
      value: string;
      /** Default string value of the property. */
      default: string;
    }
  | {
      prop_type: "int";
      /** Current integer value of the property. */
      value: number;
      /** Default integer value of the property. */
      default: number;
    }
  | {
      prop_type: "bool";
      /** Current boolean value of the property. */
      value: boolean;
      /** Default boolean value of the property. */
      default: boolean;
    }
  | {
      prop_type: "array";
      /** Current array value of the property. */
      value: unknown[];
      /** Default array value of the property. */
      default: unknown[];
    };

export interface Brick {
  /** The JSON Schema version or URI for this Brick definition. */
  $schema: string; // default: "../.schemas/brick.schema.json"

  /** The unique name identifier for the Brick. */
  name: string;

  /** A brief textual description of the Brick. */
  description: string; // default: ""

  /** A list of tags for categorizing or labeling the Brick. */
  tags: string[]; // default: []

  /** List of dependencies required by this Brick. */
  dependencies: string[]; // default: []

  /** Optional license information for the Brick. */
  license?: string | null;

  /** Optional icon path or URL representing the Brick. */
  icon?: string | null;

  /** Author or creator of the Brick. */
  author: string; // default: ""

  /** Semantic version of the Brick as [major, minor, patch]. */
  version: [number, number, number]; // default: [1, 0, 0]

  /** Flag indicating whether the Brick is enabled (true) or disabled (false). */
  enabled: boolean; // default: true

  /** Custom properties defined for the Brick. */
  props: Prop[]; // default: []
}
