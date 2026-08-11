export type * from "./generated";

import type { DeprecatedProp, InvalidProp, ValidProp, SelectPropType, ArrayPropType } from "./generated";

export type ValidPropType = ValidProp["prop_type"];
export type InvalidPropType = InvalidProp["prop_type"];
export type DeprecatedPropType =  DeprecatedProp["prop_type"];
export type AllPropsType = ValidPropType | InvalidPropType | DeprecatedPropType;

export type NotNullProp = Exclude<ValidProp, { prop_type: "Null" }>;
export type NotNullPropType = NotNullProp["prop_type"];

export type NumericOrCollectionProp = Extract<ValidProp, { prop_type: 'Int' | 'Float' | 'Array' | 'Select' }>;

export type SelectPropValueTypes = SelectPropType["value_type"]
export type ArrayPropValueTypes = ArrayPropType["value_type"]
export type CollectionValueTypes = SelectPropValueTypes | ArrayPropValueTypes;