import { PropertyType } from "../property_type/property_type";
import { DataValueBool } from "./data_value_bool";

export abstract class DataValue {
  constructor() {}

  abstract serialize(): string;

  static deserialize(serializedDataValueType: PropertyType, serializedDataValue: String): DataValue {
    return new DataValueBool(false); // TODO: Replace with none enum value when I implement it
  };

  abstract getType(): PropertyType;
}

export function dataValueFromString(dataValueType: PropertyType, serializedDataValue: string): DataValue {
  let serializedDataValueAsBool = DataValueBool.deserialize(
    dataValueType,
    serializedDataValue
  );

  if (serializedDataValueAsBool != null) {
    return serializedDataValueAsBool;
  }

  return new DataValueBool(false); //TODO: Replace with none enum when I implement its
}

// Re export data value types

export { DataValueBool };
