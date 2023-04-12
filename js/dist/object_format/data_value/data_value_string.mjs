import { PropertyType } from "../property_type/property_type.mjs";
import { escape_string } from "../string_escaper.mjs";
import { DataValue } from "./data_value.mjs";
export class DataValueString extends DataValue {
    #inner;
    constructor(inner) {
        super();
        this.#inner = inner;
    }
    get serialized() {
        return `"${escape_string(this.#inner, ['"'])}"`;
    }
    static deserialize(serializedDataValueType, serializedDataValue) {
        if (serializedDataValueType.baseType != "" && serializedDataValueType.baseType != "string" || serializedDataValueType.subTypesIncluded()) {
            return undefined;
        }
        if (serializedDataValue.startsWith('"') && serializedDataValue.endsWith('"') && serializedDataValue.length > 1) {
            return new DataValueString(serializedDataValue.slice(1, -1).replace("\\", ""));
        }
        return undefined;
    }
    ;
    get type() {
        return new PropertyType("string", []);
    }
}
