import {defineStore} from "pinia";
import {ref} from "vue";
import {BinaryInfo, DataSource} from "@/types";
import {invoke} from "@tauri-apps/api/core";

export const useAppStore = defineStore('appStore', () => {
    const isBackupOpen = ref(false)
    const isRestoreOpen = ref(false)
    const isBinariesOpen = ref(false)
    const isDataSourceFormOpen = ref(false)
    const currentDsForm = ref<DataSource | null>(null)
    const currentOptions = ref<{
        dsName: string,
        dbName: string,
        schema: string,
        backupPath?: string,
        isClone: boolean
    }>({
        dsName: '',
        dbName: '',
        schema: '',
        backupPath: undefined,
        isClone: false
    })

    function openBackup(dsName: string, dbName: string, schema: string = ''): void {
        currentOptions.value.dbName = dbName
        currentOptions.value.dsName = dsName
        currentOptions.value.schema = schema
        isBackupOpen.value = true
    }

    function openBinaries(): void {

        isBinariesOpen.value = true
    }

    async function getBinaries(): Promise<BinaryInfo[]> {
        return await invoke("get_binaries");
    }

    function openDataSourceForm(ds: (DataSource | null) = null) {

        if (ds) {
            currentDsForm.value = ds
        }
        isDataSourceFormOpen.value = true
    }

    function openRestore(dsName: string,path?:string): void {
        currentOptions.value.dsName = dsName
        currentOptions.value.backupPath = path;
        isRestoreOpen.value = true
    }

    return {
        isRestoreOpen,
        openRestore,
        openBinaries,
        isBinariesOpen,
        isBackupOpen,
        currentOptions,
        openBackup,
        getBinaries,
        openDataSourceForm,
        currentDsForm,
        isDataSourceFormOpen
    }
})