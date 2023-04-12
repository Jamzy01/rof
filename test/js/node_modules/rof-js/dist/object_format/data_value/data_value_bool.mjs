import { PropertyType } from "../property_type/property_type.mjs";
import { DataValue } from "./data_value.mjs";
export class DataValueBool extends DataValue {
    #inner;
    constructor(inner) {
        super();
        this.#inner = inner;
    }
    get serialized() {
        switch (this.#inner) {
            case true:
                return "true";
            case false:
                return "false";
        }
    }
    static deserialize(serializedDataValueType, serializedBool) {
        if ((!serializedDataValueType.isImplicit() &&
            serializedDataValueType.baseType != "bool") ||
            serializedDataValueType.subTypesIncluded()) {
            return undefined;
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
    get type() {
        return PropertyType.simple("bool");
    }
}
