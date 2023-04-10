export declare class SplitIgnoreRule {
    constructor();
    readChar(char: string): void;
    shouldIgnore(): void;
    inRawText(): boolean;
    clone(): SplitIgnoreRule;
}
export declare class PairSplitIgnoreRuleType extends SplitIgnoreRule {
    #private;
    constructor(char: string);
    setEncapsulatesRawText(encapsulatesRawText: boolean): this;
    readChar(char: string): void;
    shouldIgnore(): boolean;
    inRawText(): boolean;
    clone(): PairSplitIgnoreRuleType;
}
export declare class NestSplitIgnoreRuleType extends SplitIgnoreRule {
    #private;
    constructor(nestStartChar: string, nestEndChar: string);
    readChar(char: string): void;
    shouldIgnore(): boolean;
    inRawText(): boolean;
    clone(): NestSplitIgnoreRuleType;
}
export declare function ignoringCompliantSplitString(inputString: string, splitCharacter: string, retainBackslashes: boolean, ignoreRules: SplitIgnoreRule[]): string[];
export declare function ignoringCompliantSplitOnce(inputString: string, splitCharacter: string, retainBackslashes: boolean, ignoreRules: SplitIgnoreRule[]): string[];
export declare const ignoreStringSplit: {
    SplitIgnoreRule: typeof SplitIgnoreRule;
    PairSplitIgnoreRuleType: typeof PairSplitIgnoreRuleType;
    NestSplitIgnoreRuleType: typeof NestSplitIgnoreRuleType;
};
