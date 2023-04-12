import { PropertyType } from "../property_type/property_type.mjs";

export abstract class DataValue {
  constructor() {}

  get serialized(): string {
    return "";
  }

  static deserialize(serializedDataValueType: PropertyType, serializedDataValue: String): DataValue | undefined {
    return;
  }

  get type(): PropertyType {
    return PropertyType.empty();
  }
}