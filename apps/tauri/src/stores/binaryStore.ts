import {defineStore} from "pinia";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Asset, Release} from "@/types";

export const useBinaryStore = defineStore('binaryStore', () => {
    const remoteBinaries = ref<Asset[]>([])
    const isFetching = ref(false);

    async function getRemoteBinaries() {
        console.log('fetching');
        isFetching.value = true
        const release: Release = await invoke("fetch_bins");
        remoteBinaries.value = release.assets
        isFetching.value = false
    }

    return {
        getRemoteBinaries,
        isFetching,
        remoteBinaries
    }
});