import { PropertyType } from "../property_type/property_type.mjs";
import { DataValue } from "./data_value.mjs";
export declare class DataValueNumber extends DataValue {
    #private;
    constructor(inner: number | bigint);
    get serialized(): string;
    static deserialize(serializedDataValueType: PropertyType, serializedNumber: string): DataValue | undefined;
    get type(): PropertyType;
}
