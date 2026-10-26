//#region \0rolldown/runtime.js
var __defProp = Object.defineProperty;
var __exportAll = (all, no_symbols) => {
	let target = {};
	for (var name in all) __defProp(target, name, {
		get: all[name],
		enumerable: true
	});
	if (!no_symbols) __defProp(target, Symbol.toStringTag, { value: "Module" });
	return target;
};
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/version.js
const version = "1.0.0-beta.16";
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/utils/misc.js
const hasOwn = Object.hasOwn || ((subject, key) => Object.hasOwnProperty.call(subject, key));
const toString = Object.prototype.toString;
function addToSet(set, value) {
	if (value !== void 0) {
		if (isArray(value)) value.forEach((item) => set.add(item));
		else set.add(value);
	}
	return set;
}
function addToMapSet(map, key, value) {
	if (map.has(key)) map.get(key).add(value);
	else map.set(key, /* @__PURE__ */ new Set([value]));
}
function getPropertyValue(value, property) {
	return value && hasOwn(value, property) ? value[property] : void 0;
}
function isPlainObject(value) {
	return value !== null && typeof value === "object" && value.constructor === Object;
}
function isRegExp(value) {
	return toString.call(value) === "[object RegExp]";
}
function isArrayLike(value) {
	return value && (isArray(value) || hasOwn(value, "length") && isFinite(value.length));
}
function isArray(value) {
	return Array.isArray(value) || ArrayBuffer.isView(value);
}
function parseIntDefault(value, defaultValue = 0) {
	const int = parseInt(value, 10);
	return !isNaN(int) ? int : defaultValue;
}
function isTruthy(value) {
	if (isArray(value)) return value.length > 0;
	if (isPlainObject(value)) {
		for (const key in value) if (hasOwn(value, key)) return true;
		return false;
	}
	return Boolean(value);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/parser/convert-to-literal.js
function toNumberLiteral(value, throwError) {
	const isHex = value.startsWith("0x") || value.startsWith("0X");
	if (value.includes("_")) {
		const errorMatch = value.match(isHex ? /(?:^|[^0-9a-fA-F])_|_(?:[^0-9a-fA-F]|$)/ : /(?:^|\D)_|_(?:\D|$)/);
		if (errorMatch) {
			const m = errorMatch[0];
			const index = errorMatch.index + Number(m[1] === "_");
			throwError(`${m === "__" ? "Only one underscore is allowed" : "Wrong underscore"} as numeric separator`, { inside: [index, 1] });
		}
		value = value.replace(/_/g, "");
	}
	return isHex ? parseInt(value, 16) : parseFloat(value);
}
const ESCAPE_REPLACE = {
	"0": "\0",
	"b": "\b",
	"n": "\n",
	"r": "\r",
	"f": "\f",
	"t": "	",
	"v": "\v"
};
function toStringLiteral(value, multiline = false, end = 1, throwError) {
	return value.slice(1, value.length - end).replace(/\\(?:(\r\n?|\n|\u2028|\u2029)|([xu][0-9a-fA-F]*)|$|(.))|[\r\n\u2028\u2029]/g, (match, lineTerminator, escapeHex, other, offset) => {
		if (lineTerminator) return "";
		if (escapeHex) {
			const isUnicode = escapeHex[0] === "u";
			if (escapeHex.length === (isUnicode ? 5 : 3)) return String.fromCharCode(parseInt(escapeHex.slice(1), 16));
			throwError(`Invalid ${isUnicode ? "Unicode" : "hexadecimal"} escape sequence`, { inside: [offset + 1, match.length] });
		}
		if (match === "\\") throwError("Invalid backslash", { inside: [offset + 1, 1] });
		if (other) return ESCAPE_REPLACE[other] || other;
		if (!multiline && /[\r\n\u2028\u2029]/.test(match)) throwError("Invalid line terminator", { inside: [offset + 1, 1] });
		return match;
	});
}
function toRegExpLiteral(value, throwError) {
	const flagsMatch = value.match(/[^/]*$/) || { index: value.length };
	for (let i = flagsMatch.index; i < value.length; i++) {
		const duplicateIndex = value.indexOf(value[i], i + 1);
		if (duplicateIndex !== -1) throwError("Duplicate flag in regexp", { inside: [duplicateIndex, 1] });
	}
	return new RegExp(value.slice(1, flagsMatch.index - 1), value.slice(flagsMatch.index));
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/parser/error.js
const newlineRx = /\n|\r\n?|\u2028|\u2029/g;
function showPosition(input, offset) {
	const span = 32;
	const start = Math.max(offset - span, 0);
	const end = Math.min(offset + span, input.length);
	const pre = (start === 0 ? "" : "...") + input.slice(start, offset).replace(newlineRx, "\\n");
	return `${pre}${input.slice(offset, end).replace(newlineRx, "\\n") + (end === input.length ? "" : "...")}\n${"-".repeat(pre.length)}^`;
}
function offsetToLoc(input, offset) {
	const lines = input.slice(0, offset).split(newlineRx);
	return {
		line: lines.length,
		column: lines[lines.length - 1].length,
		offset
	};
}
const TOKEN_MAX = 59;
const tokenNames = {
	[0]: "NUMBER",
	[1]: "STRING",
	[2]: "REGEXP",
	[3]: "LITERAL",
	[4]: "IDENT",
	[5]: "$IDENT",
	[6]: "@",
	[7]: "#",
	[8]: "$",
	[9]: "$$",
	[10]: "AND",
	[11]: "OR",
	[12]: "NOT",
	[13]: "NO",
	[14]: "IS",
	[15]: "IN",
	[16]: "NOTIN",
	[17]: "HAS",
	[18]: "HASNO",
	[19]: "ORDER",
	[20]: "METHOD(",
	[21]: "$METHOD(",
	[22]: "TEMPLATE",
	[23]: "TPL_START",
	[24]: "TPL_CONTINUE",
	[25]: "TPL_END",
	[26]: ".",
	[27]: "..",
	[28]: "...",
	[29]: ".(",
	[30]: ".[",
	[31]: "..(",
	[32]: "|",
	[33]: "=>",
	[34]: "=",
	[35]: "!=",
	[36]: "~=",
	[37]: "<",
	[38]: "<=",
	[39]: ">",
	[40]: ">=",
	[41]: "+",
	[42]: "-",
	[43]: "*",
	[44]: "/",
	[45]: "%",
	[46]: "??",
	[47]: "?",
	[48]: "(",
	[49]: ")",
	[50]: "[",
	[51]: "]",
	[52]: "{",
	[53]: "}",
	[54]: ",",
	[55]: ":",
	[56]: ";",
	[57]: "BAD_TOKEN",
	[58]: "EOF"
};
const BALANCE_TOKEN_PAIR = createTokenTable(48, 49, 29, 49, 31, 49, 20, 49, 21, 49, 50, 51, 30, 51, 52, 53, 23, 25);
const KEYWORDS = {
	"has no": 18,
	"not in": 16,
	"and": 10,
	"or": 11,
	"not": 12,
	"has": 17,
	"is": 14,
	"in": 15,
	"no": 13,
	"asc": 19,
	"ascN": 19,
	"ascA": 19,
	"ascNA": 19,
	"ascAN": 19,
	"desc": 19,
	"descN": 19,
	"descA": 19,
	"descNA": 19,
	"descAN": 19
};
const STR_TO_TOKEN = {
	"...": 28,
	"..(": 31,
	"..": 27,
	".(": 29,
	".[": 30,
	"=>": 33,
	"!=": 35,
	"~=": 36,
	"<=": 38,
	">=": 40,
	"??": 46,
	"@": 6,
	"#": 7,
	".": 26,
	"|": 32,
	"=": 34,
	"<": 37,
	">": 39,
	"+": 41,
	"-": 42,
	"*": 43,
	"/": 44,
	"%": 45,
	"?": 47,
	"(": 48,
	")": 49,
	"[": 50,
	"]": 51,
	"{": 52,
	"}": 53,
	",": 54,
	":": 55,
	";": 56
};
const KEYWORD_TOKENS = createTokenSet(10, 11, 12, 13, 15, 16, 17, 18, 14, 19);
const PREVENT_KEYWORD = createTokenSet(26, 27);
const PREVENT_PRIMITIVE = createTokenSet(0, 1, 22, 25, 2, 3, 4, 9, 5, 8, 6, 7, 49, 51, 53, 26);
function createTokenSet(...tokens) {
	const set = new Int8Array(TOKEN_MAX);
	for (const token of tokens) set[token] = 1;
	return set;
}
function createTokenTable(...pairs) {
	const table = new Int8Array(TOKEN_MAX).fill(-1);
	for (let i = 0; i < pairs.length; i += 2) table[pairs[i]] = pairs[i + 1];
	return table;
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/parser/tokenizer-tolerant-token-pairs.js
const keywords = [
	10,
	11,
	15,
	16,
	17,
	18,
	14
];
const operators = [
	41,
	42,
	43,
	44,
	45,
	32,
	34,
	35,
	36,
	40,
	38,
	37,
	39
];
const baseTokens = [
	54,
	47,
	55,
	56,
	58,
	51,
	49,
	53,
	24,
	25,
	19,
	...operators,
	...keywords
];
const commonNextTokens = createTokenSet(...baseTokens);
const emptyBlockNextTokens = createTokenSet(...baseTokens);
emptyBlockNextTokens[49] = 0;
const emptyBracketNextTokens = createTokenSet(...baseTokens);
emptyBracketNextTokens[51] = 0;
const dollarNextTokens = createTokenSet(54, 47, 55, 24, 25, 19, ...operators, ...keywords);
[
	32,
	34,
	41,
	42,
	43,
	44,
	45,
	15,
	17,
	16,
	18
].forEach((token) => dollarNextTokens[token] = 0);
const TOLERANT_TOKEN_PAIRS = new Map([
	[null, commonNextTokens],
	[47, commonNextTokens],
	[55, commonNextTokens],
	[56, commonNextTokens],
	[54, commonNextTokens],
	[26, commonNextTokens],
	[27, commonNextTokens],
	[48, commonNextTokens],
	[33, commonNextTokens],
	[12, commonNextTokens],
	[13, commonNextTokens],
	[29, emptyBlockNextTokens],
	[31, emptyBlockNextTokens],
	[20, emptyBlockNextTokens],
	[21, emptyBlockNextTokens],
	[50, emptyBracketNextTokens],
	[30, emptyBracketNextTokens],
	[8, dollarNextTokens],
	...operators.map((op) => [op, commonNextTokens]),
	...keywords.map((kw) => [kw, commonNextTokens])
]);
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/parser/tokenizer.js
const COMMENT_RX = /\/\/.*?(?:\n|\r\n?|\u2028|\u2029|$)|\/\*(?:.|\s)*?(?:\*\/|$)/y;
const WHITESPACE_RX = /\s+/y;
const REST_TOKENS_RX = new RegExp(Object.keys(STR_TO_TOKEN).map((s) => s.replace(/[\[\]{}()^$|?*+.]/g, "\\$&")).join("|"), "y");
const IDENTIFIER_RX = /(?:(?:[a-zA-Z_]|\\u[0-9a-fA-F]{4})(?:[a-zA-Z_$0-9]|\\u[0-9a-fA-F]{4})*)\b/y;
const KEYWORD_RX = /(?:has no|not in|and|or|not|has|is|in|no|(?:asc|desc)(?:NA?|AN?)?)\b/y;
const REGEXP_RX = /\/(?:\\.|[^/])+\/[gimsu]*/y;
const NUMBER_RX = /(?:[_\d]*\.)?[_\d]+(?:[eE][-+]?[_\d]+)?\b|0[xX][_0-9a-fA-F]+\b/y;
const STRING_RX = /"(?:\\.|[^"])*"|'(?:\\.|[^'])*'/sy;
const TEMPLATE_START_RX = /`(?:\\.|[^`\\$]|\$(?!\{))*(\$\{|`)/sy;
const TEMPLATE_CONT_END_RX = /\}(?:\\.|[^`\\$]|\$(?!\{))*(\$\{|`)/sy;
const NOOP = () => {};
const LITERALS = /* @__PURE__ */ new Map([
	["true", true],
	["false", false],
	["null", null],
	["undefined", void 0],
	["Infinity", Infinity],
	["NaN", NaN]
]);
var Token = class {
	#input;
	#value;
	constructor(type, start, end, input, value = null) {
		this.type = type;
		this.start = start;
		this.end = end;
		this.#input = input;
		this.#value = value;
	}
	get name() {
		return tokenNames[this.type];
	}
	get value() {
		return this.#value ?? (this.#value = this.#input.slice(this.start, this.end));
	}
};
function* tokenize(input, tolerant = false) {
	const nextToken = createTokenizer(input, tolerant);
	let token;
	while (token = nextToken()) yield token;
}
function tokenizeSync(input, tolerant = false) {
	const nextToken = createTokenizer(input, tolerant);
	const tokens = [];
	let token;
	while (token = nextToken()) tokens.push(token);
	return tokens;
}
function createTokenizer(input, options) {
	const { tolerant: tolerantMode = false, onComment: onCommentHandler } = typeof options === "boolean" ? { tolerant: options } : options || {};
	const onComment = typeof onCommentHandler === "function" ? onCommentHandler : Array.isArray(onCommentHandler) ? (start, end) => onCommentHandler.push([start, end]) : NOOP;
	const length = input.length;
	let pos = 0;
	let done = false;
	let bracketStack = [];
	let preventPrimitive = false;
	let preventKeyword = false;
	let pendingToken = null;
	let prevTokenType = null;
	let prevTokenEnd = 0;
	function advance(count = 1) {
		pos = Math.min(pos + count, length);
	}
	function match(regexp) {
		regexp.lastIndex = pos;
		const match = regexp.exec(input);
		if (match) {
			const value = match[0];
			advance(value.length);
			return value;
		}
		return null;
	}
	function peek() {
		return pos < length ? input[pos] : "";
	}
	function peekNext() {
		const index = pos + 1;
		return index < length ? input[index] : "";
	}
	function isIdentStart(ch) {
		return ch >= "A" && ch <= "Z" || ch >= "a" && ch <= "z" || ch === "_" || ch === "\\" && peekNext() === "u";
	}
	function isNumberStart(ch) {
		if (ch >= "0" && ch <= "9") return true;
		if (ch === ".") {
			const next = peekNext();
			return next >= "0" && next <= "9" || next === "_";
		}
		return false;
	}
	function trackBracketBalance(tokenType) {
		if (BALANCE_TOKEN_PAIR[tokenType] !== -1) bracketStack.push(BALANCE_TOKEN_PAIR[tokenType]);
		else if (bracketBalanceTop() === tokenType) bracketStack.pop();
	}
	function bracketBalanceTop() {
		return bracketStack.length > 0 ? bracketStack[bracketStack.length - 1] : -1;
	}
	function nextTokenTolerant(token) {
		if (TOLERANT_TOKEN_PAIRS.get(prevTokenType)?.[token.type]) {
			pendingToken = token;
			let emptyStart;
			let emptyEnd;
			if (prevTokenType === null) {
				emptyStart = 0;
				emptyEnd = KEYWORD_TOKENS[token.type] ? Math.max(0, token.start - 1) : token.start;
			} else if (KEYWORD_TOKENS[prevTokenType]) {
				emptyStart = token.start;
				emptyEnd = token.start;
			} else if (KEYWORD_TOKENS[token.type]) {
				emptyStart = prevTokenEnd;
				emptyEnd = Math.max(prevTokenEnd, token.start - 1);
			} else {
				emptyStart = prevTokenEnd;
				emptyEnd = token.start;
			}
			return new Token(4, emptyStart, emptyEnd, input, "");
		}
		prevTokenType = token.type;
		prevTokenEnd = token.end;
		return token;
	}
	function nextTokenStrict() {
		if (pos >= length) {
			done = true;
			return 58;
		}
		const start = pos;
		const ch = input[pos];
		if (isNumberStart(ch) && match(NUMBER_RX)) return 0;
		if ((ch === "\"" || ch === "'") && match(STRING_RX)) return 1;
		if (ch === "`" && match(TEMPLATE_START_RX)) return input[pos - 1] == "`" ? 22 : 23;
		if (ch === "}" && bracketBalanceTop() === 25 && match(TEMPLATE_CONT_END_RX)) return input[pos - 1] === "`" ? 25 : 24;
		if (ch === "/") {
			if (!preventPrimitive && match(REGEXP_RX)) return 2;
			advance();
			return 44;
		}
		if (isIdentStart(ch)) {
			if (!preventKeyword && match(KEYWORD_RX)) return KEYWORDS[input.slice(start, pos)];
			if (match(IDENTIFIER_RX)) {
				if (!preventKeyword && LITERALS.has(input.slice(start, pos))) return 3;
				if (peek() === "(") {
					advance();
					return 20;
				}
				return 4;
			}
		}
		if (ch === "$") {
			advance();
			if (peek() === "$") {
				advance();
				return 9;
			}
			if (match(IDENTIFIER_RX)) {
				if (peek() === "(") {
					advance();
					return 21;
				}
				return 5;
			}
			return 8;
		}
		if (match(REST_TOKENS_RX)) return STR_TO_TOKEN[input.slice(start, pos)];
		pos++;
		return 57;
	}
	function scanComment() {
		const start = pos;
		if (peek() === "/" && match(COMMENT_RX)) {
			onComment(start, pos, input);
			return true;
		}
		return false;
	}
	return function nextToken() {
		if (pendingToken) {
			const token = pendingToken;
			pendingToken = null;
			prevTokenType = token.type;
			prevTokenEnd = token.end;
			return token;
		}
		if (done) return null;
		do
			if (match(WHITESPACE_RX)) preventKeyword = false;
		while (scanComment());
		const start = pos;
		const nextTokenType = nextTokenStrict();
		const token = new Token(nextTokenType, start, pos, input);
		preventPrimitive = PREVENT_PRIMITIVE[nextTokenType];
		preventKeyword = PREVENT_KEYWORD[nextTokenType];
		trackBracketBalance(nextTokenType);
		return tolerantMode ? nextTokenTolerant(token) : token;
	};
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/parser/parser.js
const RECOVERABLE_ERROR = Symbol("RECOVERABLE_ERROR");
const TOKEN_ANY = -1;
const SPREAD_ARRAY = true;
const SPREAD_OBJECT = false;
const RIGHT_ASSOCIATIVE = createTokenSet(47);
const PRECEDENCE = createTokenTable(19, 1, 32, 2, 47, 3, 14, 4, 11, 5, 10, 6, 46, 7, 12, 8, 13, 8, 15, 9, 16, 9, 17, 9, 18, 9, 34, 10, 35, 10, 36, 10, 37, 11, 38, 11, 39, 11, 40, 11, 41, 12, 42, 12, 43, 12, 44, 12, 45, 12);
function parse(input, options) {
	const tokens = tokenizeSync(input, options);
	let index = 0;
	let current = tokens[index];
	let recoverable = false;
	let result = null;
	try {
		return result = parseBlock();
	} finally {
		if (result) advance(58);
	}
	function throwError(rawMessage, details) {
		if (recoverable) throw RECOVERABLE_ERROR;
		let start = current.start;
		let end = current.end;
		details ??= {};
		if (Array.isArray(details?.inside)) {
			start += details.inside[0];
			end = start + details.inside[1];
		}
		if (current.type === 57) rawMessage = `Bad input on line ${offsetToLoc(input, start).line} column ${offsetToLoc(input, start).column}`;
		const startLoc = offsetToLoc(input, start);
		const message = current.type === 57 || details?.inside ? [
			rawMessage,
			"",
			showPosition(input, start)
		] : [
			`Parse error on line ${startLoc.line}:`,
			"",
			showPosition(input, start),
			...rawMessage ? ["", rawMessage] : []
		];
		const expected = current.type === 57 || !Array.isArray(details?.expected) ? null : [...new Set([].concat(...details.expected).map((t) => `'${tokenNames[t]}'`))];
		if (expected) message.push("", "Expecting " + expected.join(", ") + " got " + tokenNames[current.type]);
		throw Object.assign(new SyntaxError(message.join("\n")), { details: {
			rawMessage,
			text: current.value,
			token: current.name,
			expected,
			loc: {
				range: [start, end],
				start: startLoc,
				end: offsetToLoc(input, end)
			}
		} });
	}
	function throwExpected(...expected) {
		throwError(null, { expected });
	}
	function throwIfNoMatch(expectedType = TOKEN_ANY) {
		if (expectedType !== TOKEN_ANY && current.type !== expectedType) throwExpected(expectedType);
	}
	function disableRecovery(fn) {
		const savedRecoverable = recoverable;
		recoverable = false;
		const result = fn();
		recoverable = savedRecoverable;
		return result;
	}
	function maybe(fn, ...args) {
		const savedRecoverable = recoverable;
		const savedIndex = index;
		try {
			recoverable = true;
			return fn(...args);
		} catch (error) {
			if (error !== RECOVERABLE_ERROR) throw error;
			index = savedIndex;
			current = tokens[index];
			return null;
		} finally {
			recoverable = savedRecoverable;
		}
	}
	function advance(expectedType) {
		const token = current;
		throwIfNoMatch(expectedType);
		index = Math.min(index + 1, tokens.length - 1);
		current = tokens[index];
		return token;
	}
	function advanceIfMatch(type) {
		return match(type) ? advance() : null;
	}
	function getValueAndAdvance(expectedType) {
		const value = current.value;
		advance(expectedType);
		return value;
	}
	function getRangeAndAdvance(expectedType) {
		const range = [current.start, current.end];
		advance(expectedType);
		return range;
	}
	function consumeSurrounded(typeOpen, fn, typeClose) {
		advance(typeOpen);
		const value = fn();
		advance(typeClose);
		return value;
	}
	function match(type) {
		return current.type === type;
	}
	function nextMatch(type) {
		return (index < tokens.length - 1 ? tokens[index + 1].type : 58) === type;
	}
	function startRange() {
		return current.start;
	}
	function endRange(start) {
		const end = index > 0 ? tokens[index - 1].end : 0;
		if (start > end) return [end, end];
		return [start, end];
	}
	function legacyPrefixStartRange(node) {
		return node?.range[0] ?? startRange();
	}
	function createPlaceholder() {
		const pos = index > 0 ? tokens[index - 1].end : 0;
		return {
			type: "Placeholder",
			range: [pos, pos]
		};
	}
	function maybeBlock(definitions, body, start) {
		return definitions !== null ? createBlock(definitions, body, endRange(start)) : body;
	}
	function createBlock(definitions, body, range) {
		return {
			type: "Block",
			definitions,
			body,
			range
		};
	}
	function createMethod(reference, args, range) {
		return {
			type: "Method",
			reference,
			arguments: args,
			range
		};
	}
	function parseBlock() {
		const start = startRange();
		return createBlock(parseDefinitions() || [], parseExpression() || createPlaceholder(), endRange(start));
	}
	function parseDefinitions() {
		let definitions = null;
		for (let definition; definition = maybeDefinition();) if (definitions === null) definitions = [definition];
		else definitions.push(definition);
		return definitions;
	}
	function maybeDefinition() {
		if (match(8) || match(5)) return maybe(parseDefinition);
		return null;
	}
	function parseDefinition() {
		const start = startRange();
		const declarator = parseDeclarator();
		const value = advanceIfMatch(55) ? disableRecovery(parseExpression) : null;
		advance(56);
		return {
			type: "Definition",
			declarator,
			value,
			range: endRange(start)
		};
	}
	function parseDeclarator() {
		const start = startRange();
		return {
			type: "Declarator",
			name: match(5) ? getValueAndAdvance().slice(1) : advanceIfMatch(8) ? null : throwExpected(5, 8),
			range: endRange(start)
		};
	}
	function parseIdentifier(preserveDollar = false) {
		const { start, end, type, value } = current;
		let name = getValueAndAdvance();
		if (!preserveDollar) {
			if (type === 5 || type === 21) name = name.slice(1);
		}
		if (type === 20 || type === 21) name = name.slice(0, -1);
		return {
			type: "Identifier",
			name,
			range: [start, end - !value.endsWith(name)]
		};
	}
	function parseReference() {
		const name = parseIdentifier();
		return {
			type: "Reference",
			name,
			range: name.range.slice()
		};
	}
	function parseSpecialReference() {
		switch (current.type) {
			case 6: return {
				type: "Data",
				range: getRangeAndAdvance()
			};
			case 7: return {
				type: "Context",
				range: getRangeAndAdvance()
			};
			case 8: return {
				type: "Current",
				range: getRangeAndAdvance()
			};
			case 9: return {
				type: "Arg1",
				range: getRangeAndAdvance()
			};
			default: throwExpected(6, 7, 8, 9);
		}
	}
	function parseLiteralValue() {
		const start = startRange();
		let value;
		switch (current.type) {
			case 0:
				value = toNumberLiteral(current.value, throwError);
				break;
			case 1:
				value = toStringLiteral(current.value, false, 1, throwError);
				break;
			case 22:
			case 25:
				value = toStringLiteral(current.value, true, 1, throwError);
				break;
			case 23:
			case 24:
				value = toStringLiteral(current.value, true, 2, throwError);
				break;
			case 2:
				value = toRegExpLiteral(current.value, throwError);
				break;
			case 3:
				value = LITERALS.get(current.value);
				break;
			default: throwExpected(0, 1, 2, 3, 22, 23, 24, 25);
		}
		advance();
		return {
			type: "Literal",
			value,
			range: endRange(start)
		};
	}
	function parseExpression(minPrec = 0) {
		let left = parseUnary();
		while (PRECEDENCE[current.type] >= minPrec && !match(58)) {
			const prec = PRECEDENCE[current.type] + !RIGHT_ASSOCIATIVE[current.type];
			switch (current.type) {
				case 47:
					left = parseTernaryConditional(left, prec);
					break;
				case 32:
					left = parsePipeline(left);
					break;
				case 19:
					left = parseCompareFunction(left);
					break;
				default: left = parseBinaryOperator(left, prec);
			}
		}
		return left;
	}
	function parseUnary() {
		switch (current.type) {
			case 12:
			case 13:
			case 41:
			case 42:
			case 14: return parseUnaryPrefix();
			default: return parsePostfix();
		}
	}
	function parseUnaryPrefix() {
		const start = startRange();
		const prec = PRECEDENCE[current.type];
		const operator = getValueAndAdvance();
		return {
			type: "Prefix",
			operator,
			argument: operator === "is" ? parseAssertion() : parseExpression(prec + 1),
			range: endRange(start)
		};
	}
	function parsePostfix() {
		let expr = parsePrimary();
		while (!match(58)) switch (current.type) {
			case 14:
				expr = parseAssertionPostfix(expr);
				break;
			case 26:
				expr = nextMatch(4) ? parseGetProperty(expr, true) : nextMatch(20) || nextMatch(21) ? parseMethodCall(expr, true) : throwExpected(4, 20, 21);
				break;
			case 29:
				expr = parseMap(expr);
				break;
			case 30:
				expr = parseFilter(expr);
				break;
			case 27:
			case 31:
				expr = parseMapRecursive(expr);
				break;
			case 50:
				expr = maybe(parseSliceNotation, expr) || parseBracketAccess(expr);
				break;
			default: return expr;
		}
		return expr;
	}
	function parsePrimary() {
		switch (current.type) {
			case 0:
			case 1:
			case 2:
			case 3: return parseLiteralValue();
			case 22:
			case 23: return parseTemplate();
			case 6:
			case 7:
			case 8:
			case 9: return parseSpecialReference();
			case 5: return maybeFunction() || parseReference();
			case 4: return parseGetProperty();
			case 20:
			case 21: return parseMethodCall();
			case 50: return maybe(parseSliceNotation) || parseArray();
			case 52: return parseObject();
			case 48: return maybeFunction() || parseParentheses();
			case 33: return parseFunction();
			default: return null;
		}
	}
	function parseAssertionPostfix(argument) {
		const start = legacyPrefixStartRange(argument);
		advance(14);
		return {
			type: "Postfix",
			operator: parseAssertion(),
			argument,
			range: endRange(start)
		};
	}
	function parseAssertion() {
		const start = startRange();
		const negation = advanceIfMatch(12) !== null;
		let assertion = [];
		switch (current.type) {
			case 48:
				advance(48);
				do {
					if (assertion.length) assertion.push(match(10) || match(11) ? getValueAndAdvance() : throwExpected(10, 11));
					assertion.push(parseAssertion());
				} while (!advanceIfMatch(49));
				break;
			case 4:
			case 3:
				assertion = parseIdentifier();
				break;
			case 5:
				assertion = createMethod(parseReference(), [], endRange(start));
				break;
			default: throwExpected(48, 4, 3, 5);
		}
		return {
			type: "Assertion",
			negation,
			assertion,
			range: endRange(start)
		};
	}
	function parseGetProperty(value = null, prefixed = false) {
		const start = legacyPrefixStartRange(value);
		if (prefixed) advance(26);
		return {
			type: "GetProperty",
			value,
			property: parseIdentifier(),
			range: endRange(start)
		};
	}
	function parseMethod() {
		const start = startRange();
		const methodRef = match(20) ? parseIdentifier() : match(21) ? parseReference() : throwExpected(20, 21);
		const args = [];
		if (!match(49)) do
			args.push(parseExpression());
		while (advanceIfMatch(54));
		advance(49);
		return createMethod(methodRef, args, endRange(start));
	}
	function parseMethodCall(value = null, prefixed = false) {
		const start = legacyPrefixStartRange(value);
		if (prefixed) advance(26);
		return {
			type: "MethodCall",
			value,
			method: parseMethod(),
			range: endRange(start)
		};
	}
	function parseTemplate() {
		const start = startRange();
		const values = [];
		if (match(22)) values.push(parseLiteralValue());
		else {
			values.push(parseLiteralValue(23));
			while (true) {
				values.push(parseExpression());
				if (!match(24)) break;
				values.push(parseLiteralValue(24));
			}
			values.push(parseLiteralValue(25));
		}
		return {
			type: "Template",
			values,
			range: endRange(start)
		};
	}
	function maybeFunction() {
		if (match(48)) {
			if (!nextMatch(5) && !nextMatch(33) && !nextMatch(49)) return null;
		} else if (!nextMatch(33)) return null;
		return maybe(parseFunction);
	}
	function parseFunction() {
		const start = startRange();
		const args = [];
		if (advanceIfMatch(48)) {
			if (!match(49)) do {
				throwIfNoMatch(5);
				args.push(parseIdentifier());
			} while (advanceIfMatch(54));
			advance(49);
		} else if (match(5)) args.push(parseIdentifier());
		advance(33);
		return {
			type: "Function",
			arguments: args,
			body: parseExpression() || throwError("Expected non-empty expression"),
			range: endRange(start)
		};
	}
	function parseCompareFunction(expr) {
		const start = legacyPrefixStartRange(expr);
		const compares = [parseCompare(expr)];
		while (advanceIfMatch(54)) compares.push(parseCompare(parseExpression(PRECEDENCE[19] + 1)));
		return {
			type: "CompareFunction",
			compares,
			range: endRange(start)
		};
	}
	function parseCompare(query) {
		const start = legacyPrefixStartRange(query);
		return {
			type: "Compare",
			query,
			order: getValueAndAdvance(19),
			range: endRange(start)
		};
	}
	function parseParentheses() {
		const start = startRange();
		advance(48);
		const definitions = parseDefinitions();
		const expression = parseExpression() || createPlaceholder();
		advance(49);
		return {
			type: "Parentheses",
			body: maybeBlock(definitions, expression, start),
			range: endRange(start)
		};
	}
	function parseArray() {
		const start = startRange();
		const elements = [];
		advance(50);
		if (!advanceIfMatch(51)) {
			do {
				if (match(54)) throwError("Expected expression before comma");
				elements.push(match(28) ? parseSpread(SPREAD_ARRAY) : parseExpression());
			} while (advanceIfMatch(54) && !match(51));
			advance(51);
		}
		return {
			type: "Array",
			elements,
			range: endRange(start)
		};
	}
	function parseSliceNotation(expr = null) {
		const start = legacyPrefixStartRange(expr);
		const args = [consumeSurrounded(50, parseExpression, 55), parseExpression()];
		if (advanceIfMatch(55)) args.push(parseExpression());
		advance(51);
		return {
			type: "SliceNotation",
			value: expr,
			arguments: args,
			range: endRange(start)
		};
	}
	function parseObject() {
		const start = startRange();
		advance(52);
		const definitions = parseDefinitions();
		const properties = [];
		if (!match(53)) do
			properties.push(match(28) ? parseSpread(SPREAD_OBJECT) : parseObjectEntry());
		while (advanceIfMatch(54) && !match(53));
		advance(53);
		return maybeBlock(definitions, {
			type: "Object",
			properties,
			range: endRange(start)
		}, start);
	}
	function parseObjectEntry() {
		const start = startRange();
		let key;
		let value = null;
		let pipeline = null;
		switch (current.type) {
			case 20:
			case 21: {
				const method = parseMethod();
				key = {
					type: "Literal",
					value: method.reference.type === "Identifier" ? method.reference.name : method.reference.name.name,
					range: void 0
				};
				value = method;
				if (!match(54) && !match(53) && !match(58)) {
					pipeline = value;
					value = parsePipeline(pipeline, true);
				}
				break;
			}
			case 4:
				if (nextMatch(55) || nextMatch(54) || nextMatch(53) || nextMatch(58)) key = parseIdentifier();
				else {
					pipeline = parseGetProperty(null, false);
					key = pipeline.property;
					value = parsePipeline(pipeline, true);
				}
				break;
			case 5:
				if (nextMatch(55)) key = parseIdentifier(true);
				else {
					key = parseReference();
					if (!match(54) && !match(53) && !match(58)) {
						pipeline = key;
						key = key.name;
						value = parsePipeline(pipeline, true);
					}
				}
				break;
			case 3:
			case 1:
			case 0:
				key = parseLiteralValue();
				break;
			case 8:
				key = parseSpecialReference();
				break;
			case 50:
				key = consumeSurrounded(50, parseExpression, 51);
				advance(55);
				value = parseExpression();
				break;
			default: throwExpected(4, 3, 1, 0, 8, 5, 50, 20, 21);
		}
		if (!pipeline && !value) value = advanceIfMatch(55) ? parseExpression() : null;
		return {
			type: "ObjectEntry",
			key,
			value,
			range: endRange(start)
		};
	}
	function parseSpread(array) {
		const start = startRange();
		return {
			type: "Spread",
			query: advance(28) && parseExpression(),
			array,
			range: endRange(start)
		};
	}
	function parseBracketAccess(value) {
		const start = legacyPrefixStartRange(value);
		return {
			type: "Pick",
			value,
			getter: consumeSurrounded(50, parseExpression, 51),
			range: endRange(start)
		};
	}
	function parseMap(value) {
		const start = legacyPrefixStartRange(value);
		return {
			type: "Map",
			value,
			query: consumeSurrounded(29, parseBlock, 49),
			range: endRange(start)
		};
	}
	function parseMapRecursive(value) {
		const start = legacyPrefixStartRange(value);
		return {
			type: "MapRecursive",
			value,
			query: advanceIfMatch(27) ? match(4) ? parseGetProperty() : parseMethodCall() : consumeSurrounded(31, parseBlock, 49),
			range: endRange(start)
		};
	}
	function parseFilter(value) {
		const start = legacyPrefixStartRange(value);
		return {
			type: "Filter",
			value,
			query: consumeSurrounded(30, parseBlock, 51),
			range: endRange(start)
		};
	}
	function parsePipeline(left, implicit = false) {
		const start = legacyPrefixStartRange(left);
		if (implicit) advanceIfMatch(32);
		else advance(32);
		return {
			type: "Pipeline",
			left,
			right: maybeBlock(!implicit ? parseDefinitions() : null, !implicit ? parseExpression(PRECEDENCE[32] + 1) || createPlaceholder() : parseExpression(), start),
			range: endRange(start)
		};
	}
	function parseTernaryConditional(test, prec) {
		const start = legacyPrefixStartRange(test);
		advance(47);
		return {
			type: "Conditional",
			test,
			consequent: parseExpression(prec) || createPlaceholder(),
			alternate: advanceIfMatch(55) ? parseExpression(prec) || createPlaceholder() : null,
			range: endRange(start)
		};
	}
	function parseBinaryOperator(left, prec) {
		const start = legacyPrefixStartRange(left);
		return {
			type: "Binary",
			operator: getValueAndAdvance(),
			left,
			right: parseExpression(prec) || throwError("Expected expression"),
			range: endRange(start)
		};
	}
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/parser/index.js
var parser_default = {
	parse(input, tolerant = false) {
		const commentRanges = [];
		return {
			ast: parse(input, {
				tolerant,
				onComment: commentRanges
			}),
			commentRanges
		};
	},
	createTokenizer,
	tokenize
};
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Arg1.js
var Arg1_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$35,
	stringify: () => stringify$35,
	walk: () => walk$35
});
function compile$35(node, ctx) {
	const arg1 = ctx.scope.arg1;
	ctx.put(typeof arg1 === "string" ? arg1 : "undefined");
}
function walk$35() {}
function stringify$35(node, ctx) {
	ctx.put("$$");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Array.js
var Array_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$34,
	stringify: () => stringify$34,
	suggest: () => suggest$16,
	walk: () => walk$34
});
function suggest$16(node, ctx) {
	if (node.elements.length === 0) ctx.queryRoot(node.range[0] + 1, node.range[1] - 1);
}
function compile$34(node, ctx) {
	ctx.put("[");
	ctx.list(node.elements, ",");
	ctx.put("]");
}
function walk$34(node, ctx) {
	ctx.list(node.elements);
}
function stringify$34(node, ctx) {
	ctx.put("[");
	ctx.list(node.elements, ",");
	ctx.put("]");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Assertion.js
var Assertion_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$33,
	stringify: () => stringify$33,
	suggest: () => suggest$15,
	walk: () => walk$33
});
function suggest$15(node, ctx) {
	if (!Array.isArray(node.assertion)) {
		ctx.range(node.assertion.range, "var");
		ctx.range(node.assertion.range, "assertion");
	}
}
function compile$33(node, ctx) {
	if (Array.isArray(node.assertion)) {
		ctx.put(node.negation ? "!(" : "(");
		for (let i = 0; i < node.assertion.length; i++) if (i % 2 === 0) ctx.node(node.assertion[i]);
		else switch (node.assertion[i]) {
			case "and":
				ctx.put("&&");
				break;
			case "or":
				ctx.put("||");
				break;
			default: ctx.error("Bad conjuction operator in assertion", node);
		}
		ctx.put(")");
	} else {
		if (node.negation) ctx.put("!");
		switch (node.assertion.type) {
			case "Identifier":
				if (node.assertion.name) {
					ctx.put("!!");
					if (ctx.tolerant) ctx.put("(typeof ");
					ctx.put("a[");
					ctx.node(node.assertion);
					ctx.put("]");
					if (ctx.tolerant) {
						ctx.put("===\"function\"?a[");
						ctx.node(node.assertion);
						ctx.put("]");
					}
					ctx.put(".call(mctx,");
					ctx.nodeOrCurrent();
					ctx.put(")");
					if (ctx.tolerant) ctx.put(":false)");
					if (ctx.usedAssertions.has(node.assertion.name)) ctx.usedAssertions.get(node.assertion.name).push(node.assertion.range);
					else ctx.usedAssertions.set(node.assertion.name, node.assertion.range);
				} else ctx.put("false");
				break;
			case "Method":
				ctx.put("!!");
				ctx.node(node.assertion);
				break;
			default: ctx.error("Unknown assertion node type \"" + node.assertion.type + "\"", node.assertion);
		}
	}
}
function walk$33(node, ctx) {
	if (Array.isArray(node.assertion)) {
		for (let i = 0; i < node.assertion.length; i++) if (i % 2 === 0) ctx.node(node.assertion[i]);
	} else ctx.node(node.assertion);
}
function stringify$33(node, ctx) {
	if (node.negation) ctx.put("not ");
	if (Array.isArray(node.assertion)) {
		ctx.put("(");
		for (let i = 0; i < node.assertion.length; i++) if (i % 2 === 0) ctx.node(node.assertion[i]);
		else {
			ctx.put(" ");
			ctx.put(node.assertion[i]);
			ctx.put(" ");
		}
		ctx.put(")");
	} else ctx.node(node.assertion);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Binary.js
var Binary_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$32,
	stringify: () => stringify$32,
	suggest: () => suggest$14,
	walk: () => walk$32
});
const binary = {
	"in": "in",
	"not in": "notIn",
	"has": "has",
	"has no": "hasNo",
	"and": "and",
	"or": "or",
	"??": "nullish",
	"+": "add",
	"-": "sub",
	"*": "mul",
	"/": "div",
	"%": "mod",
	"=": "eq",
	"!=": "ne",
	"<": "lt",
	"<=": "lte",
	">": "gt",
	">=": "gte",
	"~=": "match"
};
function valueSubset(ctx, values, dict) {
	if (dict.type === "Array") {
		if (dict.elements.length === 0) {
			ctx.range([dict.range[0] + 1, dict.range[1] - 1], "value-subset", values, false);
			return;
		}
		const excludeValues = [];
		for (const { type, range, value } of dict.elements) if (range && (type === "Literal" || type === "Identifier")) excludeValues.push(value);
		const related = excludeValues.length ? ctx.literalList(excludeValues) : false;
		for (const { type, range, value } of dict.elements) if (range) {
			if (type === "Literal" || type === "Identifier" || type === "GetProperty" && value === null) ctx.range(range, "value-subset", values, related);
		}
	}
}
function suggest$14(node, ctx) {
	switch (node.operator) {
		case "in":
			ctx.range(node.left.range, "in-value", node.right, null);
			valueSubset(ctx, node.left, node.right);
			break;
		case "not in":
			valueSubset(ctx, node.left, node.right);
			break;
		case "has":
			ctx.range(node.right.range, "in-value", node.left, null);
			valueSubset(ctx, node.right, node.left);
			break;
		case "has no":
			valueSubset(ctx, node.right, node.left);
			break;
		case "=":
		case "!=": ctx.range(node.right.range, "value", node.left, null);
	}
}
function compile$32(node, ctx) {
	if (node.operator in binary === false) {
		ctx.error("Unknown operator \"" + node.operator + "\"", node);
		return;
	}
	switch (node.operator) {
		case "and": ctx.put("!");
		case "or": {
			const tmpVar = ctx.allocateVar();
			ctx.put(`${ctx.buildinFn("bool")}(${tmpVar}=`);
			ctx.node(node.left);
			ctx.put(`)?${tmpVar}:`);
			ctx.createScope(() => ctx.node(node.right), (sp) => {
				ctx.put(")");
				return "(" + sp + ",";
			}, ctx.scope.$ref);
			break;
		}
		case "??": {
			const tmpVar = ctx.allocateVar();
			ctx.put(`(${tmpVar}=`);
			ctx.node(node.left);
			ctx.put(`,${tmpVar}!==null&&${tmpVar}!==undefined)?${tmpVar}:`);
			ctx.createScope(() => ctx.node(node.right), (sp) => {
				ctx.put(")");
				return "(" + sp + ",";
			}, ctx.scope.$ref);
			break;
		}
		case "has no": ctx.put("!");
		case "has":
			ctx.put(ctx.buildinFn("in"));
			ctx.put("(");
			ctx.node(node.right);
			ctx.put(",");
			ctx.node(node.left);
			ctx.put(")");
			break;
		default:
			ctx.put(ctx.buildinFn(binary[node.operator]));
			ctx.put("(");
			ctx.node(node.left);
			ctx.put(",");
			ctx.node(node.right);
			ctx.put(")");
	}
}
function walk$32(node, ctx) {
	ctx.node(node.left);
	ctx.node(node.right);
}
function stringify$32(node, ctx) {
	ctx.node(node.left);
	if (/^[a-z]/i.test(node.operator)) {
		ctx.put(" ");
		ctx.put(node.operator);
		ctx.put(" ");
	} else ctx.put(node.operator);
	ctx.node(node.right);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Block.js
var Block_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$31,
	stringify: () => stringify$31,
	suggest: () => suggest$13,
	walk: () => walk$31
});
function suggest$13(node, ctx) {
	if (node.body === null) ctx.queryRoot(node.range[1]);
}
function compile$31(node, ctx) {
	if (node.definitions.length) {
		ctx.put("(()=>{");
		ctx.createScope(() => {
			for (const { declarator } of node.definitions) if (declarator.name) ctx.scope.awaitInit.add(ctx.unescapeName("$" + declarator.name, declarator));
			ctx.list(node.definitions);
			ctx.put("return ");
			ctx.nodeOrCurrent(node.body);
		}, (sp) => sp + ";", ctx.scope.$ref);
		ctx.put("})()");
	} else if (node.body && node.body.type === "Object") {
		ctx.put("(");
		ctx.nodeOrCurrent(node.body);
		ctx.put(")");
	} else ctx.nodeOrCurrent(node.body);
}
function walk$31(node, ctx) {
	ctx.list(node.definitions);
	ctx.nodeOrNothing(node.body);
}
function stringify$31(node, ctx) {
	ctx.list(node.definitions);
	ctx.nodeOrNothing(node.body);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Compare.js
var Compare_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$30,
	stringify: () => stringify$30,
	walk: () => walk$30
});
const comparator = {
	"": "cmp",
	"N": "cmpNatural",
	"A": "cmpAnalytical",
	"NA": "cmpNaturalAnalytical",
	"AN": "cmpNaturalAnalytical"
};
function compile$30(node, ctx) {
	const isDesc = node.order.startsWith("desc");
	const cmpFn = comparator[node.order.slice(isDesc ? 4 : 3)] || comparator[""];
	if (isDesc) ctx.put("-");
	ctx.put(ctx.buildinFn(cmpFn));
	ctx.put("((_q=$=>(");
	ctx.createScope(() => ctx.node(node.query), (sp) => sp + ",");
	ctx.put("))(_a),_q(_b))");
}
function walk$30(node, ctx) {
	ctx.node(node.query);
}
function stringify$30(node, ctx) {
	ctx.node(node.query);
	ctx.put(" ");
	ctx.put(node.order);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/CompareFunction.js
var CompareFunction_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$29,
	stringify: () => stringify$29,
	walk: () => walk$29
});
function compile$29(node, ctx) {
	ctx.put("(_a, _b)=>{let _q;return ");
	ctx.list(node.compares, "||");
	ctx.put("||0}");
}
function walk$29(node, ctx) {
	ctx.list(node.compares);
}
function stringify$29(node, ctx) {
	ctx.list(node.compares, ",");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Conditional.js
var Conditional_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$28,
	stringify: () => stringify$28,
	walk: () => walk$28
});
function compile$28(node, ctx) {
	ctx.put(ctx.buildinFn("bool"));
	ctx.put("(");
	ctx.nodeOrCurrent(node.test);
	ctx.put(")?");
	ctx.createScope(() => ctx.nodeOrCurrent(node.consequent), (sp) => {
		ctx.put(")");
		return "(" + sp + ",";
	}, ctx.scope.$ref);
	ctx.put(":");
	ctx.createScope(() => {
		if (node.alternate) {
			if (node.alternate.type === "Placeholder") {
				ctx.put("(");
				ctx.node(node.alternate);
				ctx.put(",undefined)");
			} else ctx.node(node.alternate);
		} else ctx.put("undefined");
	}, (sp) => {
		ctx.put(")");
		return "(" + sp + ",";
	}, ctx.scope.$ref);
}
function walk$28(node, ctx) {
	ctx.nodeOrNothing(node.test);
	ctx.nodeOrNothing(node.consequent);
	ctx.nodeOrNothing(node.alternate);
}
function stringify$28(node, ctx) {
	ctx.nodeOrNothing(node.test);
	ctx.put("?");
	ctx.nodeOrNothing(node.consequent);
	if (node.alternate) {
		ctx.put(":");
		ctx.node(node.alternate);
	}
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Context.js
var Context_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$27,
	stringify: () => stringify$27,
	walk: () => walk$27
});
function compile$27(node, ctx) {
	ctx.put("context");
}
function walk$27() {}
function stringify$27(node, ctx) {
	ctx.put("#");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Current.js
var Current_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$26,
	stringify: () => stringify$26,
	suggest: () => suggest$12,
	walk: () => walk$26
});
function suggest$12(node, ctx) {
	if (node.range) ctx.range(node.range, "var");
}
function compile$26(node, ctx) {
	ctx.put(ctx.scope.$ref);
}
function walk$26() {}
function stringify$26(node, ctx) {
	ctx.put("$");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Data.js
var Data_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$25,
	stringify: () => stringify$25,
	walk: () => walk$25
});
function compile$25(node, ctx) {
	ctx.put("data");
}
function walk$25() {}
function stringify$25(node, ctx) {
	ctx.put("@");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Declarator.js
var Declarator_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$24,
	stringify: () => stringify$24,
	walk: () => walk$24
});
function compile$24(node, ctx) {
	if (node.name) ctx.putIdent("$" + ctx.unescapeName(node.name, node), node);
}
function walk$24() {}
function stringify$24(node, ctx) {
	ctx.put(node.name ? "$" + node.name : "$");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Definition.js
var Definition_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$23,
	stringify: () => stringify$23,
	suggest: () => suggest$11,
	walk: () => walk$23
});
const reservedVars = [
	"$data",
	"$context",
	"$ctx",
	"$array",
	"$idx",
	"$index"
];
const GetProperty$1 = function(value, property, range) {
	return {
		type: "GetProperty",
		value,
		property,
		range
	};
};
const Identifier$1 = function(name, range) {
	return {
		type: "Identifier",
		name,
		range
	};
};
function suggest$11(node, ctx) {
	if (node.value === null) ctx.range(node.declarator.range, "path");
}
function compile$23(node, ctx) {
	const name = node.declarator.name;
	const unescapedName = name && ctx.unescapeName("$" + name, node);
	if (unescapedName === null) {
		ctx.node(node.declarator);
		ctx.nodeOrCurrent(node.value);
		ctx.put(";");
		return;
	}
	if (ctx.scope.own.has(unescapedName)) {
		ctx.error(`Identifier "${unescapedName}" has already been declared`, node.declarator);
		return;
	}
	if (reservedVars.includes(unescapedName)) {
		ctx.error(`Identifier "${unescapedName}" is reserved for future use`, node.declarator);
		return;
	}
	ctx.put("const ");
	ctx.node(node.declarator);
	ctx.put("=");
	ctx.node(node.value || GetProperty$1(null, Identifier$1(name)));
	ctx.put(";");
	ctx.scope.add(unescapedName);
	ctx.scope.own.add(unescapedName);
	ctx.scope.awaitInit.delete(unescapedName);
}
function walk$23(node, ctx) {
	ctx.node(node.declarator);
	ctx.nodeOrNothing(node.value);
}
function stringify$23(node, ctx) {
	ctx.node(node.declarator);
	if (node.value !== null) {
		ctx.put(":");
		ctx.node(node.value);
	}
	ctx.put(";");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Filter.js
var Filter_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$22,
	stringify: () => stringify$22,
	walk: () => walk$22
});
function compile$22(node, ctx) {
	ctx.put(ctx.buildinFn("filter"));
	ctx.put("(");
	ctx.nodeOrCurrent(node.value);
	ctx.put(",$=>");
	ctx.createScope(() => ctx.node(node.query), (sp) => {
		ctx.put(")");
		return "(" + sp + ",";
	});
	ctx.put(")");
}
function walk$22(node, ctx) {
	ctx.nodeOrNothing(node.value);
	ctx.node(node.query);
}
function stringify$22(node, ctx) {
	ctx.nodeOrNothing(node.value);
	ctx.put(".[");
	ctx.node(node.query);
	ctx.put("]");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Function.js
var Function_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$21,
	stringify: () => stringify$21,
	walk: () => walk$21
});
function compile$21(node, ctx) {
	const args = node.arguments.map((arg) => ctx.unescapeName("$" + arg.name, arg));
	const duplicateNameNode = node.arguments.find((node, index) => index > 0 && args.lastIndexOf("$" + node.name, index - 1) !== -1);
	if (duplicateNameNode) ctx.error("Duplicate parameter name \"$" + duplicateNameNode.name + "\" is not allowed", duplicateNameNode);
	ctx.put("function(");
	ctx.put(String(args) || "$");
	ctx.put("){return ");
	ctx.createScope(() => {
		ctx.scope.arg1 = args[1] || "arguments[1]";
		ctx.scope.$ref = args[0] || "$";
		for (const name of args) ctx.scope.add(name);
		ctx.node(node.body);
	}, (sp) => sp + ",");
	ctx.put("}");
}
function walk$21(node, ctx) {
	ctx.node(node.body);
}
function stringify$21(node, ctx) {
	ctx.put("=>");
	ctx.node(node.body);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/GetProperty.js
var GetProperty_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$20,
	stringify: () => stringify$20,
	suggest: () => suggest$10,
	walk: () => walk$20
});
function suggest$10(node, ctx) {
	ctx.range(node.property.range, "path", node.value || void 0, !node.value);
	if (node.value === null) ctx.range(node.property.range, "var");
	if (node.range && node.value === null && node.property.range[0] !== node.range[0]) ctx.queryRoot(node.range[0]);
}
function compile$20(node, ctx) {
	ctx.put(ctx.buildinFn("map"));
	ctx.put("(");
	ctx.nodeOrCurrent(node.value);
	ctx.put(",");
	ctx.node(node.property);
	ctx.put(")");
}
function walk$20(node, ctx) {
	ctx.nodeOrNothing(node.value);
	ctx.node(node.property);
}
function stringify$20(node, ctx) {
	ctx.nodeOrNothing(node.value) && ctx.put(".");
	ctx.node(node.property);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Identifier.js
var Identifier_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$19,
	stringify: () => stringify$19,
	walk: () => walk$19
});
function compile$19(node, ctx) {
	ctx.put(JSON.stringify(ctx.unescapeName(node.name, node, false)));
}
function walk$19() {}
function stringify$19(node, ctx) {
	ctx.put(node.name);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Literal.js
var Literal_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$18,
	stringify: () => stringify$18,
	walk: () => walk$18
});
function compile$18(node, ctx) {
	ctx.put(typeof node.value === "string" ? JSON.stringify(node.value) : String(node.value));
}
function walk$18() {}
function stringify$18(node, ctx) {
	ctx.put(typeof node.value === "string" ? JSON.stringify(node.value) : String(node.value));
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Map.js
var Map_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$17,
	stringify: () => stringify$17,
	walk: () => walk$17
});
function compile$17(node, ctx) {
	ctx.put(ctx.buildinFn("map"));
	ctx.put("(");
	ctx.nodeOrCurrent(node.value);
	ctx.put(",$=>");
	ctx.createScope(() => ctx.node(node.query), (sp) => {
		ctx.put(")");
		return "(" + sp + ",";
	});
	ctx.put(")");
}
function walk$17(node, ctx) {
	ctx.nodeOrNothing(node.value);
	ctx.node(node.query);
}
function stringify$17(node, ctx) {
	ctx.nodeOrNothing(node.value);
	ctx.put(".(");
	ctx.node(node.query);
	ctx.put(")");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/MapRecursive.js
var MapRecursive_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$16,
	stringify: () => stringify$16,
	walk: () => walk$16
});
function compile$16(node, ctx) {
	ctx.put(ctx.buildinFn("mapRecursive"));
	ctx.put("(");
	ctx.nodeOrCurrent(node.value);
	ctx.put(",$=>");
	ctx.createScope(() => ctx.node(node.query), (sp) => {
		ctx.put(")");
		return "(" + sp + ",";
	});
	ctx.put(")");
}
function walk$16(node, ctx) {
	ctx.nodeOrNothing(node.value);
	ctx.node(node.query);
}
function stringify$16(node, ctx) {
	ctx.nodeOrNothing(node.value);
	ctx.put("..");
	if (ctx.isSimpleGetPropertyQuery(node.query) || ctx.isSimpleMethodCallQuery(node.query)) ctx.node(node.query);
	else {
		ctx.put("(");
		ctx.node(node.query);
		ctx.put(")");
	}
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Method.js
var Method_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$15,
	stringify: () => stringify$15,
	suggest: () => suggest$9,
	walk: () => walk$15
});
function suggest$9(node, ctx) {
	if (node.arguments.length === 0) ctx.queryRoot(node.range[1] - 1);
}
function compile$15(node, ctx, relatedNode) {
	const dictMethod = node.reference.type === "Identifier";
	if (ctx.tolerant) ctx.put("(typeof ");
	if (dictMethod) {
		if (ctx.usedMethods.has(node.reference.name)) ctx.usedMethods.get(node.reference.name).push(node.reference.range);
		else ctx.usedMethods.set(node.reference.name, [node.reference.range]);
		ctx.put("m[");
		ctx.node(node.reference);
		ctx.put("]");
	} else ctx.node(node.reference);
	if (ctx.tolerant) {
		ctx.put("===\"function\"?");
		if (dictMethod) {
			ctx.put("m[");
			ctx.node(node.reference);
			ctx.put("]");
		} else ctx.node(node.reference);
	}
	ctx.put(dictMethod ? ".call(mctx," : "(");
	ctx.nodeOrCurrent(relatedNode);
	if (node.arguments.length) {
		ctx.put(",");
		ctx.list(node.arguments, ",");
	}
	ctx.put(")");
	if (ctx.tolerant) ctx.put(":undefined)");
}
function walk$15(node, ctx) {
	ctx.node(node.reference);
	ctx.list(node.arguments);
}
function stringify$15(node, ctx) {
	ctx.node(node.reference);
	ctx.put("(");
	ctx.list(node.arguments, ",");
	ctx.put(")");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/MethodCall.js
var MethodCall_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$14,
	stringify: () => stringify$14,
	walk: () => walk$14
});
function compile$14(node, ctx) {
	ctx.node(node.method, node.value);
}
function walk$14(node, ctx) {
	ctx.nodeOrNothing(node.value);
	ctx.node(node.method);
}
function stringify$14(node, ctx) {
	ctx.nodeOrNothing(node.value) && ctx.put(".");
	ctx.node(node.method);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Object.js
var Object_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$13,
	stringify: () => stringify$13,
	suggest: () => suggest$8,
	walk: () => walk$13
});
function suggest$8(node, ctx) {
	if (node.properties.length === 0) ctx.queryRoot(node.range[0] + 1, node.range[1] - 1);
}
function compile$13(node, ctx) {
	ctx.put("{");
	ctx.list(node.properties, ",");
	ctx.put("}");
}
function walk$13(node, ctx) {
	ctx.list(node.properties);
}
function stringify$13(node, ctx) {
	ctx.put("{");
	ctx.list(node.properties, ",");
	ctx.put("}");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/ObjectEntry.js
var ObjectEntry_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$12,
	stringify: () => stringify$12,
	suggest: () => suggest$7,
	walk: () => walk$12
});
const GetProperty = function(value, property, range) {
	return {
		type: "GetProperty",
		value,
		property,
		range
	};
};
const Identifier = function(name, range) {
	return {
		type: "Identifier",
		name,
		range
	};
};
const noBracketKeyType = /* @__PURE__ */ new Set([
	"Literal",
	"Identifier",
	"Reference",
	"Current"
]);
function suggest$7(node, ctx) {
	if (node.value === null) switch (node.key.type) {
		case "Literal":
		case "Identifier":
			ctx.range(node.range, "path");
			ctx.range(node.range, "var");
			break;
		case "Current":
		case "Reference": ctx.range(node.range, "var");
	}
}
function compile$12(node, ctx) {
	let value = node.value;
	switch (node.key.type) {
		case "Current":
			ctx.put("...null");
			return;
		case "Literal":
			ctx.node(node.key);
			value = value || GetProperty(null, Identifier(String(node.key.value)));
			break;
		case "Identifier":
			ctx.node(node.key);
			value = value || GetProperty(null, node.key);
			break;
		case "Reference":
			ctx.node(node.key.name);
			value = value || node.key;
			break;
		default:
			ctx.put("[");
			ctx.node(node.key);
			ctx.put("]");
	}
	ctx.put(":");
	ctx.node(value);
}
function walk$12(node, ctx) {
	ctx.node(node.key);
	ctx.nodeOrNothing(node.value);
}
function stringify$12(node, ctx) {
	if (noBracketKeyType.has(node.key.type)) {
		ctx.node(node.key);
		if (node.value === null) return;
	} else {
		ctx.put("[");
		ctx.node(node.key);
		ctx.put("]");
	}
	ctx.put(":");
	ctx.node(node.value);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Parentheses.js
var Parentheses_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$11,
	stringify: () => stringify$11,
	walk: () => walk$11
});
function compile$11(node, ctx) {
	ctx.put("(");
	ctx.node(node.body);
	ctx.put(")");
}
function walk$11(node, ctx) {
	ctx.node(node.body);
}
function stringify$11(node, ctx) {
	ctx.put("(");
	ctx.node(node.body);
	ctx.put(")");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Pick.js
var Pick_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$10,
	stringify: () => stringify$10,
	suggest: () => suggest$6,
	walk: () => walk$10
});
function suggest$6(node, ctx) {
	if (node.getter === null) {
		const pos = node.range[1] - 1;
		ctx.range([pos, pos], "key", node.value, false);
		ctx.queryRoot(pos);
	} else if (node.getter.type === "GetProperty" || node.getter.type === "Reference" || node.getter.type === "Literal" && typeof node.getter.value === "string") ctx.range(node.getter.range, "key", node.value, false);
}
function compile$10(node, ctx) {
	ctx.put(ctx.buildinFn("pick"));
	ctx.put("(");
	ctx.node(node.value);
	if (node.getter) {
		ctx.put(",");
		ctx.node(node.getter);
	}
	ctx.put(")");
}
function walk$10(node, ctx) {
	ctx.node(node.value);
	if (node.getter !== null) ctx.node(node.getter);
}
function stringify$10(node, ctx) {
	ctx.node(node.value);
	ctx.put("[");
	if (node.getter !== null) ctx.node(node.getter);
	ctx.put("]");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Pipeline.js
var Pipeline_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$9,
	stringify: () => stringify$9,
	walk: () => walk$9
});
function compile$9(node, ctx) {
	ctx.put("($=>(");
	ctx.createScope(() => ctx.node(node.right), (sp) => sp + ",");
	ctx.put("))");
	ctx.put("(");
	ctx.nodeOrCurrent(node.left);
	ctx.put(")");
}
function walk$9(node, ctx) {
	ctx.node(node.left);
	ctx.node(node.right);
}
function stringify$9(node, ctx) {
	ctx.node(node.left);
	ctx.put("|");
	ctx.node(node.right);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Placeholder.js
var Placeholder_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$8,
	stringify: () => stringify$8,
	suggest: () => suggest$5,
	walk: () => walk$8
});
function suggest$5(node, ctx) {
	ctx.queryRoot(node.range[1]);
}
function compile$8(node, ctx) {
	ctx.put(ctx.scope.$ref);
}
function walk$8() {}
function stringify$8() {}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Postfix.js
var Postfix_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$7,
	stringify: () => stringify$7,
	walk: () => walk$7
});
function compile$7(node, ctx) {
	if (node.operator && node.operator.type) {
		ctx.put("($=>(");
		ctx.createScope(() => ctx.node(node.operator), (sp) => sp + ",");
		ctx.put("))");
		ctx.put("(");
		ctx.node(node.argument);
		ctx.put(")");
		return;
	}
	ctx.error("Unknown operator \"" + node.operator + "\"", node);
}
function walk$7(node, ctx) {
	ctx.node(node.argument);
	if (node.operator && node.operator.type) ctx.node(node.operator);
}
function stringify$7(node, ctx) {
	ctx.node(node.argument);
	ctx.put(" ");
	ctx.node(node.operator);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Prefix.js
var Prefix_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$6,
	stringify: () => stringify$6,
	walk: () => walk$6
});
function compile$6(node, ctx) {
	switch (node.operator) {
		case "no":
		case "not":
			ctx.put("!");
			ctx.put(ctx.buildinFn("bool"));
			ctx.put("(");
			ctx.node(node.argument);
			ctx.put(")");
			break;
		case "+":
		case "-":
			ctx.put(node.operator);
			ctx.node(node.argument);
			break;
		case "is":
			ctx.put("!!(");
			ctx.node(node.argument);
			ctx.put(")");
			break;
		default: ctx.error("Unknown operator \"" + node.operator + "\"", node);
	}
}
function walk$6(node, ctx) {
	ctx.node(node.argument);
	if (node.operator && node.operator.type) ctx.node(node.operator);
}
function stringify$6(node, ctx) {
	ctx.put(node.operator);
	if (node.operator !== "-" && node.operator !== "+") ctx.put(" ");
	ctx.node(node.argument);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Reference.js
var Reference_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$5,
	stringify: () => stringify$5,
	suggest: () => suggest$4,
	walk: () => walk$5
});
function suggest$4(node, ctx) {
	if (node.range) ctx.range(node.range, "var");
}
function compile$5(node, ctx) {
	const nameNode = node.name;
	const unescapedName = ctx.unescapeName("$" + nameNode.name, nameNode);
	const inScope = ctx.scope.has(unescapedName);
	const awaitInit = ctx.scope.awaitInit.has(unescapedName);
	if (!inScope || awaitInit) {
		if (awaitInit) {
			ctx.put("f.awaitInitRef(()=>");
			ctx.putIdent(unescapedName, nameNode);
			if (!ctx.tolerant) {
				ctx.put(",");
				ctx.put(JSON.stringify(unescapedName));
				ctx.put(",");
				ctx.put(JSON.stringify(nameNode.range));
			}
			ctx.put(")");
		} else if (ctx.tolerant) ctx.put("undefined");
		else {
			ctx.putIdent(unescapedName, node);
			ctx.error(`${unescapedName} is not defined`, node);
		}
		return;
	}
	ctx.putIdent(unescapedName, nameNode);
}
function walk$5(node, ctx) {
	ctx.node(node.name);
}
function stringify$5(node, ctx) {
	ctx.put("$");
	ctx.node(node.name);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/SliceNotation.js
var SliceNotation_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$4,
	stringify: () => stringify$4,
	walk: () => walk$4
});
function compile$4(node, ctx) {
	ctx.put(ctx.buildinFn("slice"));
	ctx.put("(");
	ctx.nodeOrCurrent(node.value);
	node.arguments.slice(0, 3).forEach((item) => {
		ctx.put(",");
		item ? ctx.node(item) : ctx.put("undefined");
	});
	ctx.put(")");
}
function walk$4(node, ctx) {
	ctx.nodeOrNothing(node.value);
	for (const arg of node.arguments.slice(0, 3)) if (arg) ctx.node(arg);
}
function stringify$4(node, ctx) {
	const [a, b, c] = node.arguments;
	ctx.nodeOrNothing(node.value);
	ctx.put("[");
	if (a) ctx.node(a);
	ctx.put(":");
	if (b) ctx.node(b);
	if (c) {
		ctx.put(":");
		ctx.node(c);
	}
	ctx.put("]");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Spread.js
var Spread_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$3,
	stringify: () => stringify$3,
	suggest: () => suggest$3,
	walk: () => walk$3
});
function suggest$3(node, ctx) {
	if (node.query === null) ctx.queryRoot(node.range[1]);
}
function compile$3(node, ctx) {
	if (node.array) {
		ctx.put("...");
		ctx.put(ctx.buildinFn("ensureArray"));
		ctx.put("(");
		ctx.nodeOrCurrent(node.query);
		ctx.put(")");
		return;
	}
	ctx.put("...");
	ctx.nodeOrCurrent(node.query);
}
function walk$3(node, ctx) {
	ctx.nodeOrNothing(node.query);
}
function stringify$3(node, ctx) {
	ctx.put("...");
	ctx.nodeOrNothing(node.query);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/Template.js
var Template_exports = /* @__PURE__ */ __exportAll({
	compile: () => compile$2,
	stringify: () => stringify$2,
	suggest: () => suggest$2,
	walk: () => walk$2
});
function suggest$2(node, ctx) {
	for (const [idx, v] of Object.entries(node.values)) if (v === null) ctx.queryRoot(node.values[Number(idx) - 1].range[1]);
}
function compile$2(node, ctx) {
	for (const [k, v] of Object.entries(node.values)) {
		const idx = Number(k);
		if (v === null) continue;
		if (idx !== 0) ctx.put("+");
		if (idx % 2 === 0) ctx.put("\"" + encodeString(v.value, compileEscape) + "\"");
		else {
			ctx.put("(");
			ctx.node(v);
			ctx.put(")");
		}
	}
}
function walk$2(node, ctx) {
	for (const v of node.values) if (v !== null) ctx.node(v);
}
function stringify$2(node, ctx) {
	const lastIdx = node.values.length - 1;
	for (const [k, v] of Object.entries(node.values)) {
		const idx = Number(k);
		if (idx % 2 === 0) {
			ctx.put(idx === 0 ? "`" : "}");
			ctx.put(encodeString(v.value, stringifyEscape));
			ctx.put(idx === lastIdx ? "`" : "${");
		} else if (v !== null) ctx.node(v);
	}
}
const compileEscape = /* @__PURE__ */ new Map([
	["\b", "\\b"],
	["\n", "\\n"],
	["\r", "\\r"],
	["\f", "\\f"],
	["	", "\\t"],
	["\v", "\\v"],
	["\u2028", "\\u2028"],
	["\u2029", "\\u2029"],
	["\\", "\\\\"],
	["\"", "\\\""]
]);
const stringifyEscape = /* @__PURE__ */ new Map([
	["\b", "\\b"],
	["\f", "\\f"],
	["	", "\\t"],
	["\v", "\\v"],
	["\\", "\\\\"],
	["$", "\\$"]
]);
function encodeString(s, map) {
	let result = "";
	for (let i = 0; i < s.length; i++) result += map.get(s[i]) || s[i];
	return result;
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/nodes/index.js
const nodes = {
	Arg1: Arg1_exports,
	Array: Array_exports,
	Assertion: Assertion_exports,
	Binary: Binary_exports,
	Block: Block_exports,
	Compare: Compare_exports,
	Conditional: Conditional_exports,
	Context: Context_exports,
	Current: Current_exports,
	Data: Data_exports,
	Declarator: Declarator_exports,
	Definition: Definition_exports,
	Filter: Filter_exports,
	Function: Function_exports,
	GetProperty: GetProperty_exports,
	Identifier: Identifier_exports,
	Literal: Literal_exports,
	Map: Map_exports,
	MapRecursive: MapRecursive_exports,
	Method: Method_exports,
	MethodCall: MethodCall_exports,
	Object: Object_exports,
	ObjectEntry: ObjectEntry_exports,
	Parentheses: Parentheses_exports,
	Pick: Pick_exports,
	Pipeline: Pipeline_exports,
	Placeholder: Placeholder_exports,
	Postfix: Postfix_exports,
	Prefix: Prefix_exports,
	Reference: Reference_exports,
	SliceNotation: SliceNotation_exports,
	CompareFunction: CompareFunction_exports,
	Spread: Spread_exports,
	Template: Template_exports
};
const extract = (type) => new Map(Object.entries(nodes).map(([key, value]) => [key, value[type]]).filter(([, value]) => typeof value === "function"));
const build = {};
extract("build").forEach((value, key) => build[key] = value);
const compile$1 = extract("compile");
const walk$1 = extract("walk");
const stringify$1 = extract("stringify");
const suggest$1 = extract("suggest");
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/walk.js
function walk(ast, options) {
	function walk(node) {
		if (walk$1.has(node.type)) {
			enter(node);
			walk$1.get(node.type)(node, ctx);
			leave(node);
		} else throw new Error("Unknown node type \"" + node.type + "\"");
	}
	const ctx = {
		node: walk,
		nodeOrNothing(node) {
			if (node !== null) walk(node);
		},
		list(list) {
			list.forEach(walk);
		}
	};
	let enter = () => {};
	let leave = () => {};
	if (typeof options === "function") options = { enter: options };
	if (options) {
		if (typeof options.enter === "function") enter = options.enter;
		if (typeof options.leave === "function") leave = options.leave;
	}
	walk(ast);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/suggest.js
function isSuggestProhibitedChar(str, offset) {
	return offset >= 0 && offset < str.length && /[a-zA-Z_$0-9]/.test(str[offset]);
}
function isWhiteSpace(str, offset) {
	const code = str.charCodeAt(offset);
	return code === 9 || code === 10 || code === 13 || code === 32;
}
function onlyWsInRange(str, start, end) {
	for (; start < end; start++) if (!isWhiteSpace(str, start)) return false;
	return true;
}
function getSuggestRanges(from, to, input, commentRanges, noSuggestOnEofPos) {
	const ranges = [];
	for (let i = 0; i < commentRanges.length; i++) {
		const [commentFrom, commentTo] = commentRanges[i];
		if (commentFrom >= to) break;
		if (commentFrom < from) continue;
		ranges.push(from, commentFrom);
		from = commentTo;
	}
	if (from !== input.length || !noSuggestOnEofPos) ranges.push(from, to);
	return ranges;
}
function processSuggestRanges(suggestRanges, source, commentRanges, noSuggestOnEofPos) {
	const result = [];
	for (let i = 0; i < suggestRanges.length; i++) {
		let [start, end, type, related] = suggestRanges[i];
		if (onlyWsInRange(source, start, end)) {
			while (start >= 0 && isWhiteSpace(source, start - 1)) start--;
			while (end < source.length && isWhiteSpace(source, end)) end++;
			if (isSuggestProhibitedChar(source, start - 1)) {
				if (start === end) continue;
				start++;
			}
			if (isSuggestProhibitedChar(source, end)) {
				if (start === end) continue;
				end--;
			}
		}
		const ranges = getSuggestRanges(start, end, source, commentRanges, noSuggestOnEofPos);
		for (let j = 0; j < ranges.length; j += 2) result.push([
			ranges[j],
			ranges[j + 1],
			type,
			related
		]);
	}
	return result;
}
function collectNodeSuggestions(ast) {
	let currentNode = null;
	const literalList = [];
	const suggestions = Object.assign(/* @__PURE__ */ new Map(), { literalList });
	const add = (node, range) => {
		if (!suggestions.has(node)) suggestions.set(node, [range]);
		else suggestions.get(node).push(range);
	};
	const ctx = {
		range(range, type, node = currentNode, related = true) {
			add(node, [
				...range,
				type,
				related
			]);
		},
		literalList(values) {
			const name = "l" + literalList.length;
			literalList.push([name, values]);
			return name;
		},
		queryRoot(start, end = start) {
			add(currentNode, [
				start,
				end,
				"var",
				true
			]);
			add(currentNode, [
				start,
				end,
				"path",
				true
			]);
		}
	};
	walk(ast, (node) => {
		if (suggest$1.has(node.type)) {
			const prevNode = currentNode;
			currentNode = node;
			suggest$1.get(node.type)(node, ctx);
			currentNode = prevNode;
		}
	});
	return suggestions;
}
function suggest(source, { ast, commentRanges }) {
	const suggestions = collectNodeSuggestions(ast);
	const noSuggestOnEofPos = commentRanges.length && commentRanges[commentRanges.length - 1][1] === source.length && !/[\r\n]$/.test(source);
	for (const [node, rawRanges] of suggestions) suggestions.set(node, processSuggestRanges(rawRanges, source, commentRanges, noSuggestOnEofPos));
	return suggestions;
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/stringify.js
function isSimpleGetPropertyQuery(node) {
	if (node.type !== "GetProperty") return false;
	return true;
}
function isSimpleMethodCallQuery(node) {
	if (node.type !== "MethodCall") return false;
	return true;
}
function stringify(ast) {
	function walk(node) {
		if (stringify$1.has(node.type)) stringify$1.get(node.type)(node, ctx);
		else throw new Error("Unknown node type \"" + node.type + "\"");
	}
	const buffer = [];
	const ctx = {
		isSimpleGetPropertyQuery,
		isSimpleMethodCallQuery,
		put(chunk) {
			buffer.push(chunk);
		},
		node: walk,
		nodeOrNothing(node) {
			if (node !== null) {
				walk(node);
				return true;
			}
		},
		list(list, sep) {
			if (!sep) {
				list.forEach(walk);
				return;
			}
			list.forEach((element, idx) => {
				if (idx > 0) ctx.put(sep);
				walk(element);
			});
		}
	};
	walk(ast);
	return buffer.join("");
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/error.js
function createCustomError(name, message, extensions) {
	const error = Object.assign(Object.create(SyntaxError.prototype), extensions);
	const errorStack = /* @__PURE__ */ new Error();
	error.name = name;
	error.message = message;
	Object.defineProperty(error, "stack", { get: function() {
		return (errorStack.stack || "").replace(/^(.+\n){1,3}/, name + ": " + message + "\n");
	} });
	return error;
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/compile.js
const IdStartRx = /^[\p{ID_Start}$_]$/u;
const IdContinueRx = /^[\p{ID_Continue}$]$/u;
function compile(ast, tolerant, suggestions) {
	return compileInternal(ast, "query", tolerant, suggestions);
}
function compileMethod(ast, tolerant, suggestions) {
	return compileInternal(ast, "method", tolerant, suggestions);
}
var Scope = class Scope extends Set {
	constructor(items, awaitInit = [], arg1 = false, $ref = "$") {
		super(items);
		this.own = /* @__PURE__ */ new Set();
		this.awaitInit = new Set(awaitInit);
		this.firstCurrent = null;
		this.captureCurrent = [];
		this.arg1 = arg1;
		this.$ref = $ref;
	}
	spawn(arg1, $ref) {
		return new Scope(this, this.awaitInit, arg1, $ref);
	}
};
function compileInternal(ast, kind, tolerant = false, suggestions = null) {
	function newStatPoint(values) {
		const spName = "s" + spNames.length;
		spNames.push(values ? [spName, values] : [spName]);
		return spName;
	}
	function getNodeSpName(node) {
		let spName = nodeSpName.get(node);
		if (!spName) nodeSpName.set(node, spName = newStatPoint());
		return spName;
	}
	function addSuggestPoint(start, end, type, spName, related) {
		let range = [
			start,
			end,
			JSON.stringify(type)
		];
		if (type === "var") {
			if (!ctx.scope.size) return;
			range.push(JSON.stringify([...ctx.scope]));
		} else {
			if (!spName) spName = newStatPoint();
			range.push(spName);
			if (related) range.push(typeof related === "string" ? related : getNodeSpName(related));
		}
		normalizedSuggestRanges.push(range);
		return spName;
	}
	function createScope(fn, defCurrent, $ref = "$") {
		const prevScope = ctx.scope;
		const scopeStart = buffer.length - 1;
		const arg1 = prevScope.arg1 || kind === "method" && "arguments[1]";
		ctx.scope = prevScope.spawn(arg1, $ref);
		fn();
		if (ctx.scope.captureCurrent.length) {
			const stat = "stat(" + ctx.scope.captureCurrent.reduce((spName, range) => addSuggestPoint(...range, spName), void 0) + "," + ctx.scope.$ref + ")";
			if (ctx.scope.firstCurrent) buffer[ctx.scope.firstCurrent] = stat;
			else buffer[scopeStart] += defCurrent(stat);
		}
		ctx.scope = prevScope;
	}
	function walk(node, relatedNode) {
		let spName = false;
		if (suggestions !== null) {
			if (suggestions.has(node)) for (const [start, end, type, related] of suggestions.get(node)) if (type === "var") addSuggestPoint(start, end, type);
			else if (related === true) ctx.scope.captureCurrent.push([
				start,
				end,
				type
			]);
			else {
				if (!spName) {
					spName = getNodeSpName(node);
					buffer.push("stat(" + spName + ",");
				}
				if (type) addSuggestPoint(start, end, type, spName, related);
			}
			if (node.type === "Current" && ctx.scope.firstCurrent === null) ctx.scope.firstCurrent = buffer.length;
		}
		if (compile$1.has(node.type)) compile$1.get(node.type)(node, ctx, relatedNode);
		else throw new Error("Unknown node type \"" + node.type + "\"");
		if (spName) buffer.push(")");
	}
	const spNames = [];
	const nodeSpName = /* @__PURE__ */ new WeakMap();
	const allocatedVars = [];
	const normalizedSuggestRanges = [];
	const buffer = [
		{ toString() {
			if (kind === "query") return ["((data,context)=>{", ...ctx.usedMethods.size || ctx.usedAssertions.size ? [
				"const method = (name,...args)=>m[name].apply(mctx,args);",
				"const assertion = (name,...args)=>a[name].apply(mctx,args);",
				"const mctx = Object.freeze({context,method,assertion});"
			] : []].join("\n") + "\n";
			if (kind === "method") return [
				"(function(data){",
				"const mctx = this;",
				"const context = mctx.context;"
			].join("\n") + "\n";
			throw new Error("Unknown kind: " + kind);
		} },
		{ toString() {
			return allocatedVars.length > 0 ? "let " + allocatedVars + ";\n" : "";
		} },
		{ toString() {
			return spNames.length > 0 ? ["const stat=(s,v)=>(s.add(v),v);\n", "const " + spNames.map(([name, values]) => name + "=new Set(" + (values ? JSON.stringify(values) : "") + ")") + ";\n"].join("") : "";
		} },
		{ toString() {
			const lists = suggestions && Array.isArray(suggestions.literalList) && suggestions.literalList.map(([name, values]) => name + "=" + JSON.stringify(values));
			return lists && lists.length ? "const " + lists + ";\n" : "";
		} },
		suggestions === null ? "return " : "return{\nvalue: "
	];
	const initCtx = {};
	const usedBuildinMethods = /* @__PURE__ */ new Set();
	const ctx = {
		tolerant,
		usedAssertions: /* @__PURE__ */ new Map(),
		usedMethods: /* @__PURE__ */ new Map(),
		suggestions: suggestions !== null,
		buildinFn(name) {
			usedBuildinMethods.add(name);
			return "f." + name;
		},
		scope: new Scope(),
		createScope,
		error(message, node) {
			const error = new SyntaxError(message);
			if (node && node.range) error.details = { loc: { range: node.range } };
			if (!tolerant) throw error;
			return error;
		},
		allocateVar() {
			const name = "tmp" + allocatedVars.length;
			allocatedVars.push(name);
			return name;
		},
		unescapeName(name, node, ident = true) {
			return name.replace(/\\u[0-9a-fA-F]{4}/g, (m, offset) => {
				const unescapedChar = String.fromCharCode(parseInt(m.slice(2), 16));
				const rx = offset === 0 ? IdStartRx : IdContinueRx;
				if (ident && !rx.test(unescapedChar)) this.error(`Bad escape sequence in indentifier name (expected ${rx})`, node && { range: [node.range[0] + offset, node.range[0] + offset + 6] });
				return unescapedChar;
			});
		},
		put: (chunk) => buffer.push(chunk),
		putIdent(name, node) {
			if (!/^[\p{ID_Start}$_][\p{ID_Continue}$]*$/u.test(name)) throw this.error("Bad identifier name " + JSON.stringify(name), node);
			this.put(name);
		},
		node: walk,
		nodeOrCurrent(node, relatedNode) {
			walk(node || { type: "Current" }, relatedNode);
		},
		list(list, sep, relatedNode) {
			list.forEach((node, idx) => {
				if (idx > 0) buffer.push(sep);
				walk(node, relatedNode);
			});
		}
	};
	createScope(() => walk(ast), (sp) => {
		buffer.push(")");
		return "(" + sp + ",";
	}, "data");
	if (!tolerant) {
		const { usedMethods, usedAssertions } = ctx;
		if (usedAssertions.size) {
			buffer.unshift(" this.assertAssertions(a)||");
			initCtx.assertAssertions = function(providedAssertions) {
				for (const [assertion, range] of usedAssertions.entries()) if (!hasOwn(providedAssertions, assertion)) return () => {
					throw Object.assign(/* @__PURE__ */ new Error(`Assertion "${assertion}" is not defined`), { details: { loc: { range } } });
				};
			};
		}
		if (usedMethods.size) {
			buffer.unshift(" this.assertMethods(m)||");
			initCtx.assertMethods = function(providedMethods) {
				for (const [method, ranges] of usedMethods.entries()) if (!hasOwn(providedMethods, method)) return () => {
					throw Object.assign(/* @__PURE__ */ new Error(`Method "${method}" is not defined. If that's a custom method make sure you added it with "methods" section in options`), { details: { loc: { range: ranges[0] } } });
				};
			};
		}
	}
	if (suggestions !== null) buffer.push(",\nstats: [" + normalizedSuggestRanges.map((s) => "[" + s + "]") + "],\nmethods: m,\nassertions: a\n}");
	return compileFunction$1(initCtx, "return" + buffer.join("") + "})");
}
function compileFunction$1(initCtx, functionBody) {
	try {
		const fn = new Function("f,m,a", functionBody);
		return Object.assign(fn.bind(initCtx), { toString() {
			return fn.toString().replace(/^(\S+\s+)anonymous([^)\s]+)\s*\)/, "$1query$2)");
		} });
	} catch (e) {
		throw createCustomError("SyntaxError", "Jora query compilation error", {
			compiledSource: `function(f,m,a){\n${functionBody}\n}`,
			details: e
		});
	}
}
//#endregion
//#region ../../node_modules/.pnpm/@discoveryjs+natural-compare@1.1.0/node_modules/@discoveryjs/natural-compare/src/index.js
const TYPE_EOF = 0;
const TYPE_WS = 1;
const TYPE_DELIM = 2;
const TYPE_NUM = 4;
const TYPE_WORD = 8;
const TYPE_WS_OR_DELIM = 3;
const TYPE_NUM_OR_WORD = 12;
const isSortableValue = (value) => typeof value === "number" || typeof value === "string";
const safeCharCodeAt = (source, offset) => offset < source.length ? source.charCodeAt(offset) : 0;
const isSign = (code) => code === 43 || code === 45;
const isDigit = (code) => code >= 48 && code <= 57;
const isWS = (code) => code === 9 || code === 10 || code === 12 || code === 13 || code === 32;
const isDelim = (code) => code > 32 && code < 256 && (code < 65 || code > 90) && (code < 97 || code > 122) && !isDigit(code) && !isSign(code) || code === 8470;
const isWord = (code) => code && !isWS(code) && !isDelim(code) && !isDigit(code);
const isNumberStart = (first, second, third) => {
	if (isSign(first)) {
		if (isDigit(second)) return 2;
		return second === 46 && isDigit(third) ? 3 : 0;
	}
	if (isDigit(first)) return 1;
	return 0;
};
const findEndOfType = (source, offset, isType) => {
	while (isType(safeCharCodeAt(source, ++offset)));
	return offset;
};
const consumeNumber = (source, offset, preventFloat) => {
	let code = safeCharCodeAt(source, offset);
	if (isSign(code)) code = safeCharCodeAt(source, offset += 1);
	if (isDigit(code)) {
		offset = findEndOfType(source, offset, isDigit);
		code = safeCharCodeAt(source, offset);
	}
	if (code === 46 && isDigit(safeCharCodeAt(source, offset + 1))) {
		if (preventFloat) return offset;
		const expectedEnd = findEndOfType(source, offset + 1, isDigit);
		code = safeCharCodeAt(source, expectedEnd);
		if (code === 46) return offset;
		offset = expectedEnd;
	}
	if (code === 69 || code === 101) {
		let sign = 1;
		code = safeCharCodeAt(source, offset + 1);
		if (isSign(code)) {
			sign = 2;
			code = safeCharCodeAt(source, offset + 2);
		}
		if (isDigit(code)) offset = findEndOfType(source, offset + sign, isDigit);
	}
	return offset;
};
const getToken = (source, offset, preventFloat, preventSign) => {
	if (offset >= source.length) return TYPE_EOF;
	const code = safeCharCodeAt(source, offset);
	if (isWS(code)) return TYPE_WS | findEndOfType(source, offset, isWS) - offset << 4;
	if (isDelim(code) || preventSign && isSign(code)) return TYPE_DELIM | findEndOfType(source, offset, isDelim) - offset << 4;
	if (isNumberStart(code, safeCharCodeAt(source, offset + 1), safeCharCodeAt(source, offset + 2))) return TYPE_NUM | consumeNumber(source, offset, preventFloat) - offset << 4;
	return TYPE_WORD | findEndOfType(source, offset, isWord) - offset << 4;
};
const compare = (a, b, analytical) => {
	let offsetA = 0;
	let offsetB = 0;
	let preventFloat = false;
	let preventSign = false;
	let postCmpResult = 0;
	let postCmpResultType = 0;
	let firstPart = true;
	while (true) {
		const partA = getToken(a, offsetA, preventFloat, preventSign);
		const partB = getToken(b, offsetB, preventFloat, preventSign);
		const typeA = partA & 15;
		const lenA = partA >> 4;
		const typeB = partB & 15;
		const lenB = partB >> 4;
		if (typeA !== typeB) {
			if (firstPart && typeA & TYPE_WS_OR_DELIM && typeB & TYPE_NUM_OR_WORD) {
				postCmpResult = 1;
				postCmpResultType = typeA;
				offsetA += lenA;
				continue;
			}
			if (firstPart && typeB & TYPE_WS_OR_DELIM && typeA & TYPE_NUM_OR_WORD) {
				postCmpResult = -1;
				postCmpResultType = typeB;
				offsetB += lenB;
				continue;
			}
			return typeA - typeB;
		}
		if (typeA === TYPE_EOF) return postCmpResult;
		firstPart = false;
		preventFloat = false;
		preventSign = false;
		const minLength = lenA < lenB ? lenA : lenB;
		let substrDiff = lenA - lenB;
		let cA = "";
		let cB = "";
		for (let i = 0; i < minLength; i++) {
			cA = a[offsetA + i];
			cB = b[offsetB + i];
			if (cA !== cB) {
				substrDiff = cA < cB ? -1 : 1;
				break;
			}
		}
		if (typeA & TYPE_WS_OR_DELIM) {
			preventFloat = a[offsetA + lenA - 1] === ".";
			if (substrDiff !== 0) {
				if (typeA > postCmpResultType) {
					postCmpResultType = typeA;
					postCmpResult = substrDiff;
				}
			}
		} else if (typeA & TYPE_NUM) {
			preventSign = true;
			if (substrDiff !== 0) {
				const numDiff = a.substr(offsetA, lenA) - b.substr(offsetB, lenB);
				if (numDiff !== 0) return analytical ? -numDiff : numDiff;
				if (typeA > postCmpResultType) {
					const afc = safeCharCodeAt(a, offsetA);
					const bfc = safeCharCodeAt(b, offsetB);
					const order = afc === 45 ? -1 : 1;
					postCmpResultType = typeA;
					postCmpResult = afc !== bfc && (afc === 45 || bfc === 43) ? -1 : afc !== bfc && (afc === 43 || bfc === 45) ? 1 : (lenA - lenB || substrDiff) < 0 ? -order : order;
					if (analytical) postCmpResult = -postCmpResult;
				}
			}
		} else if (substrDiff !== 0) {
			if (cA !== cB) {
				const sA = a.substr(offsetA, lenA);
				const sB = b.substr(offsetB, lenB);
				const siA = sA.toLowerCase();
				const siB = sB.toLowerCase();
				if (siA !== siB) return siA < siB ? -1 : 1;
				return sA < sB ? -1 : 1;
			}
			return substrDiff;
		}
		offsetA += lenA;
		offsetB += lenB;
	}
};
const createCompareFn = (analytical) => (a, b) => {
	if (isSortableValue(a) && isSortableValue(b)) return Math.sign(compare(String(a), String(b), analytical));
	return 0;
};
const naturalCompare = createCompareFn(false);
const naturalAnalyticalCompare = createCompareFn(true);
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/utils/compare.js
const TYPE_BOOLEAN = 1;
const TYPE_NAN = 2;
const TYPE_NUMBER = 3;
const TYPE_STRING = 4;
const TYPE_NULL = 5;
const TYPE_OBJECT = 6;
const TYPE_OTHER = 7;
const TYPE_UNDEFINED = 8;
function cmpType(value) {
	switch (typeof value) {
		case "boolean": return TYPE_BOOLEAN;
		case "number": return value !== value ? TYPE_NAN : TYPE_NUMBER;
		case "string": return TYPE_STRING;
		case "object": return value === null ? TYPE_NULL : TYPE_OBJECT;
		case "undefined": return TYPE_UNDEFINED;
		default: return TYPE_OTHER;
	}
}
function cmp(a, b) {
	const typeA = cmpType(a);
	const typeB = cmpType(b);
	return typeA !== typeB ? typeA < typeB ? -1 : 1 : a < b ? -1 : a > b ? 1 : 0;
}
function cmpAnalytical(a, b) {
	const typeA = cmpType(a);
	const typeB = cmpType(b);
	if (typeA !== typeB) return typeA < typeB ? -1 : 1;
	if (typeA === TYPE_NUMBER) return b - a;
	return a < b ? -1 : a > b ? 1 : 0;
}
function cmpNatural(a, b) {
	const typeA = cmpType(a);
	const typeB = cmpType(b);
	if ((typeA === TYPE_NUMBER || typeA === TYPE_STRING) && (typeB === TYPE_NUMBER || typeB === TYPE_STRING)) return naturalCompare(a, b);
	return typeA !== typeB ? typeA < typeB ? -1 : 1 : a < b ? -1 : a > b ? 1 : 0;
}
function cmpNaturalAnalytical(a, b) {
	const typeA = cmpType(a);
	const typeB = cmpType(b);
	if ((typeA === TYPE_NUMBER || typeA === TYPE_STRING) && (typeB === TYPE_NUMBER || typeB === TYPE_STRING)) return naturalAnalyticalCompare(a, b, true);
	return typeA !== typeB ? typeA < typeB ? -1 : 1 : a < b ? -1 : a > b ? 1 : 0;
}
function getterToCmp(getter, cmp) {
	return getter.length === 1 ? (a, b) => cmp(getter(a), getter(b)) : getter;
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/lang/compile-buildin.js
var compile_buildin_default = Object.freeze({
	awaitInitRef,
	ensureArray,
	bool: isTruthy,
	and: (a, b) => isTruthy(a) ? b : a,
	or: (a, b) => isTruthy(a) ? a : b,
	add,
	sub,
	mul,
	div,
	mod,
	eq,
	ne,
	lt,
	lte,
	gt,
	gte,
	in: in_,
	notIn: (a, b) => !in_(a, b),
	has: (a, b) => in_(b, a),
	hasNo: (a, b) => !in_(b, a),
	cmp,
	cmpAnalytical,
	cmpNatural,
	cmpNaturalAnalytical,
	match,
	pick,
	indexOf,
	lastIndexOf,
	map,
	mapRecursive,
	some,
	filter,
	slice
});
function awaitInitRef(fn, name, range) {
	try {
		return fn();
	} catch (e) {
		if (name) throw Object.assign(/* @__PURE__ */ new ReferenceError(`Cannot access ${name} before initialization`), { details: {
			name,
			loc: { range }
		} });
	}
}
function ensureArray(value) {
	return isArray(value) ? value : [value];
}
function add(a, b) {
	if (isArray(a) || isArray(b)) return [.../* @__PURE__ */ new Set([...isArray(a) ? a : [a], ...isArray(b) ? b : [b]])];
	return a + b;
}
function sub(a, b) {
	if (isArray(a)) {
		const result = new Set(a);
		if (isArray(b)) b.forEach((item) => result.delete(item));
		else result.delete(b);
		return [...result];
	}
	return a - b;
}
function mul(a, b) {
	return a * b;
}
function div(a, b) {
	return a / b;
}
function mod(a, b) {
	return a % b;
}
function eq(a, b) {
	return Object.is(a, b);
}
function ne(a, b) {
	return !Object.is(a, b);
}
function lt(a, b) {
	return a < b;
}
function lte(a, b) {
	return a <= b;
}
function gt(a, b) {
	return a > b;
}
function gte(a, b) {
	return a >= b;
}
function in_(a, b) {
	if (isPlainObject(b)) return hasOwn(b, a);
	return b ? internalIndexOf(b, a) !== -1 : false;
}
function match(value, pattern) {
	if (typeof pattern === "function") return some(value, pattern);
	if (isRegExp(pattern)) return some(value, pattern.test.bind(pattern));
	if (pattern === null || pattern === void 0) return true;
	return false;
}
function pick(current, ref = () => true) {
	if (!current) return;
	if (typeof ref === "function") {
		if (isArray(current) || typeof current === "string") {
			for (let i = 0; i < current.length; i++) if (isTruthy(ref(current[i], i))) return current[i];
		}
		for (const key in current) if (hasOwn(current, key)) {
			if (isTruthy(ref(current[key], key))) return current[key];
		}
		return;
	}
	if (isArray(current) || typeof current === "string") return isFinite(ref) ? current[ref < 0 ? current.length + Number(ref) : Number(ref) || 0] : void 0;
	return hasOwn(current, ref) ? current[ref] : void 0;
}
function indexOf(dict, searchElement, fromIndex) {
	return dict ? internalIndexOf(dict, searchElement, fromIndex) : -1;
}
function internalIndexOf(dict, searchElement, fromIndex = 0) {
	if (Number.isNaN(searchElement)) {
		if (isArrayLike(dict)) {
			for (let i = parseIntDefault(fromIndex, 0); i < dict.length; i++) if (Number.isNaN(dict[i])) return i;
		}
	}
	if (typeof dict.indexOf === "function") return dict.indexOf(searchElement, fromIndex);
	return -1;
}
function lastIndexOf(dict, searchElement, fromIndex) {
	return dict ? internalLastIndexOf(dict, searchElement, fromIndex) : -1;
}
function internalLastIndexOf(dict, searchElement, fromIndex) {
	if (Number.isNaN(searchElement)) {
		if (isArrayLike(dict)) {
			for (let i = parseIntDefault(fromIndex, dict.length - 1); i >= 0; i--) if (Number.isNaN(dict[i])) return i;
		}
	}
	if (typeof dict.lastIndexOf === "function") return dict.lastIndexOf(searchElement, parseIntDefault(fromIndex, dict.length - 1));
	return -1;
}
function map(value, getter) {
	const fn = typeof getter === "function" ? getter : (current) => getPropertyValue(current, getter);
	if (isArray(value)) return [...value.reduce((set, item) => addToSet(set, fn(item)), /* @__PURE__ */ new Set())];
	return value !== void 0 ? fn(value) : value;
}
function mapRecursive(value, getter) {
	const result = /* @__PURE__ */ new Set();
	addToSet(result, map(value, getter));
	result.forEach((current) => addToSet(result, map(current, getter)));
	return [...result];
}
function some(value, fn) {
	return isArray(value) ? value.some((current) => isTruthy(fn(current))) : isTruthy(fn(value));
}
function filter(value, fn) {
	if (isArray(value)) return value.filter((current) => isTruthy(fn(current)));
	return isTruthy(fn(value)) ? value : void 0;
}
function slice(value, from, to, step = 1) {
	if (!isArrayLike(value)) return [];
	from = parseIntDefault(from, 0);
	to = parseIntDefault(to, value.length);
	step = parseIntDefault(step, 1) || 1;
	from = from < 0 ? Math.max(0, value.length + from) : Math.min(value.length, from);
	to = to < 0 ? Math.max(0, value.length + to) : Math.min(value.length, to);
	if (from >= 0 && to >= 0 && from <= to && step === 1) return typeof value === "string" ? value.slice(from, to) : Array.prototype.slice.call(value, from, to);
	if (from > to) {
		[from, to] = [to, from];
		step = -step;
	}
	const result = [];
	if (step > 0) for (let i = from; i < to; i += step) result.push(value[i]);
	else for (let i = to - 1; i >= from; i += step) result.push(value[i]);
	return typeof value === "string" ? result.join("") : result;
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/utils/stable-sort.js
const stableSortSize = isSortStable(20) ? Infinity : isSortStable(10) ? 10 : 0;
function isSortStable(n) {
	return Array.from({ length: n }, (_, idx) => ({ idx })).sort((a, b) => a.idx % 2 - b.idx % 2).every((a, idx) => idx < n / 2 ? a.idx >> 1 === idx : Math.ceil(n / 2) + (a.idx >> 1) === idx);
}
function stableSort(array, cmp) {
	if (array.length <= stableSortSize) return array.slice().sort(cmp);
	return array.map((value, idx) => ({
		value,
		idx
	})).sort((a, b) => (a.value === void 0 ? b.value !== void 0 : b.value === void 0 ? -1 : cmp(a.value, b.value)) || a.idx - b.idx).map((item) => item.value);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/utils/heap.js
const defaultMinCompare = (a, b) => a - b;
const defaultMaxCompare = (a, b) => b - a;
var Heap = class {
	constructor(maxSize, compare, accept, disallowDuplicates = false) {
		this.maxSize = maxSize || Infinity;
		this.compare = compare || defaultMaxCompare;
		this.accept = accept || null;
		this.values = [];
		this.valuesSet = disallowDuplicates ? /* @__PURE__ */ new Set() : null;
	}
	add(value) {
		if (this.accept !== null && !this.accept(value)) return;
		if (this.valuesSet !== null && this.valuesSet.has(value)) return;
		if (this.values.length < this.maxSize) {
			if (this.valuesSet !== null) this.valuesSet.add(value);
			this.values.push(value);
			this.heapifyUp(this.values.length - 1);
		} else if (this.compare(this.values[0], value) > 0) {
			if (this.valuesSet !== null) {
				this.valuesSet.delete(this.values[0]);
				this.valuesSet.add(value);
			}
			this.values[0] = value;
			this.heapifyDown();
		}
	}
	addArray(array) {
		for (let i = 0; i < array.length; i++) this.add(array[i]);
	}
	extract() {
		const topValue = this.values[0];
		const lastValue = this.values.pop();
		if (this.values.length > 0) {
			this.values[0] = lastValue;
			this.heapifyDown();
		}
		if (this.valuesSet !== null) this.valuesSet.delete(topValue);
		return topValue;
	}
	heapifyUp(idx) {
		const values = this.values;
		let idxValue = values[idx];
		while (idx > 0) {
			const parentIdx = idx - 1 >> 1;
			const parentValue = values[parentIdx];
			if (this.compare(parentValue, idxValue) > 0) break;
			values[parentIdx] = idxValue;
			values[idx] = parentValue;
			idx = parentIdx;
		}
	}
	heapifyDown() {
		const values = this.values;
		const size = values.length;
		const halfSize = size >> 1;
		let idx = 0;
		let idxValue = values[idx];
		let largestIdx = idx;
		let largestValue = idxValue;
		while (idx < halfSize) {
			const left = 2 * idx + 1;
			const leftValue = values[left];
			const right = left + 1;
			if (this.compare(leftValue, idxValue) > 0) {
				largestIdx = left;
				largestValue = leftValue;
			}
			if (right < size) {
				const rightValue = values[right];
				if (this.compare(rightValue, largestValue) > 0) {
					largestIdx = right;
					largestValue = rightValue;
				}
			}
			if (largestIdx === idx) break;
			values[idx] = largestValue;
			values[largestIdx] = idxValue;
			idx = largestIdx;
			largestValue = idxValue;
		}
	}
	[Symbol.iterator]() {
		return this.values.slice().sort(this.compare)[Symbol.iterator]();
	}
};
var MaxHeap = class extends Heap {};
var MinHeap = class extends Heap {
	constructor(maxSize, compare, accept, allowDuplicates) {
		super(maxSize, compare ? (a, b) => -compare(a, b) : defaultMinCompare, accept, allowDuplicates);
	}
};
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/utils/process-numeric-array.js
const self$2 = (value) => value;
function toNumber(value) {
	return value !== null && typeof value === "object" ? NaN : Number(value);
}
function processNumericArray(current, getter, apply) {
	if (isArrayLike(current)) {
		if (typeof getter !== "function") getter = self$2;
		for (const value of current) {
			const mappedValue = getter(value);
			if (mappedValue !== void 0) apply(toNumber(mappedValue));
		}
	}
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/utils/percentile.js
function percentile$1(array, k, getter, compare) {
	if (array.length === 0 || !isFinite(k) || k < 0 || k > 100) return;
	let arrayLength = 0;
	let rank = k * (array.length - 1) / 100;
	let lowerRank = Math.floor(rank);
	let upperRank = Math.ceil(rank);
	let hasNaNs = false;
	const heap = k < 50 ? new MinHeap(upperRank + 1, compare) : new MaxHeap(array.length - lowerRank, compare);
	processNumericArray(array, getter, (value) => {
		if (Number.isNaN(value)) hasNaNs = true;
		heap.add(value);
		arrayLength++;
	});
	if (hasNaNs) return NaN;
	if (array.length !== arrayLength) {
		if (arrayLength === 0) return;
		rank = k * (arrayLength - 1) / 100;
		lowerRank = Math.floor(rank);
		upperRank = Math.ceil(rank);
		const maxSize = k < 50 ? upperRank + 1 : arrayLength - lowerRank;
		for (let i = heap.values.length; i > maxSize; i--) heap.extract();
	}
	if (lowerRank !== upperRank) {
		const a = heap.extract();
		const b = heap.values[0];
		return a <= b ? a + (b - a) * (rank - lowerRank) : b + (a - b) * (rank - lowerRank);
	}
	return heap.values[0];
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/utils/statistics.js
const self$1 = (value) => value;
function sumAndCount(current, getter) {
	let sum = void 0;
	let correction = 0;
	let count = 0;
	processNumericArray(current, getter, (num) => {
		count++;
		if (sum === void 0) sum = num;
		else {
			const transition = sum;
			const absTransition = Math.abs(transition);
			const absNum = Math.abs(num);
			sum += num;
			if (absTransition !== Infinity && absNum !== Infinity) {
				if (absTransition >= absNum) correction += transition - sum + num;
				else correction += num - sum + transition;
			}
		}
	});
	if (sum !== void 0) sum += correction;
	return {
		sum,
		count
	};
}
function numbers(current, getter) {
	const result = [];
	processNumericArray(current, getter, result.push.bind(result));
	return result;
}
function count(current, getter) {
	let count = 0;
	if (isArrayLike(current)) {
		if (typeof getter !== "function") getter = self$1;
		for (const value of current) if (getter(value) !== void 0) count++;
	}
	return count;
}
function sum(current, getter) {
	return sumAndCount(current, getter).sum;
}
function mean(current, getter) {
	const { sum, count } = sumAndCount(current, getter);
	if (count > 0) return sum / count;
}
function variance(current, getter) {
	let count = 0;
	let mean = 0;
	let M2 = 0;
	processNumericArray(current, getter, (num) => {
		count += 1;
		let delta = num - mean;
		mean += delta / count;
		M2 += delta * (num - mean);
	});
	if (count > 0) return M2 / count;
}
function stdev(current, getter) {
	const v = variance(current, getter);
	if (v !== void 0) return Math.sqrt(v);
}
function min(current, cmp = cmpNatural) {
	let min;
	if (current && isFinite(current.length) && typeof cmp === "function") {
		cmp = getterToCmp(cmp, cmpNatural);
		for (let i = 0; i < current.length; i++) {
			const value = current[i];
			if ((min === void 0 || cmp(value, min) < 0) && cmp(value, void 0) !== 0) min = value;
		}
	}
	return min;
}
function max(current, cmp = cmpNatural) {
	let max;
	if (current && isFinite(current.length) && typeof cmp === "function") {
		cmp = getterToCmp(cmp, cmpNatural);
		for (let i = 0; i < current.length; i++) {
			const value = current[i];
			if ((max === void 0 || cmp(value, max) >= 0) && cmp(value, void 0) !== 0) max = value;
		}
	}
	return max;
}
function percentile(current, p, getter) {
	if (isArrayLike(current)) return percentile$1(current, p, getter);
}
function median(current, getter) {
	return percentile(current, 50, getter);
}
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/methods.js
function noop() {}
function self(value) {
	return value;
}
function matchEntry(match) {
	if (match === null) return null;
	return {
		matched: match.slice(),
		start: match.index,
		end: match.index + match[0].length,
		input: match.input,
		groups: match.groups || null
	};
}
function replaceMatchEntry(args) {
	const last = args.pop();
	const groups = typeof last === "string" ? null : last;
	const input = groups === null ? last : args.pop();
	const start = args.pop();
	return {
		matched: args,
		start,
		end: start + args[0].length,
		input,
		groups
	};
}
const replaceAll = String.prototype.replaceAll || function(pattern, replacement) {
	return isRegExp(pattern) ? this.replace(pattern, replacement) : this.split(pattern).join(String(replacement));
};
var methods_default = Object.freeze({
	bool: compile_buildin_default.bool,
	filter: compile_buildin_default.filter,
	map: compile_buildin_default.map,
	pick: compile_buildin_default.pick,
	indexOf: compile_buildin_default.indexOf,
	lastIndexOf: compile_buildin_default.lastIndexOf,
	keys(current) {
		return Object.keys(current || {});
	},
	values(current) {
		const values = /* @__PURE__ */ new Set();
		for (const key in current) if (hasOwn(current, key)) addToSet(values, current[key]);
		return [...values];
	},
	entries(current) {
		const entries = [];
		for (const key in current) if (hasOwn(current, key)) entries.push({
			key,
			value: current[key]
		});
		return entries;
	},
	fromEntries(current) {
		const result = Object.create(null);
		if (Array.isArray(current)) {
			for (const entry of current) if (entry) result[entry.key] = entry.value;
		}
		return result;
	},
	size(current) {
		if (isPlainObject(current)) return Object.keys(current).length;
		return current && current.length || 0;
	},
	sort(current, comparator = cmp) {
		if (!isArray(current)) return current;
		if (typeof comparator === "function" && comparator.length !== 2) {
			const getter = comparator;
			comparator = (a, b) => {
				a = getter(a);
				b = getter(b);
				if (Array.isArray(a) && Array.isArray(b)) {
					if (a.length !== b.length) return a.length < b.length ? -1 : 1;
					for (let i = 0; i < a.length; i++) {
						const ret = cmp(a[i], b[i]);
						if (ret !== 0) return ret;
					}
					return 0;
				}
				return cmp(a, b);
			};
		}
		return stableSort(current, comparator);
	},
	reverse(current) {
		return isArray(current) ? current.slice().reverse() : current;
	},
	slice(current, from, to) {
		return typeof current === "string" || isArray(current) ? current.slice(from, to) : Array.prototype.slice.call(current, from, to);
	},
	group(current, keyGetter, valueGetter) {
		const map = /* @__PURE__ */ new Map();
		const result = [];
		if (typeof keyGetter !== "function") keyGetter = noop;
		if (typeof valueGetter !== "function") valueGetter = self;
		if (!isArray(current)) current = [current];
		for (const item of current) {
			const keys = keyGetter(item);
			if (isArray(keys)) {
				if (keys.length > 0) {
					const value = valueGetter(item);
					for (const key of keys) addToMapSet(map, key, value);
				} else addToMapSet(map, void 0, valueGetter(item));
			} else addToMapSet(map, keys, valueGetter(item));
		}
		for (const [key, value] of map) result.push({
			key,
			value: [...value]
		});
		return result;
	},
	join(current, separator) {
		return isArray(current) ? current.join(separator) : String(current);
	},
	match(current, pattern, matchAll) {
		const input = String(current);
		const flags = isRegExp(pattern) ? pattern.flags : "";
		if (matchAll || flags.includes("g")) {
			const result = [];
			let cursor = new RegExp(pattern, (flags || "").replace(/g|$/, "g"));
			let match;
			while (match = cursor.exec(input)) result.push(matchEntry(match));
			return result;
		}
		return matchEntry(input.match(pattern));
	},
	reduce(current, fn, initValue = void 0) {
		if (isArray(current)) return initValue !== void 0 ? current.reduce((res, current) => fn(current, res), initValue) : current.reduce((res, current) => fn(current, res));
		return fn(current, initValue);
	},
	split(current, pattern) {
		if (isArray(current)) {
			const patternFn = typeof pattern === "function" ? pattern : Object.is.bind(null, pattern);
			const result = [];
			let start = 0;
			let end = 0;
			for (; end < current.length; end++) if (patternFn(current[end])) {
				result.push(current.slice(start, end));
				start = end + 1;
			}
			result.push(current.slice(start, end));
			return result;
		}
		return String(current).split(pattern);
	},
	replace(current, pattern, replacement) {
		if (isArray(current)) {
			const patternFn = typeof pattern === "function" ? pattern : Object.is.bind(null, pattern);
			const result = [...current];
			for (let i = 0; i < result.length; i++) {
				const value = result[i];
				if (patternFn(value)) result[i] = typeof replacement === "function" ? replacement(value) : replacement;
			}
			return result;
		}
		if (isRegExp(pattern) && !pattern.flags.includes("g")) pattern = new RegExp(pattern, pattern.flags + "g");
		return replaceAll.call(String(current), pattern, typeof replacement === "function" ? (...args) => replacement(replaceMatchEntry(args)) : replacement);
	},
	toLowerCase(current, locales) {
		return String(current).toLocaleLowerCase(locales);
	},
	toUpperCase(current, locales) {
		return String(current).toLocaleUpperCase(locales);
	},
	trim(current) {
		return String(current).trim();
	},
	...[
		"abs",
		"acos",
		"acosh",
		"asin",
		"asinh",
		"atan",
		"atan2",
		"atanh",
		"cbrt",
		"ceil",
		"clz32",
		"cos",
		"cosh",
		"exp",
		"expm1",
		"floor",
		"fround",
		"hypot",
		"imul",
		"log10",
		"log2",
		"pow",
		"round",
		"sign",
		"sin",
		"sinh",
		"sqrt",
		"tan",
		"tanh",
		"trunc"
	].reduce((res, method) => {
		res[method] = Math[method];
		return res;
	}, {}),
	ln: Math.log,
	ln1p: Math.log1p,
	numbers,
	count,
	sum,
	avg: mean,
	variance,
	stdev,
	min,
	max,
	percentile,
	p: percentile,
	median
});
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/assertions.js
var assertions_default = Object.freeze({
	function: (value) => typeof value === "function",
	symbol: (value) => typeof value === "symbol",
	primitive: (value) => value === null || typeof value !== "object" && typeof value !== "function",
	string: (value) => typeof value === "string",
	number: (value) => typeof value === "number",
	int: Number.isInteger,
	finite: (value) => Number.isFinite(value),
	nan: (value) => Number.isNaN(value),
	infinity: (value) => value === Infinity || value === -Infinity,
	boolean: (value) => value === true || value === false,
	null: (value) => value === null,
	undefined: (value) => value === void 0,
	nullish: (value) => value === null || value === void 0,
	object: isPlainObject,
	array: isArray,
	regexp: isRegExp,
	truthy: isTruthy,
	falsy: (value) => !isTruthy(value)
});
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/stat.js
const contextToType = {
	"path": "property",
	"key": "value",
	"value": "value",
	"in-value": "value",
	"value-subset": "value",
	"var": "variable",
	"assertion": "assertion"
};
function addObjectKeysToSet(object, set) {
	if (object !== null && typeof object === "object" && !isArray(object)) Reflect.ownKeys(object).forEach(set.add, set);
}
function valuesToSuggestions(context, values, related, suggestions = /* @__PURE__ */ new Set()) {
	const addValue = (value) => {
		switch (typeof value) {
			case "string":
			case "number": suggestions.add(value);
		}
	};
	switch (context) {
		case "path": {
			const keys = /* @__PURE__ */ new Set();
			for (const value of values) if (Array.isArray(value)) for (const item of value) addObjectKeysToSet(item, keys);
			else addObjectKeysToSet(value, keys);
			keys.forEach(suggestions.add, suggestions);
			break;
		}
		case "key": {
			const keys = /* @__PURE__ */ new Set();
			for (const value of values) addObjectKeysToSet(value, keys);
			keys.forEach(suggestions.add, suggestions);
			break;
		}
		case "value":
			for (const value of values) addValue(value);
			break;
		case "in-value": {
			const keys = /* @__PURE__ */ new Set();
			for (const value of values) if (isArray(value)) value.forEach(addValue);
			else addObjectKeysToSet(value, keys);
			keys.forEach(suggestions.add, suggestions);
			break;
		}
		case "var":
			for (const value of values) suggestions.add(value);
			break;
		case "value-subset": {
			const ignoreValues = new Set(related);
			for (const value of values) if (!ignoreValues.has(value)) addValue(value);
			break;
		}
	}
	return suggestions;
}
function findSourcePosRanges(source, pos, points, includeEmpty = false) {
	const ranges = [];
	for (let [from, to, context, values, related = null] of points) if (pos >= from && pos <= to && (includeEmpty || values.size || values.length)) {
		let text = source.substring(from, to);
		if (!/\S/.test(text)) {
			from = to = pos;
			text = "";
		}
		ranges.push({
			context,
			from,
			to,
			text,
			values,
			related
		});
	}
	return ranges;
}
function normalizeFunctionOption(value, fn) {
	if (typeof value === "function") return value;
	if (value === true) return fn;
	return false;
}
function normalizeFilterPattern(value) {
	if (/^(["']).*\1$/.test(value)) try {
		if (value[0] === "'") value = `"${value.slice(1, -1).replace(/\\.|"/g, (m) => m === "\\'" ? "'" : m === "\"" ? "\\\"" : m)}"`;
		return JSON.parse(value);
	} catch (e) {}
	return value;
}
function defaultFilterFactory(pattern) {
	return (value) => (typeof value === "string" ? value : String(value)).toLowerCase().indexOf(pattern) !== -1;
}
var stat_default = (source, { value, stats, assertions }) => ({
	get value() {
		return value;
	},
	stat(pos, includeEmpty) {
		return findSourcePosRanges(source, pos, stats, includeEmpty);
	},
	suggestion(pos, options) {
		let { limit = Infinity, sort, filter: filterFactory } = options || {};
		sort = normalizeFunctionOption(sort, compile_buildin_default.cmp);
		filterFactory = normalizeFunctionOption(filterFactory, defaultFilterFactory);
		const storageType = sort && isFinite(limit) ? MaxHeap : Set;
		const ranges = findSourcePosRanges(source, pos, stats, true);
		const typeSuggestions = /* @__PURE__ */ new Map();
		const result = [];
		for (const range of ranges) {
			const { context, text, from, to, values, related } = range;
			const type = contextToType[context];
			if (!typeSuggestions.has(type)) {
				let storage;
				switch (storageType) {
					case MaxHeap:
						storage = new MaxHeap(limit, sort, filterFactory && filterFactory(normalizeFilterPattern(text)), true);
						break;
					case Set: storage = /* @__PURE__ */ new Set();
				}
				typeSuggestions.set(type, {
					type,
					from,
					to,
					text,
					suggestions: storage
				});
			}
			const { suggestions } = typeSuggestions.get(type);
			switch (context) {
				case "assertion":
					if (suggestions.size === 0 || suggestions.values && suggestions.values.length === 0) for (const value of Object.keys(assertions)) suggestions.add(value);
					break;
				default: valuesToSuggestions(context, values, related, suggestions);
			}
		}
		if (storageType === Set) for (const entry of typeSuggestions.values()) {
			let { suggestions } = entry;
			if (sort) suggestions = [...suggestions].sort(sort);
			if (filterFactory || isFinite(limit)) {
				const result = [];
				const accept = filterFactory ? filterFactory(normalizeFilterPattern(entry.text)) : () => true;
				for (const value of suggestions) if (accept(value) && result.push(value) >= limit) break;
				suggestions = result;
			}
			entry.suggestions = suggestions;
		}
		for (const entry of typeSuggestions.values()) {
			entry.suggestions = Array.isArray(entry.suggestions) ? entry.suggestions : [...entry.suggestions];
			if (entry.suggestions.length) result.push(entry);
		}
		return result.length ? result : null;
	}
});
//#endregion
//#region ../../node_modules/.pnpm/jora@1.0.0-beta.16/node_modules/jora/src/index.js
const cacheStrict = /* @__PURE__ */ new Map();
const cacheStrictStat = /* @__PURE__ */ new Map();
const cacheTollerant = /* @__PURE__ */ new Map();
const cacheTollerantStat = /* @__PURE__ */ new Map();
function defineDictFunction(dict, name, value, queryMethods, queryAssertions) {
	if (typeof value === "string") Object.defineProperty(dict, name, {
		configurable: true,
		get() {
			const fn = compileFunction(value, "method")(compile_buildin_default, queryMethods, queryAssertions);
			Object.defineProperty(dict, name, {
				enumerable: true,
				value: fn
			});
			return fn;
		}
	});
	else if (typeof value === "function") Object.defineProperty(dict, name, {
		enumerable: true,
		value
	});
}
function buildQueryMethodsAndAssertions(customMethods, customAssertions) {
	if (!customMethods && !customAssertions) return {
		queryMethods: methods_default,
		queryAssertions: assertions_default
	};
	const queryMethods = { ...methods_default };
	const queryAssertions = { ...assertions_default };
	for (const [name, value] of Object.entries(customMethods || {})) {
		if (hasOwn(methods_default, name)) throw new Error(`Builtin method "${name}" can\'t be overridden`);
		defineDictFunction(queryMethods, name, value, queryMethods, queryAssertions);
	}
	for (const [name, value] of Object.entries(customAssertions || {})) {
		if (hasOwn(assertions_default, name)) throw new Error(`Builtin assertion "${name}" can\'t be overridden`);
		defineDictFunction(queryAssertions, name, value, queryMethods, queryAssertions);
	}
	return {
		queryMethods,
		queryAssertions
	};
}
function defaultDebugHandler(sectionName, value) {
	console.log(`[${sectionName}]`);
	if (typeof value === "string") console.log(value);
	else if (value !== void 0) console.dir(value, { depth: null });
	console.log();
}
function compileFunction(source, kind, statMode, tolerantMode, debug) {
	debug = typeof debug === "function" ? debug : Boolean(debug) ? defaultDebugHandler : false;
	if (debug) {
		debug("=========================");
		debug("Compile query from source", source);
	}
	const parseResult = parser_default.parse(source, tolerantMode);
	if (debug) {
		debug("AST", parseResult.ast);
		debug("Restored source", stringify(parseResult.ast));
	}
	const suggestions = statMode ? suggest(source, parseResult) : null;
	if (debug && suggestions) {
		const esc = (s) => JSON.stringify(s).slice(1, -1);
		const ranges = [].concat(...[...suggestions.entries()].map(([node, ranges]) => ranges.map((range) => [node, ...range])));
		let prevRange = [];
		let prevPrefix = null;
		debug("Stat/suggestion ranges", ranges.sort((a, b) => a[1] - b[1]).map(([node, ...range]) => {
			const [start, end, type, related] = range;
			let prelude;
			if (start === prevRange[0] && end === prevRange[1]) prelude = " ".repeat(prevPrefix.length);
			else {
				const pre = esc(source.slice(0, start)).length;
				const long = esc(source.substring(start, end)).length;
				prevRange = range;
				prevPrefix = " ".repeat(pre) + (!long ? "\\" : "~".repeat(long)) + " " + start + ":" + end;
				prelude = esc(source) + "\n" + prevPrefix;
			}
			return prelude + " [" + type + "] on " + node.type + (related === true ? " (current)" : related && related.type ? " & " + related.type : "");
		}).join("\n"));
	}
	const fn = kind === "method" ? compileMethod(parseResult.ast, tolerantMode, suggestions) : compile(parseResult.ast, tolerantMode, suggestions);
	if (debug) debug("Compiled code", fn.toString());
	return fn;
}
function createQuery(source, options) {
	options = options || {};
	const statMode = Boolean(options.stat);
	const tolerantMode = Boolean(options.tolerant);
	const cache = statMode ? tolerantMode ? cacheTollerantStat : cacheStrictStat : tolerantMode ? cacheTollerant : cacheStrict;
	const { methods: customMethods, assertions: customAssertions } = options || {};
	const { queryMethods, queryAssertions } = buildQueryMethodsAndAssertions(customMethods, customAssertions);
	let fn;
	source = String(source);
	if (cache.has(source) && !options.debug) fn = cache.get(source);
	else {
		fn = compileFunction(source, "query", statMode, tolerantMode, options.debug);
		cache.set(source, fn);
	}
	fn = fn(compile_buildin_default, queryMethods, queryAssertions);
	return statMode ? Object.assign((data, context) => stat_default(source, fn(data, context)), { query: fn }) : fn;
}
function setup(options) {
	const cacheStrict = /* @__PURE__ */ new Map();
	const cacheStrictStat = /* @__PURE__ */ new Map();
	const cacheTollerant = /* @__PURE__ */ new Map();
	const cacheTollerantStat = /* @__PURE__ */ new Map();
	const { methods: customMethods, assertions: customAssertions } = options || {};
	const { queryMethods, queryAssertions } = buildQueryMethodsAndAssertions(customMethods, customAssertions);
	return function query(source, queryOptions) {
		queryOptions = queryOptions || {};
		const statMode = Boolean(queryOptions.stat);
		const tolerantMode = Boolean(queryOptions.tolerant);
		const cache = statMode ? tolerantMode ? cacheTollerantStat : cacheStrictStat : tolerantMode ? cacheTollerant : cacheStrict;
		let fn;
		source = String(source);
		if (cache.has(source) && !queryOptions.debug) fn = cache.get(source);
		else {
			const perform = compileFunction(source, "query", statMode, tolerantMode, queryOptions.debug)(compile_buildin_default, queryMethods, queryAssertions);
			fn = statMode ? Object.assign((data, context) => stat_default(source, perform(data, context)), { query: perform }) : perform;
			cache.set(source, fn);
		}
		return fn;
	};
}
var src_default = Object.assign(createQuery, {
	version,
	buildin: compile_buildin_default,
	methods: methods_default,
	assertions: assertions_default,
	setup,
	syntax: {
		tokenize: parser_default.tokenize,
		parse: parser_default.parse,
		suggest,
		walk,
		stringify,
		compile,
		compileMethod
	}
});
//#endregion
export { src_default as default };
