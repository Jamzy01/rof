import { DataValue } from "../data_value/data_value.mjs";
import { dataValueFromString } from "../data_value/universal_data_value_deserializer.mjs";
import { DataValueBool } from "../data_value/data_value_bool.mjs";
import { ignoringCompliantSplitOnce } from "../ignore_string_split.mjs";
import { PropertyType } from "../property_type/property_type.mjs";

export class Property {
  propertyIndex: string;
  #propertyValue: DataValue;

  constructor(propertyIndex: string, propertyValue: DataValue) {
    this.propertyIndex = propertyIndex;
    this.#propertyValue = propertyValue;
  }

  serialize(): string {
    if (this.#propertyValue.type.isImplicit()) {
      return `${this.propertyIndex} = ${this.#propertyValue.serialized}`;
    } else {
      return `${this.propertyIndex}: ${this.#propertyValue
        .type
        .serialized} = ${this.#propertyValue.serialized}`;
    }
  }

  static deserialize(serializedProperty: string) {
    let propertyIndex: string = "";

    let propertyValue: DataValue = new DataValueBool(false); //TODO: Replace with none value when I implement it

    // Get Property Index (Property Name)

    let [rawData, rawValue] = ignoringCompliantSplitOnce(
      serializedProperty,
      "=",
      true,
      []
    );

    if (rawData.includes(":")) {
      let [rawIndex, rawType] = ignoringCompliantSplitOnce(
        rawData,
        ":",
        true,
        []
      );

      // Explicit Type

      propertyIndex = rawIndex.trim();

      let deserializedPropertyType: DataValue | null = dataValueFromString(
        PropertyType.deserialize(rawType),
        rawValue.trim()
      );

      if (deserializedPropertyType != null) {
        propertyValue = deserializedPropertyType as DataValue;
      }

      
    } else {
      // Implicit type (only supported in certain situations)

      propertyIndex = rawData.trim();

      let deserializedPropertyType: DataValue | null = dataValueFromString(
        PropertyType.implicit(),
        rawValue.trim()
      );

      if (deserializedPropertyType != null) {
        propertyValue = deserializedPropertyType as DataValue;
      }
    }

    if (propertyValue == null) {
      propertyValue = new DataValueBool(false); //TODO: Replace with none enum value when I implement it
    }

    return new Property(propertyIndex, propertyValue);
  }

  get propertyValue(): DataValue {
    return this.#propertyValue;
  }
}
