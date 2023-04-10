import { dataValueFromString } from "../data_value/data_value.mjs";
import { DataValueBool } from "../data_value/data_value_bool.mjs";
import { ignoringCompliantSplitOnce } from "../ignore_string_split.mjs";
import { PropertyType } from "../property_type/property_type.mjs";

export class Property {
  #propertyIndex; // String (property name)
  #propertyValue; // Data Value Inheriting Object

  constructor(propertyIndex, propertyValue) {
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

  static deserialize(serialized_property) {
    let propertyIndex = "";

    let propertyValue = new DataValueBool(false); //TODO: Replace with none value when I implement it

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
