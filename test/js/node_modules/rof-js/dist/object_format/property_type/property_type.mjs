import { NestSplitIgnoreRuleType, ignoringCompliantSplitOnce, ignoringCompliantSplitString, } from "../ignore_string_split.mjs";
export class PropertyType {
    #baseType;
    #subTypes;
    constructor(baseType, subTypes) {
        this.#baseType = baseType;
        this.#subTypes = subTypes;
    }
    serialized() {
        if (this.#subTypes.length < 1) {
            return this.#baseType;
        }
        return `${this.#baseType}<${this.#subTypes.map(subType => subType.serialized).join(", ")}>`;
    }
    static deserialize(serializedPropertyType) {
        if (!serializedPropertyType.includes("<")) {
            return PropertyType.simple(serializedPropertyType.trim());
        }
        let [baseType, serializedSubTypes] = ignoringCompliantSplitOnce(serializedPropertyType, "<", true, []);
        let subTypes = [];
        ignoringCompliantSplitString(serializedSubTypes.slice(0, -1), ",", true, [
            new NestSplitIgnoreRuleType("<", ">"),
        ]).forEach((serializedSubType) => {
            subTypes.push(PropertyType.deserialize(serializedSubType));
        });
        return new PropertyType(baseType.trim(), subTypes);
    }
    static simple(baseType) {
        return new PropertyType(baseType, []);
    }
    static implicit() {
        return PropertyType.simple("");
    }
    static empty() {
        return PropertyType.simple("");
    }
    isImplicit() {
        return (this.#baseType == "" ||
            ["bool", "char", "string", "struct"].includes(this.#baseType));
    }
    subTypesIncluded() {
        return this.#subTypes.length > 0;
    }
    get baseType() {
        return this.#baseType;
    }
    get subTypes() {
        return this.#subTypes;
    }
}
