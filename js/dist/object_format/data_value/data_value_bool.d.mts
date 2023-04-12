import { PropertyType } from "../property_type/property_type.mjs";
import { DataValue } from "./data_value.mjs";
export declare class DataValueBool extends DataValue {
    #private;
    constructor(inner: boolean);
    get serialized(): string;
    static deserialize(serializedDataValueType: PropertyType, serializedBool: string): DataValue | undefined;
    get type(): PropertyType;
}
