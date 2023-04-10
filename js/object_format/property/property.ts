import { DataValue, DataValueBool, dataValueFromString } from "../data_value/data_value";
import { ignoringCompliantSplitOnce } from "../ignore_string_split.js";
import { PropertyType } from "../property_type/property_type.js";

export class Property {
  #propertyIndex;
  #propertyValue;

  constructor(propertyIndex: string, propertyValue: DataValue) {
    this.#propertyIndex = propertyIndex;
    this.#propertyValue = propertyValue;
  }

  serialize() {
    if (this.#propertyValue.getType().isImplicit()) {
      return `${this.#propertyIndex} = ${this.#propertyValue.serialize()}`;
    } else {
      return `${this.#propertyIndex}: ${this.#propertyValue
        .getType()
        .serialize()} = ${this.#propertyValue.serialize()}`;
    }
  }

  static deserialize(serialized_property: string) {
    let propertyIndex: string = "";

    let propertyValue: DataValue = new DataValueBool(false); //TODO: Replace with none value when I implement it

    // Get Property Index (Property Name)

    let [rawData, rawValue] = ignoringCompliantSplitOnce(
      serialized_property.slice(0, -1),
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

      propertyValue = dataValueFromString(
        PropertyType.deserialize(rawType),
        rawValue.trim()
      );
    } else {
      // Implicit type (only supported in certain situations)

      propertyIndex = rawData.trim();

      console.log("IMPLICIT TYPE");

      propertyValue = dataValueFromString(
        PropertyType.implicit(),
        rawValue.trim()
      );
    }

    if (propertyValue == null) {
      propertyValue = new DataValueBool(false); //TODO: Replace with none enum value when I implement it
    }

    return new Property(propertyIndex, propertyValue);
  }
}
