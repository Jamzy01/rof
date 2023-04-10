var __classPrivateFieldSet = (this && this.__classPrivateFieldSet) || function (receiver, state, value, kind, f) {
    if (kind === "m") throw new TypeError("Private method is not writable");
    if (kind === "a" && !f) throw new TypeError("Private accessor was defined without a setter");
    if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver)) throw new TypeError("Cannot write private member to an object whose class did not declare it");
    return (kind === "a" ? f.call(receiver, value) : f ? f.value = value : state.set(receiver, value)), value;
};
var __classPrivateFieldGet = (this && this.__classPrivateFieldGet) || function (receiver, state, kind, f) {
    if (kind === "a" && !f) throw new TypeError("Private accessor was defined without a getter");
    if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver)) throw new TypeError("Cannot read private member from an object whose class did not declare it");
    return kind === "m" ? f : kind === "a" ? f.call(receiver) : f ? f.value : state.get(receiver);
};
var _PropertyType_baseType, _PropertyType_subTypes;
import { NestSplitIgnoreRuleType, ignoringCompliantSplitOnce, ignoringCompliantSplitString, } from "../ignore_string_split.js";
export class PropertyType {
    constructor(baseType, subTypes) {
        _PropertyType_baseType.set(this, void 0);
        _PropertyType_subTypes.set(this, void 0);
        __classPrivateFieldSet(this, _PropertyType_baseType, baseType, "f");
        __classPrivateFieldSet(this, _PropertyType_subTypes, subTypes, "f");
    }
    serialize() { }
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
        return (__classPrivateFieldGet(this, _PropertyType_baseType, "f") == "" ||
            ["bool", "char", "string", "struct"].includes(__classPrivateFieldGet(this, _PropertyType_baseType, "f")));
    }
    subTypesIncluded() {
        return __classPrivateFieldGet(this, _PropertyType_subTypes, "f").length > 0;
    }
    get baseType() {
        return __classPrivateFieldGet(this, _PropertyType_baseType, "f");
    }
    get subTypes() {
        return __classPrivateFieldGet(this, _PropertyType_subTypes, "f");
    }
}
_PropertyType_baseType = new WeakMap(), _PropertyType_subTypes = new WeakMap();
