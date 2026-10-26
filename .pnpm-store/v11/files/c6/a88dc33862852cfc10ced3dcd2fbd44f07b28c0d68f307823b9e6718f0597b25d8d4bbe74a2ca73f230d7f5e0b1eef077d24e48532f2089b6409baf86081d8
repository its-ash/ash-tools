import { i as parseOgMetadata, n as resolveMetadata, r as fetchOgMetadata, t as createResolveMetadataRpc } from "../resolve-metadata-Dx1MlNop.mjs";
//#region src/node/setup.ts
function setupOg(ctx, options = {}) {
	const hasOptions = options.defaultUrl !== void 0 || options.fetch !== void 0;
	ctx.rpc.register(hasOptions ? createResolveMetadataRpc(options) : resolveMetadata);
}
//#endregion
export { fetchOgMetadata, parseOgMetadata, setupOg };
