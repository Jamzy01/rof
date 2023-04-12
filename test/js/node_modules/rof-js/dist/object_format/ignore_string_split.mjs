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
    #pairChar;
    #encapsulatesRawText;
    #withinPair;
    constructor(char) {
        super();
        this.#pairChar = char;
        this.#encapsulatesRawText = false;
        this.#withinPair = false;
    }
    setEncapsulatesRawText(encapsulatesRawText) {
        this.#encapsulatesRawText = encapsulatesRawText;
        return this;
    }
    readChar(char) {
        if (char == this.#pairChar) {
            this.#withinPair = !this.#withinPair;
        }
    }
    shouldIgnore() {
        return this.#withinPair;
    }
    inRawText() {
        return this.shouldIgnore() && this.#encapsulatesRawText;
    }
    clone() {
        return new PairSplitIgnoreRuleType(this.#pairChar).setEncapsulatesRawText(this.#encapsulatesRawText);
    }
}
export class NestSplitIgnoreRuleType extends SplitIgnoreRule {
    #nestStartChar;
    #nestEndChar;
    #nestIndex;
    constructor(nestStartChar, nestEndChar) {
        super();
        this.#nestStartChar = nestStartChar;
        this.#nestEndChar = nestEndChar;
        this.#nestIndex = 0;
    }
    readChar(char) {
        if (char == this.#nestStartChar) {
            this.#nestIndex += 1;
        }
        if (char == this.#nestEndChar) {
            if (this.#nestIndex > 0) {
                this.#nestIndex -= 1;
            }
        }
    }
    shouldIgnore() {
        return this.#nestIndex > 0;
    }
    inRawText() {
        return false;
    }
    clone() {
        return new NestSplitIgnoreRuleType(this.#nestStartChar, this.#nestEndChar);
    }
}
function ignoringCompliantSplitStringMaxSplits(inputString, splitCharacter, retainBackslashes, ignoreRules, maxSplits) {
    let stringFragments = [];
    let builtStringFragment = "";
    let splits = 0;
    let oneTimeIgnoreRules = ignoreRules.map((ignoreRule) => ignoreRule.clone());
    for (let i = 0; i < inputString.length; i++) {
        let stringChar = inputString[i];
        if (oneTimeIgnoreRules.some((ignoreRule) => ignoreRule.inRawText())) {
            oneTimeIgnoreRules.forEach((ignoreRule) => {
                if (ignoreRule.inRawText()) {
                    ignoreRule.readChar(stringChar);
                }
            });
        }
        else {
            oneTimeIgnoreRules.forEach((ignoreRule) => {
                ignoreRule.readChar(stringChar);
            });
        }
        if (stringChar == "\\") {
            i++;
            let escapedChar = inputString[i];
            if (escapedChar == null) {
                break;
            }
            if (retainBackslashes) {
                builtStringFragment = builtStringFragment + "\\";
            }
            builtStringFragment = builtStringFragment + escapedChar;
        }
        else if (stringChar == splitCharacter &&
            !oneTimeIgnoreRules.some((ignoreRule) => ignoreRule.shouldIgnore()) &&
            (maxSplits == null || splits < maxSplits)) {
            stringFragments.push(builtStringFragment);
            splits += 1;
            builtStringFragment = "";
        }
        else {
            builtStringFragment = builtStringFragment + stringChar;
        }
    }
    if (builtStringFragment.length > 0) {
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
