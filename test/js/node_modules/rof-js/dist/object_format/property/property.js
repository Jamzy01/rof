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
var _Property_propertyIndex, _Property_propertyValue;
import { DataValueBool, dataValueFromString } from "../data_value/data_value";
import { ignoringCompliantSplitOnce } from "../ignore_string_split.js";
import { PropertyType } from "../property_type/property_type.js";
export class Property {
    constructor(propertyIndex, propertyValue) {
        _Property_propertyIndex.set(this, void 0);
        _Property_propertyValue.set(this, void 0);
        __classPrivateFieldSet(this, _Property_propertyIndex, propertyIndex, "f");
        __classPrivateFieldSet(this, _Property_propertyValue, propertyValue, "f");
    }
    serialize() {
        if (__classPrivateFieldGet(this, _Property_propertyValue, "f").getType().isImplicit()) {
            return `${__classPrivateFieldGet(this, _Property_propertyIndex, "f")} = ${__classPrivateFieldGet(this, _Property_propertyValue, "f").serialize()}`;
        }
        else {
            return `${__classPrivateFieldGet(this, _Property_propertyIndex, "f")}: ${__classPrivateFieldGet(this, _Property_propertyValue, "f")
                .getType()
                .serialize()} = ${__classPrivateFieldGet(this, _Property_propertyValue, "f").serialize()}`;
        }
    }
    static deserialize(serialized_property) {
        let propertyIndex = "";
        let propertyValue = new DataValueBool(false);
        let [rawData, rawValue] = ignoringCompliantSplitOnce(serialized_property.slice(0, -1), "=", true, []);
        if (rawData.includes(":")) {
            let [rawIndex, rawType] = ignoringCompliantSplitOnce(rawData, ":", true, []);
            propertyIndex = rawIndex.trim();
            propertyValue = dataValueFromString(PropertyType.deserialize(rawType), rawValue.trim());
        }
        else {
            propertyIndex = rawData.trim();
            console.log("IMPLICIT TYPE");
            propertyValue = dataValueFromString(PropertyType.implicit(), rawValue.trim());
        }
        if (propertyValue == null) {
            propertyValue = new DataValueBool(false);
        }
        return new Property(propertyIndex, propertyValue);
    }
}
_Property_propertyIndex = new WeakMap(), _Property_propertyValue = new WeakMap();
