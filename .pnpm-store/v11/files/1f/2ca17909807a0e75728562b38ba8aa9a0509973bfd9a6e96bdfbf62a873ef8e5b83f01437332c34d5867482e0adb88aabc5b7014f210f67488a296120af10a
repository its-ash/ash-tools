import { defineRpcFunction } from "devframe";
import { s } from "devframe/utils/simple-schema";
import { defineDiagnostics } from "devframe/utils/nostics";
import { parse } from "parse5";
//#region src/node/diagnostics.ts
const diagnostics = defineDiagnostics({
	docsBase: "https://devfra.me/errors",
	codes: {
		DP_OG_0001: {
			why: (p) => `Cannot inspect "${p.url}" because it is not a valid HTTP or HTTPS URL.`,
			fix: "Enter an absolute URL, such as `http://localhost:3000/about`."
		},
		DP_OG_0002: {
			why: (p) => `Could not fetch Open Graph metadata from "${p.url}": ${p.reason}`,
			fix: "Check that the target server is running and reachable from the devframe process."
		},
		DP_OG_0003: {
			why: (p) => `The target "${p.url}" returned HTTP ${p.status}.`,
			fix: "Inspect a route that returns a successful HTML response."
		},
		DP_OG_0004: {
			why: (p) => `The HTML response from "${p.url}" is ${p.size} bytes, which exceeds the 2 MB inspection limit.`,
			fix: "Inspect a smaller HTML document or reduce the response payload."
		},
		DP_OG_0005: {
			why: "A static Open Graph report requires a default URL to inspect.",
			fix: "Pass `defaultUrl` to `createOgDevframe()` before running the build adapter."
		}
	}
});
//#endregion
//#region src/node/metadata.ts
const MAX_HTML_BYTES = 2097152;
const URL_TAGS = /* @__PURE__ */ new Set([
	"canonical",
	"icon",
	"og:image",
	"og:image:secure_url",
	"og:url",
	"twitter:image"
]);
function getAttribute(node, name) {
	return node.attrs?.find((attribute) => attribute.name === name)?.value ?? "";
}
function findElement(node, name) {
	if (node.tagName === name) return node;
	for (const child of node.childNodes ?? []) {
		const match = findElement(child, name);
		if (match) return match;
	}
}
function textContent(node) {
	if (node.nodeName === "#text") return node.value ?? "";
	return (node.childNodes ?? []).map(textContent).join("");
}
function resolveTagValue(name, value, baseUrl) {
	if (!value || !URL_TAGS.has(name) && !name.endsWith(":image")) return value;
	try {
		const resolved = new URL(value, baseUrl);
		return resolved.protocol === "http:" || resolved.protocol === "https:" ? resolved.href : "";
	} catch {
		return "";
	}
}
function parseOgMetadata(html, url) {
	const document = parse(html);
	const htmlElement = findElement(document, "html");
	const head = findElement(document, "head");
	if (!head) return [];
	let baseUrl = url;
	const baseHref = findElement(head, "base");
	const baseValue = baseHref ? getAttribute(baseHref, "href") : "";
	if (baseValue) try {
		baseUrl = new URL(baseValue, url).href;
	} catch {}
	const tags = [];
	if (htmlElement) {
		const lang = getAttribute(htmlElement, "lang");
		if (lang) tags.push({
			tag: "html",
			name: "lang",
			value: lang
		});
	}
	function visit(node) {
		if (node.tagName === "title") {
			const value = textContent(node).trim();
			if (value) tags.push({
				tag: "title",
				name: "title",
				value
			});
		} else if (node.tagName === "meta") {
			const property = getAttribute(node, "property");
			const metaName = getAttribute(node, "name");
			const charset = getAttribute(node, "charset");
			const name = (property || metaName || (charset ? "charset" : "")).toLowerCase();
			const value = resolveTagValue(name, getAttribute(node, "content") || charset, baseUrl);
			if (name && value) tags.push({
				tag: "meta",
				name,
				value
			});
		} else if (node.tagName === "link") {
			const name = getAttribute(node, "rel").toLowerCase();
			const value = resolveTagValue(name, getAttribute(node, "href"), baseUrl);
			if (name && value) tags.push({
				tag: "link",
				name,
				value
			});
		}
		for (const child of node.childNodes ?? []) visit(child);
	}
	visit(head);
	return tags;
}
function normalizeUrl(input) {
	const candidate = /^[a-z][a-z\d+.-]*:\/\//i.test(input) ? input : `http://${input}`;
	try {
		const url = new URL(candidate);
		if (url.protocol !== "http:" && url.protocol !== "https:") throw new TypeError("Unsupported protocol");
		return url.href;
	} catch {
		throw diagnostics.DP_OG_0001({ url: input });
	}
}
async function readHtml(response, url) {
	const declaredSize = Number(response.headers.get("content-length"));
	if (Number.isFinite(declaredSize) && declaredSize > MAX_HTML_BYTES) {
		await response.body?.cancel().catch(() => {});
		throw diagnostics.DP_OG_0004({
			url,
			size: declaredSize
		});
	}
	const chunks = [];
	let size = 0;
	const reader = response.body?.getReader();
	if (reader) while (true) {
		const { done, value } = await reader.read();
		if (done) break;
		size += value.byteLength;
		if (size > MAX_HTML_BYTES) {
			await reader.cancel().catch(() => {});
			throw diagnostics.DP_OG_0004({
				url,
				size
			});
		}
		chunks.push(value);
	}
	const buffer = new Uint8Array(size);
	let offset = 0;
	for (const chunk of chunks) {
		buffer.set(chunk, offset);
		offset += chunk.byteLength;
	}
	const headerCharset = (response.headers.get("content-type") ?? "").match(/charset\s*=\s*["']?([^\s;"']+)/i)?.[1];
	const documentCharset = new TextDecoder("windows-1252").decode(buffer.subarray(0, 2048)).match(/<meta\s[^>]*charset\s*=\s*["']?([^\s;"'/>]+)/i)?.[1];
	try {
		return new TextDecoder(headerCharset || documentCharset || "utf-8").decode(buffer);
	} catch {
		return new TextDecoder().decode(buffer);
	}
}
async function fetchOgMetadata(input, fetcher = globalThis.fetch) {
	const requestedUrl = normalizeUrl(input.trim());
	let response;
	try {
		response = await fetcher(requestedUrl, {
			headers: {
				"accept": "text/html,application/xhtml+xml",
				"user-agent": "devframe-og/0.7"
			},
			redirect: "follow",
			signal: AbortSignal.timeout(1e4)
		});
	} catch (error) {
		throw diagnostics.DP_OG_0002({
			url: requestedUrl,
			reason: error instanceof Error ? error.message : String(error),
			cause: error
		});
	}
	if (!response.ok) {
		await response.body?.cancel().catch(() => {});
		throw diagnostics.DP_OG_0003({
			url: requestedUrl,
			status: response.status
		});
	}
	const resolvedUrl = response.url || requestedUrl;
	let html;
	try {
		html = await readHtml(response, requestedUrl);
	} catch (error) {
		if (error instanceof Error && error.name === "DP_OG_0004") throw error;
		throw diagnostics.DP_OG_0002({
			url: requestedUrl,
			reason: error instanceof Error ? error.message : String(error),
			cause: error
		});
	}
	return {
		requestedUrl,
		url: resolvedUrl,
		status: response.status,
		fetchedAt: Date.now(),
		tags: parseOgMetadata(html, resolvedUrl)
	};
}
//#endregion
//#region src/node/rpc/functions/resolve-metadata.ts
const EMPTY_SNAPSHOT = {
	requestedUrl: "",
	url: "",
	status: 0,
	fetchedAt: 0,
	tags: []
};
const tagSchema = s.object({
	tag: s.picklist([
		"html",
		"link",
		"meta",
		"title"
	]),
	name: s.string(),
	value: s.string()
});
const snapshotSchema = s.object({
	requestedUrl: s.string(),
	url: s.string(),
	status: s.number(),
	fetchedAt: s.number(),
	tags: s.array(tagSchema)
});
function createResolveMetadataRpc(options = {}) {
	return defineRpcFunction({
		name: "devframes:plugin:og:resolve-metadata",
		type: "query",
		jsonSerializable: true,
		args: [s.object({ url: s.optional(s.string()) })],
		returns: snapshotSchema,
		agent: {
			title: "Inspect Open Graph metadata",
			description: "Fetch an HTTP or HTTPS page and return its normalized title, language, Open Graph, Twitter, and link metadata.",
			safety: "read",
			tags: ["open-graph", "seo"]
		},
		dump: async (_ctx, handler) => {
			if (!options.defaultUrl) throw diagnostics.DP_OG_0005();
			const input = { url: options.defaultUrl };
			const snapshot = await handler(input);
			return {
				records: [{
					inputs: [input],
					output: snapshot
				}],
				fallback: snapshot
			};
		},
		setup: () => ({ 
		/**
		* The RPC runtime awaits handlers before validating `returns`; its public
		* setup type currently models schema-backed returns as synchronous.
		*/
handler: (async ({ url = "" }) => {
			const target = url.trim() || options.defaultUrl?.trim();
			if (!target) return EMPTY_SNAPSHOT;
			return fetchOgMetadata(target, options.fetch);
		}) })
	});
}
const resolveMetadata = createResolveMetadataRpc();
//#endregion
export { parseOgMetadata as i, resolveMetadata as n, fetchOgMetadata as r, createResolveMetadataRpc as t };
