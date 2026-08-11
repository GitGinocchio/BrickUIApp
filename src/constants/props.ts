import type { AllPropsType, ValidPropType } from "~/interfaces";
import type { SatisfiesArray } from "~/utils/constants";

export const VALID_PROPS_TYPES = [
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
] as const satisfies SatisfiesArray<ValidPropType>;

export const ALL_PROPS_TYPES = [
  ...VALID_PROPS_TYPES,
  "Deprecated", 
  "Unknown"
] as const satisfies SatisfiesArray<AllPropsType>;