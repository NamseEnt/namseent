const API_KEY_STORAGE_KEY = "yhm_google_ai_api_key";

export const storage = {
    getApiKey(): string | null {
        if (typeof window === "undefined") return null;
        return localStorage.getItem(API_KEY_STORAGE_KEY);
    },

    setApiKey(key: string): void {
        if (typeof window === "undefined") return;
        localStorage.setItem(API_KEY_STORAGE_KEY, key);
    },

    removeApiKey(): void {
        if (typeof window === "undefined") return;
        localStorage.removeItem(API_KEY_STORAGE_KEY);
    },

    hasApiKey(): boolean {
        return this.getApiKey() !== null;
    },
};
