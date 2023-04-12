import { PropertyType } from "../property_type/property_type.mjs";
import { escape_string } from "../string_escaper.mjs";
import { DataValue } from "./data_value.mjs";

export class DataValueString extends DataValue {
    #inner: string;

    constructor(inner: string) {
        super();

        this.#inner = inner;
    }

    get serialized(): string {
        return `"${escape_string(this.#inner, ['"'])}"`;
    }

    static deserialize(serializedDataValueType: PropertyType, serializedDataValue: string): DataValue | undefined {
        if (serializedDataValueType.baseType != "" && serializedDataValueType.baseType != "string" || serializedDataValueType.subTypesIncluded()) {
            return undefined;
        }

        if (serializedDataValue.startsWith('"') && serializedDataValue.endsWith('"') && serializedDataValue.length > 1) {
            return new DataValueString(serializedDataValue.slice(1, -1).replace("\\", ""));
        }

        return undefined;
    };

    get type(): PropertyType {
        return new PropertyType("string", []);
    }
}