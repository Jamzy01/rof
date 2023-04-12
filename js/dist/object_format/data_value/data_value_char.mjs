import { PropertyType } from "../property_type/property_type.mjs";
import { escape_string } from "../string_escaper.mjs";
import { DataValue } from "./data_value.mjs";
export class DataValueCharacter extends DataValue {
    #inner;
    constructor(inner) {
        super();
        this.#inner = inner;
    }
    get serialized() {
        return `'${escape_string(this.#inner, ["'"])}'`;
    }
    static deserialize(serializedDataValueType, serializedDataValueCharacter) {
        if (serializedDataValueType.baseType != "" && serializedDataValueType.baseType != "char" || serializedDataValueType.subTypesIncluded()) {
            return undefined;
        }
        if (serializedDataValueCharacter.startsWith("'") && serializedDataValueCharacter.endsWith("'") && serializedDataValueCharacter.length <= 4) {
            if (serializedDataValueCharacter.startsWith("'\\")) {
                return new DataValueCharacter(serializedDataValueCharacter[2]);
            }
            else {
                return new DataValueCharacter(serializedDataValueCharacter[1]);
            }
        }
        return undefined;
    }
    get type() {
        return new PropertyType("char", []);
    }
}
