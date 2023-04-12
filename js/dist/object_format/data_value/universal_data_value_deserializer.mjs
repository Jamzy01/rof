import { DataValueBool } from "./data_value_bool.mjs";
import { DataValueCharacter } from "./data_value_char.mjs";
import { DataValueNumber } from "./data_value_number.mjs";
import { DataValueString } from "./data_value_string.mjs";
export function dataValueFromString(dataValueType, serializedDataValue) {
    let serializedDataValueAsBool = DataValueBool.deserialize(dataValueType, serializedDataValue);
    if (serializedDataValueAsBool != null) {
        return serializedDataValueAsBool;
    }
    let serializedDataValueAsChar = DataValueCharacter.deserialize(dataValueType, serializedDataValue);
    if (serializedDataValueAsChar != null) {
        return serializedDataValueAsChar;
    }
    let serializedDataValueAsString = DataValueString.deserialize(dataValueType, serializedDataValue);
    if (serializedDataValueAsString != null) {
        return serializedDataValueAsString;
    }
    let serializedDataValueAsNumber = DataValueNumber.deserialize(dataValueType, serializedDataValue);
    if (serializedDataValueAsNumber != null) {
        return serializedDataValueAsNumber;
    }
    return null;
}
