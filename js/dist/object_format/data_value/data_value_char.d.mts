import { PropertyType } from "../property_type/property_type.mjs";
import { DataValue } from "./data_value.mjs";
export declare class DataValueCharacter extends DataValue {
    #private;
    constructor(inner: string);
    get serialized(): string;
    static deserialize(serializedDataValueType: PropertyType, serializedDataValueCharacter: string): DataValue | undefined;
    get type(): PropertyType;
}
