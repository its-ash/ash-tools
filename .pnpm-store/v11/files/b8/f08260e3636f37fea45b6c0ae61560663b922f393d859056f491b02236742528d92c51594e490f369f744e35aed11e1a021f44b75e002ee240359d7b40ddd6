//#region src/node/types.d.ts
type OgHeadTagName = 'html' | 'link' | 'meta' | 'title';
interface OgHeadTag {
  tag: OgHeadTagName;
  name: string;
  value: string;
}
interface OgSnapshot {
  requestedUrl: string;
  url: string;
  status: number;
  fetchedAt: number;
  tags: OgHeadTag[];
}
interface OgResolveInput {
  url?: string;
}
type OgFetch = (input: string, init: RequestInit) => Promise<Response>;
//#endregion
export { OgSnapshot as a, OgResolveInput as i, OgHeadTag as n, OgHeadTagName as r, OgFetch as t };