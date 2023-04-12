import { PropertyType } from "../property_type/property_type.mjs";
export class DataValue {
    constructor() { }
    get serialized() {
        return "";
    }
    static deserialize(serializedDataValueType, serializedDataValue) {
        return;
    }
    get type() {
        return PropertyType.empty();
    }
}
