import { PropertyType } from "../property_type/property_type.mjs";
import { DataValue } from "./data_value.mjs";

export class DataValueBool extends DataValue {
  #inner: boolean;

  constructor(inner: boolean) {
    super();

    this.#inner = inner;
  }

  get serialized(): string {
    switch (this.#inner) {
      case true:
        return "true";
      case false:
        return "false";
    }
  }

  static deserialize(serializedDataValueType: PropertyType, serializedBool: string): DataValue | undefined {
    if (
      (!serializedDataValueType.isImplicit() &&
        serializedDataValueType.baseType != "bool") ||
      serializedDataValueType.subTypesIncluded()
    ) {
      return undefined;
    }

    switch (serializedBool) {
      case "true":
        return new DataValueBool(true);
      case "false":
        return new DataValueBool(false);
      default:
        return undefined;
    }
  }

  get type(): PropertyType {
    return PropertyType.simple("bool");
  }
}
