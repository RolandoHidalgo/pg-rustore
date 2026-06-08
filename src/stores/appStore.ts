import {defineStore} from "pinia";
import {ref} from "vue";
import {BinaryInfo, DataSource} from "@/types";
import {invoke} from "@tauri-apps/api/core";
import {deletePassword, getPassword, setPassword} from "tauri-plugin-keyring-api";


const KEY_SERVICE_NAME = 'PG_RUSTORE'
const KEY_USER_NAME = 'ADMIN'
//bunconst p = await getPassword(KEY_SERVICE_NAME, KEY_USER_NAME);
export const useAppStore = defineStore('appStore', () => {
    const isAuth = ref(false);
    const isBackupOpen = ref(false)
    const isRestoreOpen = ref(false)
    const isBinariesOpen = ref(false)
    const isAboutOpen = ref(false)
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


    async function getAuth() {
        return await getPassword(KEY_SERVICE_NAME, KEY_USER_NAME);
    }

    async function login(passwd: string) {

        const old = await getPassword(KEY_SERVICE_NAME, KEY_USER_NAME);
        if (!old) {
            await setPassword(KEY_SERVICE_NAME, KEY_USER_NAME, passwd);
            isAuth.value = true
            return true
        } else {
            const valid = old === passwd;
            if (valid) {
                isAuth.value = true
            }
            return valid;
        }


    }

    async function deletePass() {

        await deletePassword(KEY_SERVICE_NAME, KEY_USER_NAME);
        isAuth.value = false;


    }

    function openAbout(): void {

        isAboutOpen.value = true
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

    function openRestore(dsName: string, path?: string): void {
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
        isAboutOpen,
        openDataSourceForm,
        currentDsForm,
        openAbout,
        isAuth,
        getAuth,
        login,
        deletePass,
        isDataSourceFormOpen
    }
})