//#region src/node/cli.d.ts
/** The static-export dataset consumed by the SPA in static mode. */
export interface StaticDataset {
  sources: Array<{
    id: string;
    title: string;
    description?: string;
    icon?: string;
    static: boolean;
    queries?: {
      query: string;
      title?: string;
      description?: string;
    }[];
    data: unknown;
  }>;
}
export declare function createDataInspectorCli(): {
  cli: import("cac").CAC;
  parse: (argv?: string[]) => Promise<void>;
};
//#endregion