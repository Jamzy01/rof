import { NestSplitIgnoreRuleType, PairSplitIgnoreRuleType, SplitIgnoreRule, ignoringCompliantSplitString } from "../ignore_string_split.mjs";
import { PropertyType } from "../property_type/property_type.mjs";
import { DataValue } from "./data_value.mjs";
import { dataValueFromString } from "./universal_data_value_deserializer.mjs";

export class DataValueArray extends DataValue {
    #objects: DataValue[];

    constructor(objects: DataValue[]) {
        super();

        this.#objects = objects;
    }

    get serialized(): string {
        return `[${this.#objects.join(", ")}]`;
    }

    static deserialize(serializedDataValueType: PropertyType, serializedArray: String): DataValue | undefined {
        if (serializedDataValueType.baseType != "array") {
            return undefined;
        }

        if (!(serializedArray.startsWith("[") && serializedArray.endsWith("]"))) {
            return undefined;
        }

        let arrayObjectType: PropertyType = serializedDataValueType.subTypes[0];

        let objects: DataValue[] = [];

        ignoringCompliantSplitString(serializedArray.slice(1, -1).trim(), ",", true, [
            new PairSplitIgnoreRuleType('"').setEncapsulatesRawText(true),
            new PairSplitIgnoreRuleType("'").setEncapsulatesRawText(true),
            new NestSplitIgnoreRuleType("[", "]"),
            new NestSplitIgnoreRuleType("{", "}")
        ]).forEach(serializedProperty => {
            if (serializedProperty.trim().length < 1) {
                return;
            }

            let deserializedProperty = dataValueFromString(arrayObjectType, serializedProperty.trim());

            if (deserializedProperty instanceof DataValue) {
                objects.push(deserializedProperty);
            }
        });

        return new DataValueArray(objects);
    }
}