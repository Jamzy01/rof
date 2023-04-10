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
var _PairSplitIgnoreRuleType_pairChar, _PairSplitIgnoreRuleType_encapsulatesRawText, _PairSplitIgnoreRuleType_withinPair, _NestSplitIgnoreRuleType_nestStartChar, _NestSplitIgnoreRuleType_nestEndChar, _NestSplitIgnoreRuleType_nestIndex;
export class SplitIgnoreRule {
    constructor() { }
    readChar(char) { }
    shouldIgnore() {
        false;
    }
    inRawText() {
        return false;
    }
    clone() {
        return new SplitIgnoreRule();
    }
}
export class PairSplitIgnoreRuleType extends SplitIgnoreRule {
    constructor(char) {
        super();
        _PairSplitIgnoreRuleType_pairChar.set(this, void 0);
        _PairSplitIgnoreRuleType_encapsulatesRawText.set(this, void 0);
        _PairSplitIgnoreRuleType_withinPair.set(this, void 0);
        __classPrivateFieldSet(this, _PairSplitIgnoreRuleType_pairChar, char, "f");
        __classPrivateFieldSet(this, _PairSplitIgnoreRuleType_encapsulatesRawText, false, "f");
        __classPrivateFieldSet(this, _PairSplitIgnoreRuleType_withinPair, false, "f");
    }
    setEncapsulatesRawText(encapsulatesRawText) {
        __classPrivateFieldSet(this, _PairSplitIgnoreRuleType_encapsulatesRawText, encapsulatesRawText, "f");
        return this;
    }
    readChar(char) {
        if (char == __classPrivateFieldGet(this, _PairSplitIgnoreRuleType_pairChar, "f")) {
            __classPrivateFieldSet(this, _PairSplitIgnoreRuleType_withinPair, !__classPrivateFieldGet(this, _PairSplitIgnoreRuleType_withinPair, "f"), "f");
        }
    }
    shouldIgnore() {
        return __classPrivateFieldGet(this, _PairSplitIgnoreRuleType_withinPair, "f");
    }
    inRawText() {
        return this.shouldIgnore() && __classPrivateFieldGet(this, _PairSplitIgnoreRuleType_encapsulatesRawText, "f");
    }
    clone() {
        return new PairSplitIgnoreRuleType(__classPrivateFieldGet(this, _PairSplitIgnoreRuleType_pairChar, "f")).setEncapsulatesRawText(__classPrivateFieldGet(this, _PairSplitIgnoreRuleType_encapsulatesRawText, "f"));
    }
}
_PairSplitIgnoreRuleType_pairChar = new WeakMap(), _PairSplitIgnoreRuleType_encapsulatesRawText = new WeakMap(), _PairSplitIgnoreRuleType_withinPair = new WeakMap();
export class NestSplitIgnoreRuleType extends SplitIgnoreRule {
    constructor(nestStartChar, nestEndChar) {
        super();
        _NestSplitIgnoreRuleType_nestStartChar.set(this, void 0);
        _NestSplitIgnoreRuleType_nestEndChar.set(this, void 0);
        _NestSplitIgnoreRuleType_nestIndex.set(this, void 0);
        __classPrivateFieldSet(this, _NestSplitIgnoreRuleType_nestStartChar, nestStartChar, "f");
        __classPrivateFieldSet(this, _NestSplitIgnoreRuleType_nestEndChar, nestEndChar, "f");
        __classPrivateFieldSet(this, _NestSplitIgnoreRuleType_nestIndex, 0, "f");
    }
    readChar(char) {
        if (char == __classPrivateFieldGet(this, _NestSplitIgnoreRuleType_nestStartChar, "f")) {
            __classPrivateFieldSet(this, _NestSplitIgnoreRuleType_nestIndex, __classPrivateFieldGet(this, _NestSplitIgnoreRuleType_nestIndex, "f") + 1, "f");
        }
        if (char == __classPrivateFieldGet(this, _NestSplitIgnoreRuleType_nestEndChar, "f")) {
            if (__classPrivateFieldGet(this, _NestSplitIgnoreRuleType_nestIndex, "f") > 0) {
                __classPrivateFieldSet(this, _NestSplitIgnoreRuleType_nestIndex, __classPrivateFieldGet(this, _NestSplitIgnoreRuleType_nestIndex, "f") - 1, "f");
            }
        }
    }
    shouldIgnore() {
        return __classPrivateFieldGet(this, _NestSplitIgnoreRuleType_nestIndex, "f") > 0;
    }
    inRawText() {
        return false;
    }
    clone() {
        return new NestSplitIgnoreRuleType(__classPrivateFieldGet(this, _NestSplitIgnoreRuleType_nestStartChar, "f"), __classPrivateFieldGet(this, _NestSplitIgnoreRuleType_nestEndChar, "f"));
    }
}
_NestSplitIgnoreRuleType_nestStartChar = new WeakMap(), _NestSplitIgnoreRuleType_nestEndChar = new WeakMap(), _NestSplitIgnoreRuleType_nestIndex = new WeakMap();
function ignoringCompliantSplitStringMaxSplits(inputString, splitCharacter, retainBackslashes, ignoreRules, maxSplits) {
    let stringFragments = [];
    let builtStringFragment = "";
    let splits = 0;
    let oneTimeIgnoreRules = ignoreRules.map((ignoreRule) => ignoreRule.clone());
    for (let i = 0; i < inputString.length; i++) {
        let string_char = inputString[i];
        if (oneTimeIgnoreRules.some((ignoreRule) => ignoreRule.inRawText())) {
            oneTimeIgnoreRules.forEach((ignoreRule) => {
                if (ignoreRule.inRawText()) {
                    ignoreRule.readChar(string_char);
                }
            });
        }
        else {
            oneTimeIgnoreRules.forEach((ignoreRule) => {
                ignoreRule.readChar(string_char);
            });
        }
        if (string_char == "\\") {
            i++;
            let escaped_character = inputString[i];
            if (escaped_character == null) {
                break;
            }
            if (retainBackslashes) {
                builtStringFragment = builtStringFragment + "\\";
            }
            builtStringFragment = builtStringFragment + escaped_character;
        }
        else if (string_char == splitCharacter &&
            !oneTimeIgnoreRules.some((ignoreRule) => ignoreRule.shouldIgnore()) &&
            (maxSplits == null || splits < maxSplits)) {
            stringFragments.push(builtStringFragment);
            splits += 1;
            builtStringFragment = "";
        }
        else {
            builtStringFragment = builtStringFragment + string_char;
        }
    }
    if (builtStringFragment != "") {
        stringFragments.push(builtStringFragment);
    }
    return stringFragments;
}
export function ignoringCompliantSplitString(inputString, splitCharacter, retainBackslashes, ignoreRules) {
    return ignoringCompliantSplitStringMaxSplits(inputString, splitCharacter, retainBackslashes, ignoreRules, null);
}
export function ignoringCompliantSplitOnce(inputString, splitCharacter, retainBackslashes, ignoreRules) {
    let splitString = ignoringCompliantSplitStringMaxSplits(inputString, splitCharacter, retainBackslashes, ignoreRules, 1);
    return [splitString[0], splitString[1]];
}
export const ignoreStringSplit = {
    SplitIgnoreRule,
    PairSplitIgnoreRuleType,
    NestSplitIgnoreRuleType,
};
