import type { AllPropsType, CollectionValueTypes, ValidPropType } from "~/interfaces";

export const VALID_PROPS_TYPES = createExhaustiveArray<ValidPropType>()(
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

export const ALL_PROPS_TYPES = createExhaustiveArray<AllPropsType>()(
  ...VALID_PROPS_TYPES,
  "Deprecated", 
  "Unknown"
);

export const COLLECTION_VALUE_TYPES = createExhaustiveArray<CollectionValueTypes>()(
  "Float",
  "Int",
  "String"
);