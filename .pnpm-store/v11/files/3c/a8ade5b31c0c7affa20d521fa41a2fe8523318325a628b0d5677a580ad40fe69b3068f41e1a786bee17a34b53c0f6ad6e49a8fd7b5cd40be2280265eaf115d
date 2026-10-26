//#region src/node/engine/normalize.ts
/** True when a property key is excluded by the filter options. */
function isExcludedKey(key, opts) {
	if (opts.excludeUnderscoreProps && key.startsWith("_")) return true;
	if (opts.excludeDollarProps && key.startsWith("$")) return true;
	return false;
}
const OPAQUE_TAGS = [];
for (const name of [
	"WeakMap",
	"WeakSet",
	"WeakRef",
	"ArrayBuffer",
	"SharedArrayBuffer"
]) {
	const ctor = globalThis[name];
	if (typeof ctor === "function") OPAQUE_TAGS.push([ctor, name]);
}
function normalize(value, options = {}) {
	const start = performance.now();
	const walker = {
		seen: /* @__PURE__ */ new Map(),
		stats: {
			nodes: 0,
			refs: 0,
			truncatedDepth: 0,
			truncatedEntries: 0,
			truncatedProps: 0,
			ms: 0
		},
		opts: {
			maxDepth: options.maxDepth ?? 8,
			maxEntries: options.maxEntries ?? 200,
			maxProps: options.maxProps ?? 150,
			maxString: options.maxString ?? 4e3,
			excludeFunctions: options.excludeFunctions ?? false,
			excludeUnderscoreProps: options.excludeUnderscoreProps ?? false,
			excludeDollarProps: options.excludeDollarProps ?? false
		}
	};
	const data = walk$1(value, walker, 0, "#", []);
	walker.stats.ms = Math.round((performance.now() - start) * 100) / 100;
	return {
		data,
		stats: walker.stats
	};
}
/**
* Re-descend a live graph along a `NodePath`, mirroring the walker's own
* traversal (and re-applying `excludeFunctions` to array indices so a lazily
* fetched index lines up with what the client saw). Returns the node the
* path addresses, or `undefined` if any step falls off the graph.
*/
function navigate(value, path, options = {}) {
	let cur = value;
	for (const [kind, at] of path) {
		if (cur === null || typeof cur !== "object") return void 0;
		switch (kind) {
			case "k":
				cur = cur instanceof Map ? cur.get(at) : Object.hasOwn(cur, at) ? cur[at] : void 0;
				break;
			case "i": {
				const arr = cur;
				cur = (options.excludeFunctions ? arr.filter((item) => typeof item !== "function") : arr)[at];
				break;
			}
			case "s":
				cur = [...cur][at];
				break;
			case "mk":
				cur = [...cur.entries()][at]?.[0];
				break;
			case "mv": cur = [...cur.entries()][at]?.[1];
		}
	}
	return cur;
}
function walk$1(value, w, depth, path, segs) {
	w.stats.nodes++;
	const primitive = walkPrimitive(value, w);
	if (primitive.handled) return primitive.value;
	const obj = value;
	const seenPath = w.seen.get(obj);
	if (seenPath !== void 0) {
		w.stats.refs++;
		return { $ref: seenPath };
	}
	const exotic = walkExotic(obj);
	if (exotic) return exotic;
	if (depth >= w.opts.maxDepth) {
		w.stats.truncatedDepth++;
		return {
			$truncated: "depth",
			$preview: preview(obj),
			$path: segs
		};
	}
	w.seen.set(obj, path);
	if (Array.isArray(obj)) return walkArray(obj, w, depth, path, segs);
	if (ArrayBuffer.isView(obj)) return walkTypedArray(obj);
	if (obj instanceof Map) return walkMap(obj, w, depth, path, segs);
	if (obj instanceof Set) return walkSet(obj, w, depth, path, segs);
	return walkObject$1(obj, w, depth, path, segs);
}
/** Serialize the leaf scalar/function forms; `handled: false` means recurse into the object. */
function walkPrimitive(value, w) {
	if (value === null || value === void 0) return {
		handled: true,
		value: value ?? null
	};
	const t = typeof value;
	if (t === "string") {
		const s = value;
		return {
			handled: true,
			value: s.length > w.opts.maxString ? `${s.slice(0, w.opts.maxString)}… [$truncated string, ${s.length} chars]` : s
		};
	}
	if (t === "number") return {
		handled: true,
		value: Number.isFinite(value) ? value : String(value)
	};
	if (t === "boolean") return {
		handled: true,
		value
	};
	if (t === "bigint") return {
		handled: true,
		value: {
			$type: "bigint",
			value: String(value)
		}
	};
	if (t === "symbol") return {
		handled: true,
		value: {
			$type: "symbol",
			value: String(value)
		}
	};
	if (t === "function") return {
		handled: true,
		value: {
			$type: "function",
			name: value.name || "(anonymous)"
		}
	};
	return { handled: false };
}
/** Tag the cheap non-recursive exotic types, or `undefined` to keep walking. */
function walkExotic(obj) {
	if (obj instanceof Date) return {
		$type: "Date",
		value: Number.isNaN(obj.getTime()) ? "Invalid Date" : obj.toISOString()
	};
	if (obj instanceof RegExp) return {
		$type: "RegExp",
		value: String(obj)
	};
	if (obj instanceof URL) return {
		$type: "URL",
		value: obj.href
	};
	if (obj instanceof Error) return {
		$type: "Error",
		name: obj.name,
		message: obj.message
	};
	if (obj instanceof Promise) return { $type: "Promise" };
	for (const [ctor, tag] of OPAQUE_TAGS) if (obj instanceof ctor) return { $type: tag };
}
function walkArray(arr, w, depth, path, segs) {
	const source = w.opts.excludeFunctions ? arr.filter((item) => typeof item !== "function") : arr;
	const cap = Math.min(source.length, w.opts.maxEntries);
	const out = Array.from({ length: cap });
	for (let i = 0; i < cap; i++) out[i] = walk$1(source[i], w, depth + 1, `${path}[${i}]`, seg(segs, ["i", i]));
	if (source.length > cap) {
		w.stats.truncatedEntries++;
		out.push({
			$truncated: "entries",
			$total: source.length,
			$shown: cap
		});
	}
	return out;
}
function walkTypedArray(obj) {
	const view = obj;
	return {
		$type: obj.constructor?.name ?? "TypedArray",
		length: view.length ?? view.byteLength
	};
}
function walkMap(map, w, depth, path, segs) {
	const entries = [...map.entries()].slice(0, w.opts.maxEntries);
	if (map.size > entries.length) w.stats.truncatedEntries++;
	if (entries.every(([k]) => typeof k === "string")) {
		const value = {};
		for (const [k, v] of entries) value[k] = walk$1(v, w, depth + 1, `${path}.${String(k)}`, seg(segs, ["k", k]));
		return {
			$type: "Map",
			size: map.size,
			value
		};
	}
	return {
		$type: "Map",
		size: map.size,
		entries: entries.map(([k, v], i) => ({
			key: walk$1(k, w, depth + 1, `${path}~keys[${i}]`, seg(segs, ["mk", i])),
			value: walk$1(v, w, depth + 1, `${path}~values[${i}]`, seg(segs, ["mv", i]))
		}))
	};
}
function walkSet(set, w, depth, path, segs) {
	const values = [...set].slice(0, w.opts.maxEntries);
	if (set.size > values.length) w.stats.truncatedEntries++;
	return {
		$type: "Set",
		size: set.size,
		values: values.map((v, i) => walk$1(v, w, depth + 1, `${path}~set[${i}]`, seg(segs, ["s", i])))
	};
}
/** Plain object or class instance: own enumerable string-keyed props. */
function walkObject$1(obj, w, depth, path, segs) {
	const proto = Object.getPrototypeOf(obj);
	const className = proto && proto !== Object.prototype && proto !== null ? proto.constructor?.name : void 0;
	const out = {};
	if (className && className !== "Object") out.$class = className;
	const keys = Object.keys(obj).filter((key) => !isExcludedKey(key, w.opts));
	const cap = Math.min(keys.length, w.opts.maxProps);
	for (let i = 0; i < cap; i++) walkObjectKey(obj, keys[i], w, depth, path, segs, out);
	if (keys.length > cap) {
		w.stats.truncatedProps++;
		out.$truncated = `props: showing ${cap} of ${keys.length}`;
	}
	return out;
}
/** Read one own property (getters may throw) and walk it into `out`. */
function walkObjectKey(obj, key, w, depth, path, segs, out) {
	let v;
	try {
		v = obj[key];
	} catch (error) {
		out[key] = {
			$type: "getter-error",
			message: error instanceof Error ? error.message : String(error)
		};
		return;
	}
	if (w.opts.excludeFunctions && typeof v === "function") return;
	out[key] = walk$1(v, w, depth + 1, `${path}.${key}`, seg(segs, ["k", key]));
}
/** Append one structural step to a path, returning a fresh array. */
function seg(path, step) {
	return [...path, step];
}
function preview(obj) {
	if (Array.isArray(obj)) return `Array(${obj.length})`;
	if (obj instanceof Map) return `Map(${obj.size})`;
	if (obj instanceof Set) return `Set(${obj.size})`;
	const name = obj.constructor?.name ?? "Object";
	const keys = Object.keys(obj);
	return `${name} { ${keys.slice(0, 5).join(", ")}${keys.length > 5 ? ", …" : ""} }`;
}
//#endregion
//#region src/node/engine/query-engine.ts
function isMapTag(v) {
	return !!v && typeof v === "object" && v.$type === "Map";
}
function isSetTag(v) {
	return !!v && typeof v === "object" && v.$type === "Set";
}
/**
* Duck-typed Map detection: Vite 8's backward-compat `moduleGraph.idToModuleMap`
* is a Map-like facade (plain object with get/entries/size), not a real Map.
*/
function isMapLike$1(v) {
	return !!v && typeof v === "object" && typeof v.entries === "function" && typeof v.get === "function";
}
function isSetLike$1(v) {
	return !!v && typeof v === "object" && typeof v.has === "function" && typeof v[Symbol.iterator] === "function" && typeof v.get !== "function";
}
/**
* jora loads on first use and is cached for the process lifetime: a single
* `import('jora')` + `setup()`, however many queries follow.
*/
let createQueryPromise;
function getCreateQuery() {
	return createQueryPromise ??= import("./src-mTZRF6AU.mjs").then(({ default: jora }) => jora.setup({ methods: {
		/** Map(-like or normalized tag) -> plain object (string-coerced keys). */
		fromMap: (v) => {
			if (isMapLike$1(v)) return Object.fromEntries(v.entries());
			if (isMapTag(v)) return v.value ?? Object.fromEntries((v.entries ?? []).map((e) => [String(e.key), e.value]));
			return v;
		},
		/** Map(-like or normalized tag) -> [{ key, value }] preserving key identity. */
		mapEntries: (v) => {
			if (isMapLike$1(v)) return [...v.entries()].map(([key, value]) => ({
				key,
				value
			}));
			if (isMapTag(v)) {
				if (v.entries) return v.entries;
				return Object.entries(v.value ?? {}).map(([key, value]) => ({
					key,
					value
				}));
			}
			return [];
		},
		/** Set(-like or normalized tag) -> array. */
		fromSet: (v) => {
			if (isSetLike$1(v)) return [...v];
			if (isSetTag(v)) return v.values ?? [];
			return v;
		},
		/** Constructor name of any value. */
		typeOf: (v) => {
			if (v === null) return "null";
			if (typeof v !== "object") return typeof v;
			return v.constructor?.name ?? "Object";
		},
		/** All own keys (incl. non-enumerable), as strings. */
		ownKeys: (v) => v && typeof v === "object" ? Reflect.ownKeys(v).map(String) : []
	} }));
}
async function runQuery(target, query, options) {
	try {
		const started = performance.now();
		const raw = (await getCreateQuery())(query)(target);
		const queryMs = Math.round((performance.now() - started) * 100) / 100;
		const { data, stats } = normalize(raw, options);
		return {
			ok: true,
			result: data,
			stats: {
				queryMs,
				normalize: stats,
				payloadBytes: new TextEncoder().encode(JSON.stringify(data) ?? "").length
			}
		};
	} catch (error) {
		const e = error instanceof Error ? error : new Error(String(error));
		return {
			ok: false,
			error: {
				message: e.message,
				name: e.name
			}
		};
	}
}
/**
* Lazy-expand a depth-truncated node: re-run the base query against the live
* object, re-descend to the node the `NodePath` addresses, and normalize just
* that subtree with a fresh depth budget. The path comes from a `$truncated:
* 'depth'` marker the client is expanding, so the same filter options must be
* threaded through (they shift array indices and drop keys).
*/
async function runQueryAtPath(target, query, path, options) {
	try {
		const started = performance.now();
		const node = navigate((await getCreateQuery())(query)(target), path, options);
		const queryMs = Math.round((performance.now() - started) * 100) / 100;
		const { data, stats } = normalize(node, options);
		return {
			ok: true,
			result: data,
			stats: {
				queryMs,
				normalize: stats,
				payloadBytes: new TextEncoder().encode(JSON.stringify(data) ?? "").length
			}
		};
	} catch (error) {
		const e = error instanceof Error ? error : new Error(String(error));
		return {
			ok: false,
			error: {
				message: e.message,
				name: e.name
			}
		};
	}
}
/**
* jora stat mode: evaluates the (tolerant) query against the target and
* reports completions for the given cursor position. Each stat entry carries
* its candidates in a nested `suggestions` array, flattened here into plain,
* RPC-safe completion items.
*/
async function suggest(target, query, pos, limit = 30) {
	try {
		const started = performance.now();
		const raw = (await getCreateQuery())(query, {
			tolerant: true,
			stat: true
		})(target).suggestion(pos, { limit }) ?? [];
		const statMs = Math.round((performance.now() - started) * 100) / 100;
		const suggestions = [];
		for (const entry of raw) {
			for (const candidate of entry.suggestions ?? []) {
				suggestions.push({
					type: String(entry.type),
					from: entry.from,
					to: entry.to,
					current: String(entry.text ?? ""),
					value: String(candidate).slice(0, 200)
				});
				if (suggestions.length >= limit) break;
			}
			if (suggestions.length >= limit) break;
		}
		return {
			ok: true,
			suggestions,
			statMs
		};
	} catch (error) {
		return {
			ok: false,
			suggestions: [],
			statMs: 0,
			error: error instanceof Error ? error.message : String(error)
		};
	}
}
//#endregion
//#region src/node/engine/skeleton.ts
function skeletonOf(value, options = {}) {
	const started = performance.now();
	const walker = {
		seen: /* @__PURE__ */ new WeakSet(),
		nodes: 0,
		opts: {
			maxDepth: options.maxDepth ?? 5,
			maxProps: options.maxProps ?? 80,
			excludeFunctions: options.excludeFunctions ?? false,
			excludeUnderscoreProps: options.excludeUnderscoreProps ?? false,
			excludeDollarProps: options.excludeDollarProps ?? false
		}
	};
	return {
		skeleton: walk(value, walker, 0),
		nodes: walker.nodes,
		ms: Math.round((performance.now() - started) * 100) / 100
	};
}
function isMapLike(v) {
	return typeof v.entries === "function" && typeof v.get === "function" && typeof v.size === "number";
}
function isSetLike(v) {
	return typeof v.has === "function" && typeof v[Symbol.iterator] === "function" && typeof v.get !== "function" && typeof v.size === "number";
}
function walk(value, w, depth) {
	w.nodes++;
	if (value === null || value === void 0) return String(value);
	const t = typeof value;
	if (t !== "object") return t;
	const obj = value;
	if (obj instanceof Date) return "Date";
	if (obj instanceof RegExp) return "RegExp";
	if (obj instanceof URL) return "URL";
	if (obj instanceof Error) return "Error";
	if (obj instanceof Promise) return "Promise";
	if (w.seen.has(obj)) return "[circular]";
	if (depth >= w.opts.maxDepth) return "...";
	w.seen.add(obj);
	try {
		return walkObject(obj, w, depth);
	} finally {
		w.seen.delete(obj);
	}
}
function walkObject(obj, w, depth) {
	if (Array.isArray(obj)) return walkArraySkeleton(obj, w, depth);
	if (ArrayBuffer.isView(obj)) return obj.constructor?.name ?? "TypedArray";
	if (isMapLike(obj)) return walkMapSkeleton(obj, w, depth);
	if (isSetLike(obj)) return walkSetSkeleton(obj, w, depth);
	return walkPlainObject(obj, w, depth);
}
function walkArraySkeleton(arr, w, depth) {
	const items = w.opts.excludeFunctions ? arr.filter((item) => typeof item !== "function") : arr;
	if (items.length === 0) return [];
	const first = walk(items[0], w, depth + 1);
	return items.length > 1 ? [first, `+${items.length - 1} more`] : [first];
}
function walkMapSkeleton(map, w, depth) {
	const first = map.entries().next().value;
	if (!first) return `Map(0)`;
	return { [`Map(${map.size})`]: {
		key: walk(first[0], w, depth + 1),
		value: walk(first[1], w, depth + 1)
	} };
}
function walkSetSkeleton(set, w, depth) {
	const first = set[Symbol.iterator]().next().value;
	return first === void 0 ? `Set(0)` : { [`Set(${set.size})`]: walk(first, w, depth + 1) };
}
function walkPlainObject(obj, w, depth) {
	const proto = Object.getPrototypeOf(obj);
	const className = proto && proto !== Object.prototype ? proto.constructor?.name : void 0;
	const out = {};
	if (className && className !== "Object") out.$class = className;
	const keys = Object.keys(obj).filter((key) => !isExcludedKey(key, w.opts));
	const cap = Math.min(keys.length, w.opts.maxProps);
	for (let i = 0; i < cap; i++) walkPlainObjectKey(obj, keys[i], w, depth, out);
	if (keys.length > cap) out["..."] = `+${keys.length - cap} more props`;
	return out;
}
function walkPlainObjectKey(obj, key, w, depth, out) {
	let v;
	try {
		v = obj[key];
	} catch {
		out[key] = "getter-error";
		return;
	}
	if (w.opts.excludeFunctions && typeof v === "function") return;
	out[key] = walk(v, w, depth + 1);
}
//#endregion
//#region src/node/engine/write.ts
var WriteError = class extends Error {
	constructor(name, message) {
		super(message);
		this.name = name;
	}
};
/**
* Property names that reach or replace a shared prototype through ordinary
* property access (`__proto__`, `constructor.prototype`, …). Plain-object
* set/add/rename destinations reject these; Map keys are data, not property
* names, and never go through this check.
*/
const UNSAFE_OBJECT_KEYS = /* @__PURE__ */ new Set([
	"__proto__",
	"prototype",
	"constructor"
]);
/** Reject a plain-object property name that could reach a shared prototype. */
function assertSafeObjectKey(key) {
	if (UNSAFE_OBJECT_KEYS.has(key)) throw new WriteError("InvalidKey", `"${key}" is a prototype-sensitive property name`);
}
/**
* Create an own data property with a plain descriptor, bypassing any setter
* inherited from the prototype chain. Used for every write that introduces a
* property name the target doesn't already own (`add`, `rename`'s new key).
*/
function defineOwnDataProperty(target, key, value) {
	Object.defineProperty(target, key, {
		configurable: true,
		enumerable: true,
		writable: true,
		value
	});
}
/** Decode a discriminated wire value into the raw JS value to write. */
function decode(value) {
	return value.kind === "undefined" ? void 0 : value.value;
}
/** A JSON-expressible Map key / Set element (string-coerced object keys aside). */
function decodeKey(value, op) {
	if (!value) throw new WriteError("MissingKey", `"${op}" needs a key`);
	return decode(value);
}
/** Map a filtered array index back onto the real one (mirrors `navigate`). */
function realIndex(arr, filtered, opts) {
	if (!opts.excludeFunctions) return filtered;
	let seen = -1;
	for (let i = 0; i < arr.length; i++) {
		if (typeof arr[i] === "function") continue;
		seen++;
		if (seen === filtered) return i;
	}
	return -1;
}
function entryAt(map, index) {
	const entry = [...map.entries()][index];
	if (!entry) throw new WriteError("PathNotFound", `Map entry ${index} does not exist`);
	return entry;
}
function elementAt(set, index) {
	const values = [...set];
	if (index < 0 || index >= values.length) throw new WriteError("PathNotFound", `Set element ${index} does not exist`);
	return values[index];
}
function assertMutableObject(target) {
	if (Object.isFrozen(target)) throw new WriteError("FrozenTarget", "the target object is frozen");
}
/** Resolve the container a path's final segment applies to. */
function resolveParent(root, path, opts) {
	const parent = navigate(root, path.slice(0, -1), opts);
	if (parent === null || typeof parent !== "object") throw new WriteError("PathNotFound", "the path does not resolve to a container");
	return parent;
}
/** Set an own property, requiring it already exists and is writable. */
function setObjectProperty(parent, key, value) {
	assertMutableObject(parent);
	assertSafeObjectKey(key);
	if (!Object.hasOwn(parent, key)) throw new WriteError("PathNotFound", `property "${key}" does not exist`);
	const desc = Object.getOwnPropertyDescriptor(parent, key);
	if (!desc.writable && !desc.set) throw new WriteError("ReadonlyProperty", `property "${key}" has no setter`);
	const record = parent;
	record[key] = value;
}
function setArrayIndex(parent, at, value, opts) {
	if (!Array.isArray(parent)) throw new WriteError("WrongContainer", "an index step needs an array");
	const index = realIndex(parent, at, opts);
	if (index < 0 || index >= parent.length) throw new WriteError("PathNotFound", `array index ${at} does not exist`);
	parent[index] = value;
}
function setAt(parent, seg, value, opts) {
	const [kind, at] = seg;
	switch (kind) {
		case "k":
			if (parent instanceof Map) parent.set(at, value);
			else setObjectProperty(parent, at, value);
			return;
		case "i":
			setArrayIndex(parent, at, value, opts);
			return;
		case "s":
			if (!(parent instanceof Set)) throw new WriteError("WrongContainer", "a set step needs a Set");
			parent.delete(elementAt(parent, at));
			parent.add(value);
			return;
		case "mk": {
			if (!(parent instanceof Map)) throw new WriteError("WrongContainer", "a map-key step needs a Map");
			const [oldKey, entryValue] = entryAt(parent, at);
			parent.delete(oldKey);
			parent.set(value, entryValue);
			return;
		}
		case "mv": {
			if (!(parent instanceof Map)) throw new WriteError("WrongContainer", "a map-value step needs a Map");
			const [key] = entryAt(parent, at);
			parent.set(key, value);
		}
	}
}
function deleteObjectProperty(parent, key) {
	assertMutableObject(parent);
	if (!Object.hasOwn(parent, key)) throw new WriteError("PathNotFound", `property "${key}" does not exist`);
	if (!delete parent[key]) throw new WriteError("ReadonlyProperty", `property "${key}" cannot be deleted`);
}
function deleteArrayIndex(parent, at, opts) {
	if (!Array.isArray(parent)) throw new WriteError("WrongContainer", "an index step needs an array");
	const index = realIndex(parent, at, opts);
	if (index < 0 || index >= parent.length) throw new WriteError("PathNotFound", `array index ${at} does not exist`);
	parent.splice(index, 1);
}
function deleteAt(parent, seg, opts) {
	const [kind, at] = seg;
	switch (kind) {
		case "k":
			if (parent instanceof Map) {
				if (!parent.delete(at)) throw new WriteError("PathNotFound", `Map key "${at}" does not exist`);
			} else deleteObjectProperty(parent, at);
			return;
		case "i":
			deleteArrayIndex(parent, at, opts);
			return;
		case "s":
			if (!(parent instanceof Set)) throw new WriteError("WrongContainer", "a set step needs a Set");
			parent.delete(elementAt(parent, at));
			return;
		case "mk":
		case "mv": {
			if (!(parent instanceof Map)) throw new WriteError("WrongContainer", "a map-entry step needs a Map");
			const [key] = entryAt(parent, at);
			parent.delete(key);
		}
	}
}
function addTo(container, key, value, opts) {
	if (container instanceof Map) {
		container.set(decodeKey(key, "add"), value);
		return;
	}
	if (container instanceof Set) {
		container.add(value);
		return;
	}
	if (Array.isArray(container)) {
		const rawIndex = key ? decode(key) : void 0;
		if (rawIndex === void 0) {
			container.push(value);
			return;
		}
		if (typeof rawIndex !== "number" || !Number.isInteger(rawIndex)) throw new WriteError("InvalidKey", "an array insertion index must be an integer");
		const index = realIndex(container, rawIndex, opts);
		container.splice(index < 0 ? container.length : index, 0, value);
		return;
	}
	assertMutableObject(container);
	const propKey = decodeKey(key, "add");
	if (typeof propKey !== "string") throw new WriteError("InvalidKey", "an object property key must be a string");
	assertSafeObjectKey(propKey);
	defineOwnDataProperty(container, propKey, value);
}
function renameMapNamedKey(map, at, newKey) {
	if (!map.has(at)) throw new WriteError("PathNotFound", `Map key "${at}" does not exist`);
	if (newKey === at) return;
	const value = map.get(at);
	map.delete(at);
	map.set(newKey, value);
}
function renameObjectKey(parent, key, newKey) {
	assertMutableObject(parent);
	if (!Object.hasOwn(parent, key)) throw new WriteError("PathNotFound", `property "${key}" does not exist`);
	if (typeof newKey !== "string") throw new WriteError("InvalidKey", "an object property key must be a string");
	assertSafeObjectKey(newKey);
	if (newKey === key) return;
	const value = parent[key];
	delete parent[key];
	defineOwnDataProperty(parent, newKey, value);
}
function renameMapEntry(parent, at, newKey) {
	if (!(parent instanceof Map)) throw new WriteError("WrongContainer", "a map-entry step needs a Map");
	const [oldKey, value] = entryAt(parent, at);
	if (newKey === oldKey) return;
	parent.delete(oldKey);
	parent.set(newKey, value);
}
function renameAt(parent, seg, newKey) {
	const [kind, at] = seg;
	if (kind === "k") {
		if (parent instanceof Map) renameMapNamedKey(parent, at, newKey);
		else renameObjectKey(parent, at, newKey);
		return;
	}
	if (kind === "mk" || kind === "mv") {
		renameMapEntry(parent, at, newKey);
		return;
	}
	throw new WriteError("WrongContainer", "only keyed entries (objects, Maps) can be renamed");
}
/**
* Apply one write request to a live root object. Mutates in place;
* returns a named error outcome instead of throwing.
*/
function applyWrite(root, request, options = {}) {
	try {
		if (request.op === "add") {
			const container = navigate(root, request.path, options);
			if (container === null || typeof container !== "object") throw new WriteError("PathNotFound", "the path does not resolve to a container");
			addTo(container, request.key, decode(request.value), options);
			return { ok: true };
		}
		if (request.path.length === 0) throw new WriteError("InvalidPath", "the root itself cannot be replaced, deleted, or renamed");
		const parent = resolveParent(root, request.path, options);
		const seg = request.path[request.path.length - 1];
		switch (request.op) {
			case "set":
				setAt(parent, seg, decode(request.value), options);
				break;
			case "delete":
				deleteAt(parent, seg, options);
				break;
			case "rename": renameAt(parent, seg, decode(request.key));
		}
		return { ok: true };
	} catch (error) {
		const e = error instanceof Error ? error : new Error(String(error));
		return {
			ok: false,
			error: {
				name: e.name,
				message: e.message
			}
		};
	}
}
//#endregion
export { suggest as a, normalize as c, runQueryAtPath as i, skeletonOf as n, isExcludedKey as o, runQuery as r, navigate as s, applyWrite as t };
