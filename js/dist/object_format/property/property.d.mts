import { DataValue } from "../data_value/data_value.mjs";
export declare class Property {
    #private;
    constructor(propertyIndex: string, propertyValue: DataValue);
    serialize(): string;
    static deserialize(serialized_property: string): Property;
}
