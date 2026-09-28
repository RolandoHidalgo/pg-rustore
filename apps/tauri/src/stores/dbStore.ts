import {defineStore} from "pinia";
import {computed, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

export const useDbStore = defineStore('dbStore', () => {
    const dbs = ref<string[]>([])
    const isLoadingDbs = ref(false);
    const conexionErrors = ref('')
    const hasConexionErrors = computed(() => {
        return conexionErrors.value && conexionErrors.value !== ''
    })

    async function loadDbs(dsName: string) {
        try {
            isLoadingDbs.value = true;
            conexionErrors.value = ''
            dbs.value = await invoke("list_db", {name: dsName});

        } catch (e: any) {
            conexionErrors.value = e
        } finally {
            isLoadingDbs.value = false
        }
    }
    async function getSchemasOfDbInDataSource(dsName: string, dbName: string): Promise<string[]> {
        return await invoke("list_db_schemas", {dbName,dsName: dsName,});
    }

    return {dbs, loadDbs, hasConexionErrors, isLoadingDbs,getSchemasOfDbInDataSource}
})