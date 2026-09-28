import {useAppStore} from '@/stores/appStore'
import {Ref, ref, watchEffect} from 'vue'
import {invoke} from "@tauri-apps/api/core";

export type BackupInfo = {
    fecha: string
    entries: number
    db_version: string
    pg_dump_version: string
    db_name: string
}

const useBackupInfo = (value: Ref<string>) => {

    const store = useAppStore()
    const backupInfo = ref<BackupInfo>({
        fecha: '',
        entries: 0,
        db_version: '',
        pg_dump_version: '',
        db_name: ''
    })
    const parseBackup = async (backupPath: string): Promise<BackupInfo> => {

        const params = {
            path: backupPath
        }

        return await invoke("backup_info", params);
    }

    watchEffect(async () => {
        if (value.value?.length > 0) {
            backupInfo.value = await parseBackup(value.value);
            console.log(backupInfo)
        }
    })

    watchEffect(() => {
        if (!store.isRestoreOpen) {
            backupInfo.value = {
                fecha: '',
                entries: 0,
                db_version: '',
                pg_dump_version: '',
                db_name: ''
            }
        }
    })

    return backupInfo
}

export default useBackupInfo
