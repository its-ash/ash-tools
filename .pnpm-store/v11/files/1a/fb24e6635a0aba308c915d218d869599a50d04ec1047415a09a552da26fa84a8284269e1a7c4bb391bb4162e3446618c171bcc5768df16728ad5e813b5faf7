import { t as serverFunctions } from "../rpc-B1wXP13m.mjs";
//#region src/node/setup.ts
/**
* Register the inspector's introspection RPC functions on a devframe
* node context. Called from the definition's `setup(ctx)` and reusable
* by host adapters that wire their own context.
*/
function setupInspect(ctx) {
	for (const fn of serverFunctions) ctx.rpc.register(fn);
}
//#endregion
export { serverFunctions, setupInspect };
