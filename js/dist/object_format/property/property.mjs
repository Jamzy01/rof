import { dataValueFromString } from "../data_value/universal_data_value_deserializer.mjs";
import { DataValueBool } from "../data_value/data_value_bool.mjs";
import { ignoringCompliantSplitOnce } from "../ignore_string_split.mjs";
import { PropertyType } from "../property_type/property_type.mjs";
export class Property {
    propertyIndex;
    #propertyValue;
    constructor(propertyIndex, propertyValue) {
        this.propertyIndex = propertyIndex;
        this.#propertyValue = propertyValue;
    }
    serialize() {
        if (this.#propertyValue.type.isImplicit()) {
            return `${this.propertyIndex} = ${this.#propertyValue.serialized}`;
        }
        else {
            return `${this.propertyIndex}: ${this.#propertyValue
                .type
                .serialized} = ${this.#propertyValue.serialized}`;
        }
    }
    static deserialize(serializedProperty) {
        let propertyIndex = "";
        let propertyValue = new DataValueBool(false);
        let [rawData, rawValue] = ignoringCompliantSplitOnce(serializedProperty, "=", true, []);
        if (rawData.includes(":")) {
            let [rawIndex, rawType] = ignoringCompliantSplitOnce(rawData, ":", true, []);
            propertyIndex = rawIndex.trim();
            let deserializedPropertyType = dataValueFromString(PropertyType.deserialize(rawType), rawValue.trim());
            if (deserializedPropertyType != null) {
                propertyValue = deserializedPropertyType;
            }
        }
        else {
            propertyIndex = rawData.trim();
            let deserializedPropertyType = dataValueFromString(PropertyType.implicit(), rawValue.trim());
            if (deserializedPropertyType != null) {
                propertyValue = deserializedPropertyType;
            }
        }
        if (propertyValue == null) {
            propertyValue = new DataValueBool(false);
        }
        return new Property(propertyIndex, propertyValue);
    }
    get propertyValue() {
        return this.#propertyValue;
    }
}
