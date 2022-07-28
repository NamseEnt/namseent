import { get, set } from "./idb-keyval.6.2.0/index.js";

function cachePrefix(key) {
    return `cache-${key}`;
}

export function cacheGet(key) {
    return get(cachePrefix(key));
}

export function cacheSet(key, value) {
    return set(cachePrefix(key), value);
}