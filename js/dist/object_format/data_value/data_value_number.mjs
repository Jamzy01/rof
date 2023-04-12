import { PropertyType } from "../property_type/property_type.mjs";
import { DataValue } from "./data_value.mjs";
export class DataValueNumber extends DataValue {
    #inner;
    constructor(inner) {
        super();
        this.#inner = inner;
    }
    get serialized() {
        return this.#inner.toString();
    }
    static deserialize(serializedDataValueType, serializedNumber) {
        if (!["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "u128", "i128", "usize", "isize", "f32", "f64"].includes(serializedDataValueType.baseType) ||
            serializedDataValueType.subTypesIncluded()) {
            return undefined;
        }
        if (["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "u128", "i128", "usize", "isize"].includes(serializedDataValueType.baseType)) {
            return new DataValueNumber(BigInt(serializedNumber));
        }
        else if (["f32", "f64"].includes(serializedDataValueType.baseType)) {
            return new DataValueNumber(Number(serializedNumber));
        }
        return undefined;
    }
    get type() {
        switch (typeof this.#inner) {
            case "number":
                return PropertyType.simple("f64");
            case "bigint":
                if (this.#inner > 9_223_372_036_854_775_807) {
                    return PropertyType.simple("u128");
                }
                else if (this.#inner < -9_223_372_036_854_775_807) {
                    return PropertyType.simple("i128");
                }
                else {
                    return PropertyType.simple("i64");
                }
        }
    }
}
