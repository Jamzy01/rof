import { PropertyType } from "../property_type/property_type.mjs";
import { DataValueBool } from "./data_value_bool.mjs";
export declare abstract class DataValue {
    constructor();
    abstract serialize(): string;
    static deserialize(serializedDataValueType: PropertyType, serializedDataValue: String): DataValue;
    abstract getType(): PropertyType;
}
export declare function dataValueFromString(dataValueType: PropertyType, serializedDataValue: string): DataValue;
export { DataValueBool };
