import { PropertyType } from "../property_type/property_type.mjs";

export class DataValueBool {
  #inner;

  constructor(inner) {
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

  static deserialize(serializedDataValueType, serializedBool) {
    console.log("DESERIALIZING BOOL");

    console.log(serializedBool);

    console.log(serializedDataValueType);

    if (
      (!serializedDataValueType.isImplicit() &&
        serializedDataValueType.baseType != "bool") ||
      serializedDataValueType.subTypesIncluded()
    ) {
      console.log("FAILED BOOL DESERIALIZING CHECKS");

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
