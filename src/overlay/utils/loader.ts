export function normalizeProps(propsArray) {
  return Object.fromEntries(
    propsArray.map(p => [p.prop_name, p.value ?? p.default])
  );
}