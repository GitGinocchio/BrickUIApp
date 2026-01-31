import { Prop } from "interfaces/brick";

export function formatPropsValueOrDefault(propsArray: Prop[]): Record<string, any> {
  return Object.fromEntries(
    propsArray.map((p) => [p.prop_name, p.value ?? p.default])
  );
}


export function formatPropValue(prop: Prop): any {
  let value: any;

  // Se il prop e' un select e il massimo di scelte e' una, passiamo solo l'unica scelta
  // al posto di un array con una scelta
  if (
    Array.isArray(prop.value) &&
    prop.prop_type === "Select" &&
    prop.value.length == 1 &&
    prop.max === 1
  ) {
    value = prop.value[0];
  } else if (Array.isArray(prop.value) && prop.value.length > 0) {
    value = prop.value;
  } else if (Array.isArray(prop.value)) {
    value = prop.default;
  } else {
    value = prop.value;
  }

  return value;
}

export function formatProps(props: Prop[]): Record<string, any> {
  return Object.fromEntries(
    props.map((p: Prop) => {
      return [p.prop_name, formatPropValue(p)];
    })
  );
}