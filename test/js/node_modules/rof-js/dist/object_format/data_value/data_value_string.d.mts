import { PropertyType } from "../property_type/property_type.mjs";
import { DataValue } from "./data_value.mjs";
export declare class DataValueString extends DataValue {
    #private;
    constructor(inner: string);
    get serialized(): string;
    static deserialize(serializedDataValueType: PropertyType, serializedDataValue: string): DataValue | undefined;
    get type(): PropertyType;
}
