import { PropertyType } from "../property_type/property_type.mjs";
export declare abstract class DataValue {
    constructor();
    get serialized(): string;
    static deserialize(serializedDataValueType: PropertyType, serializedDataValue: String): DataValue | undefined;
    get type(): PropertyType;
}
