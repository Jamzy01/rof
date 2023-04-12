import {
  NestSplitIgnoreRuleType,
  ignoringCompliantSplitOnce,
  ignoringCompliantSplitString,
} from "../ignore_string_split.mjs";

export class PropertyType {
  #baseType: string;
  #subTypes: PropertyType[];

  constructor(baseType: string, subTypes: PropertyType[]) {
    this.#baseType = baseType;
    this.#subTypes = subTypes;
  }

  serialized(): string {
    if (this.#subTypes.length < 1) {
      // No sub types

      return this.#baseType;
    }

    return `${this.#baseType}<${this.#subTypes.map(subType => subType.serialized).join(", ")}>`;
  }

  static deserialize(serializedPropertyType: string) {
    if (!serializedPropertyType.includes("<")) {
      // No sub types

      return PropertyType.simple(serializedPropertyType.trim());
    }

    let [baseType, serializedSubTypes] = ignoringCompliantSplitOnce(
      serializedPropertyType,
      "<",
      true,
      []
    );

    let subTypes: PropertyType[] = [];

    ignoringCompliantSplitString(serializedSubTypes.slice(0, -1), ",", true, [
      new NestSplitIgnoreRuleType("<", ">"),
    ]).forEach((serializedSubType) => {
      subTypes.push(PropertyType.deserialize(serializedSubType));
    });

    return new PropertyType(baseType.trim(), subTypes);
  }

  static simple(baseType: string) {
    return new PropertyType(baseType, []);
  }

  static implicit() {
    return PropertyType.simple("");
  }

  static empty() {
    return PropertyType.simple("");
  }

  isImplicit() {
    return (
      this.#baseType == "" ||
      ["bool", "char", "string", "struct"].includes(this.#baseType)
    );
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
