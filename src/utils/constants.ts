// Utility per garantire che l'array contenga TUTTI i valori del tipo
export type MissingValues<T extends string, U extends readonly string[]> = Exclude<T, U[number]>;

export type EnforceExhaustive<T extends string, U extends readonly T[]> = 
  [MissingValues<T, U>] extends [never]
    ? U
    : [`Error: Missing elements in the array ->`, MissingValues<T, U>];


export function createExhaustiveArray<T extends string>() {
  return <U extends readonly T[]>(...arr: U & EnforceExhaustive<T, U>): U => arr;
}