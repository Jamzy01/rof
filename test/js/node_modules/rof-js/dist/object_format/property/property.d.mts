import { DataValue } from "../data_value/data_value.mjs";
export declare class Property {
    #private;
    propertyIndex: string;
    constructor(propertyIndex: string, propertyValue: DataValue);
    serialize(): string;
    static deserialize(serializedProperty: string): Property;
    get propertyValue(): DataValue;
}
