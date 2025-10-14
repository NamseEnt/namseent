/// <reference types="vite/client" />

declare const __IMAGE_COUNT__: number;

declare module "virtual:asset-list" {
    export interface AssetInfo {
        id: number;
        path: string;
    }
    export const assetList: AssetInfo[];
}
