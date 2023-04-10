import { DataValueBool } from "./data_value_bool";
export class DataValue {
    constructor() { }
    static deserialize(serializedDataValueType, serializedDataValue) {
        return new DataValueBool(false);
    }
    ;
}
export function dataValueFromString(dataValueType, serializedDataValue) {
    let serializedDataValueAsBool = DataValueBool.deserialize(dataValueType, serializedDataValue);
    if (serializedDataValueAsBool != null) {
        return serializedDataValueAsBool;
    }
    return new DataValueBool(false);
}
export { DataValueBool };
