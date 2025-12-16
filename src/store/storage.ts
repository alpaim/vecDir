import type { StateStorage } from "zustand/middleware";
import { LazyStore } from "@tauri-apps/plugin-store";
import { createJSONStorage } from "zustand/middleware";

export const fileStore = new LazyStore("app-state.json");

const hybridStorage: StateStorage = {
    // trying to get item from LocalStorage for blazzzingly fast native access
    getItem: async (name: string): Promise<string | null> => {
        const localValue = localStorage.getItem(name);

        if (localValue) {
            return localValue;
        }

        // if there are no item on LocalStorage, accessing Tauri

        try {
            await fileStore.init();

            const fileValue = await fileStore.get<string>(name);

            if (fileValue) {
                return fileValue;
            }

            return null;
        }
        catch (e) {
            console.error("HybridStorage: Failed to get item from disk", e);

            return null;
        }
    },

    // saving to BOTH places: LocalStorage and Tauri
    setItem: async (name: string, value: string): Promise<void> => {
        localStorage.setItem(name, value);

        try {
            await fileStore.set(name, value);
            await fileStore.save();
        }
        catch (e) {
            console.error("Failed to save to disk:", e);
        }
    },

    // 3. Deleting from BOTH places
    removeItem: async (name: string): Promise<void> => {
        localStorage.removeItem(name);
        try {
            await fileStore.delete(name);
            await fileStore.save();
        }
        catch (e) {
            console.error("Failed to delete from disk:", e);
        }
    },
};

export const storage = createJSONStorage(() => hybridStorage);
