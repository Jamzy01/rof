import { PropertyType } from "../property_type/property_type.mjs";
import { DataValue } from "./data_value.mjs";
import { DataValueBool } from "./data_value_bool.mjs";
import { DataValueCharacter } from "./data_value_char.mjs";
import { DataValueNumber } from "./data_value_number.mjs";
import { DataValueString } from "./data_value_string.mjs";

export function dataValueFromString(dataValueType: PropertyType, serializedDataValue: string): DataValue | null {
    // Deserialize Bool
    
    let serializedDataValueAsBool = DataValueBool.deserialize(
      dataValueType,
      serializedDataValue
    );
  
    if (serializedDataValueAsBool != null) {
      return serializedDataValueAsBool;
    }
  
    // Deserialize Character
  
    let serializedDataValueAsChar = DataValueCharacter.deserialize(
      dataValueType,
      serializedDataValue
    );
  
    if (serializedDataValueAsChar != null) {
      return serializedDataValueAsChar;
    }
    
    // Deserialize String
  
    let serializedDataValueAsString = DataValueString.deserialize(
      dataValueType,
      serializedDataValue
    );
  
    if (serializedDataValueAsString != null) {
      return serializedDataValueAsString;
    }

    // Deserialize Number
  
    let serializedDataValueAsNumber = DataValueNumber.deserialize(
        dataValueType,
        serializedDataValue
    );

    if (serializedDataValueAsNumber != null) {
        return serializedDataValueAsNumber;
    }
  
    // Failed To Deserialize
  
    return null;
  }