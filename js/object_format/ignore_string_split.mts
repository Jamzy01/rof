export class SplitIgnoreRule {
  constructor() {}

  readChar(char: string) {}

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
  #pairChar: string; /* Won't split when within a pair of this character, eg single or double quotation marks */
  #encapsulatesRawText: boolean; // Encapsulates literals
  #withinPair: boolean;

  constructor(char: string) {
    super();

    this.#pairChar = char;
    this.#encapsulatesRawText = false;
    this.#withinPair = false;
  }

  setEncapsulatesRawText(encapsulatesRawText: boolean) {
    this.#encapsulatesRawText = encapsulatesRawText;

    return this;
  }

  readChar(char: string) {
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
    return new PairSplitIgnoreRuleType(this.#pairChar).setEncapsulatesRawText(
      this.#encapsulatesRawText
    );
  }
}

export class NestSplitIgnoreRuleType extends SplitIgnoreRule {
  #nestStartChar;
  #nestEndChar;
  #nestIndex;

  constructor(nestStartChar: string, nestEndChar: string) {
    super();

    this.#nestStartChar = nestStartChar;
    this.#nestEndChar = nestEndChar;
    this.#nestIndex = 0;
  }

  readChar(char: string) {
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

function ignoringCompliantSplitStringMaxSplits(
  inputString: string,
  splitCharacter: string,
  retainBackslashes: boolean,
  ignoreRules: SplitIgnoreRule[],
  maxSplits: number | null
) {
  let stringFragments = [];

  let builtStringFragment = "";

  let splits = 0;

  let oneTimeIgnoreRules = ignoreRules.map((ignoreRule) => ignoreRule.clone());

  for (let i = 0; i < inputString.length; i++) {
    let string_char = inputString[i];

    if (oneTimeIgnoreRules.some((ignoreRule) => ignoreRule.inRawText())) {
      // Only check for rules activating the raw text state

      oneTimeIgnoreRules.forEach((ignoreRule) => {
        if (ignoreRule.inRawText()) {
          ignoreRule.readChar(string_char);
        }
      });
    } else {
      // Check for any rule

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
    } else if (
      string_char == splitCharacter &&
      !oneTimeIgnoreRules.some((ignoreRule) => ignoreRule.shouldIgnore()) &&
      (maxSplits == null || splits < maxSplits)
    ) {
      stringFragments.push(builtStringFragment);

      splits += 1;

      builtStringFragment = "";
    } else {
      builtStringFragment = builtStringFragment + string_char;
    }
  }

  if (builtStringFragment != "") {
    stringFragments.push(builtStringFragment);
  }

  return stringFragments;
}

export function ignoringCompliantSplitString(
  inputString: string,
  splitCharacter: string,
  retainBackslashes: boolean,
  ignoreRules: SplitIgnoreRule[]
) {
  return ignoringCompliantSplitStringMaxSplits(
    inputString,
    splitCharacter,
    retainBackslashes,
    ignoreRules,
    null
  );
}

export function ignoringCompliantSplitOnce(
  inputString: string,
  splitCharacter: string,
  retainBackslashes: boolean,
  ignoreRules: SplitIgnoreRule[]
) {
  let splitString = ignoringCompliantSplitStringMaxSplits(
    inputString,
    splitCharacter,
    retainBackslashes,
    ignoreRules,
    1
  );

  return [splitString[0], splitString[1]];
}

export const ignoreStringSplit = {
  SplitIgnoreRule,
  PairSplitIgnoreRuleType,
  NestSplitIgnoreRuleType,
};
