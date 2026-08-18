import type { AllPropsKind, CollectionValueKind, ValidPropKind } from "~/interfaces";

export const VALID_PROPS_TYPES = createExhaustiveArray<ValidPropKind>()(
  "Bool", 
  "String", 
  "Text", 
  "Int", 
  "Float", 
  "Color", 
  "Gradient", 
  "Date", 
  "Datetime", 
  "Time", 
  "Array", 
  "Select", 
  "Null"
);

export const ALL_PROPS_TYPES = createExhaustiveArray<AllPropsKind>()(
  ...VALID_PROPS_TYPES,
  "Deprecated", 
  "Unknown"
);

export const COLLECTION_VALUE_TYPES = createExhaustiveArray<CollectionValueKind>()(
  "Float",
  "Int",
  "String"
);