export type * from "./generated";

import type { DeprecatedProp, InvalidProp, ValidProp, SelectPropKind, ArrayPropKind } from "./generated";

export type ValidPropKind = ValidProp["prop_type"];
export type InvalidPropKind = InvalidProp["prop_type"];
export type DeprecatedPropKind =  DeprecatedProp["prop_type"];
export type AllPropsKind = ValidPropKind | InvalidPropKind | DeprecatedPropKind;

export type NotNullPropType = Exclude<ValidProp, { prop_type: "Null" }>;
export type NotNullPropKind = NotNullPropType["prop_type"];

export type BoolPropType = Extract<ValidProp, { prop_type: 'Bool' }>;

export type StringPropType = Extract<ValidProp, { prop_type: 'String' }>;
export type TextPropType = Extract<ValidProp, { prop_type: 'Text' }>;

export type IntPropType = Extract<ValidProp, { prop_type: 'Int' }>;
export type FloatPropType = Extract<ValidProp, { prop_type: 'Float' }>;
export type NumericPropType = IntPropType | FloatPropType;

export type ColorPropType = Extract<ValidProp, { prop_type: 'Color' }>;
export type GradientPropType = Extract<ValidProp, { prop_type: 'Gradient' }>;

export type ArrayPropType = Extract<ValidProp, { prop_type: 'Array' }>;
export type SelectPropType = Extract<ValidProp, { prop_type: 'Select' }>;
export type CollectionPropType = ArrayPropType | SelectPropType;

export type DatePropType = Extract<ValidProp, { prop_type: 'Date' }>;
export type DatetimePropType = Extract<ValidProp, { prop_type: 'Datetime' }>;
export type TimePropType = Extract<ValidProp, { prop_type: 'Time' }>;

export type NumericOrCollectionProp = NumericPropType | CollectionPropType;

export type SelectPropValueKind = SelectPropKind["value_type"]
export type ArrayPropValueKind = ArrayPropKind["value_type"]
export type CollectionValueKind = SelectPropValueKind | ArrayPropValueKind;