/// <reference types="vite/client" />

declare module "virtual:asset-list" {
    export interface AssetInfo {
        id: number;
        path: string;
    }
    export const assetList: AssetInfo[];
}

declare module "virtual:font-asset" {
    export interface FontInfo {
        name: string;
        weight: string;
        path: string;
    }
    export const fontAsset: FontInfo[];
}
