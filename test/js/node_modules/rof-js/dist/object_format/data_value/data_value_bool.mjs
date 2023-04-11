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
var _DataValueBool_inner;
import { PropertyType } from "../property_type/property_type.mjs";
export class DataValueBool {
    constructor(inner) {
        _DataValueBool_inner.set(this, void 0);
        __classPrivateFieldSet(this, _DataValueBool_inner, inner, "f");
    }
    serialize() {
        switch (__classPrivateFieldGet(this, _DataValueBool_inner, "f")) {
            case true:
                return "true";
            case false:
                return "false";
        }
    }
    static deserialize(serializedDataValueType, serializedBool) {
        if ((!serializedDataValueType.isImplicit() &&
            serializedDataValueType.baseType != "bool") ||
            serializedDataValueType.subTypesIncluded()) {
            return;
        }
        switch (serializedBool) {
            case "true":
                return new DataValueBool(true);
            case "false":
                return new DataValueBool(false);
            default:
                return undefined;
        }
    }
    getType() {
        return PropertyType.simple("bool");
    }
}
_DataValueBool_inner = new WeakMap();
