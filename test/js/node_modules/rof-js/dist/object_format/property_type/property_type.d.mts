export declare class PropertyType {
    #private;
    constructor(baseType: string, subTypes: PropertyType[]);
    serialized(): string;
    static deserialize(serializedPropertyType: string): PropertyType;
    static simple(baseType: string): PropertyType;
    static implicit(): PropertyType;
    static empty(): PropertyType;
    isImplicit(): boolean;
    subTypesIncluded(): boolean;
    get baseType(): string;
    get subTypes(): PropertyType[];
}
