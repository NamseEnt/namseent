/// <reference types="vite/client" />

declare module "virtual:asset-list" {
    export interface AssetInfo {
        id: number;
        path: string;
    }
    export const assetList: AssetInfo[];
}
