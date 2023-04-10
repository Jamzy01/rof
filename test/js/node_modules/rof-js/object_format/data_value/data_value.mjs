import { PropertyType } from "../property_type/property_type.mjs";
import { DataValueBool } from "./data_value_bool.mjs";

export class DataValue {
  constructor() {}

  serialize() {
    "";
  }

  static deserialize(serializedDataValueType, serializedDataValue) {
    return new DataValue();
  }

  getType() {
    return PropertyType.empty();
  }
}

export function dataValueFromString(dataValueType, serializedDataValue) {
  let serializedDataValueAsBool = DataValueBool.deserialize(
    dataValueType,
    serializedDataValue
  );

  if (serializedDataValueAsBool != null) {
    return serializedDataValueAsBool;
  }
}

// Re export data value types

export { DataValueBool };
