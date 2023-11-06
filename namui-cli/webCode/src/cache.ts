import { get, set } from "idb-keyval";

function cachePrefix(key: string) {
    return `cache-${key}`;
}

export function cacheGet(key: string) {
    return get(cachePrefix(key));
}

export function cacheSet(key: string, value: any) {
    return set(cachePrefix(key), value);
}
