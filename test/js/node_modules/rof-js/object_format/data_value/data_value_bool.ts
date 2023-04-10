import { PropertyType } from "../property_type/property_type.js";

export class DataValueBool {
  #inner: boolean;

  constructor(inner: boolean) {
    this.#inner = inner;
  }

  serialize() {
    switch (this.#inner) {
      case true:
        return "true";
      case false:
        return "false";
    }
  }

  static deserialize(serializedDataValueType: PropertyType, serializedBool: string) {
    if (
      (!serializedDataValueType.isImplicit() &&
        serializedDataValueType.baseType != "bool") ||
      serializedDataValueType.subTypesIncluded()
    ) {
      return;
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

  getType() {
    return PropertyType.simple("bool");
  }
}
