import {defineStore} from "pinia";
import {computed, ref} from "vue";
import {DataSource, DatasourceConfig, DownloadEvent} from "@/types";
import {Channel, invoke} from "@tauri-apps/api/core";

export const useDatasourceStore = defineStore('datasourceStore', () => {
    const datasourceConfig = ref<DatasourceConfig>()
    const activeDs = ref<DataSource>()
    const datasources = computed<DataSource[]>(() => {
        return datasourceConfig.value?.datasources ?? []
    })
    const defaultDs = computed(() => {
        return datasourceConfig.value?.datasources.filter(e => {
            return e.name === datasourceConfig.value?.active_ds
        })[0]
    })
    const defaultDsName = computed(() => {
        return datasourceConfig.value?.active_ds ?? ''
    })

    async function loadDs(setActive = false): Promise<void> {
        datasourceConfig.value = await invoke("list_data_sources");
        if (setActive) {
            activeDs.value = ({...defaultDs.value}) as Required<DataSource>
        }
        
    }

    async function addDatasource(ds: DataSource) {
        const val = {
            name: ds.name,
            bin: ds.bin.replace(/\\/g, '/'),
            host: ds.host,
            port: ds.port,
            user: ds.user,
            password: ds.password,
            is_active: false,
            is_ssh: false,
        }
        await invoke("add_ds", {ds: val});
        await loadDs()
    }

    async function editDatasource(ds: DataSource) {
        const val = {
            name: ds.name,
            bin: ds.bin.replace(/\\/g, '/'),
            host: ds.host,
            port: ds.port,
            user: ds.user,
            password: ds.password,
            is_active: false,
            is_ssh: false,
        }
        await invoke("edit_ds", {ds: val});
        await loadDs()
    }

    async function deleteDatasource(name: string) {

        await invoke("delete_ds", {dsName: name});
        await loadDs()
    }
    async function getbin() {
        const onEvent = new Channel<DownloadEvent>();
        await invoke("fetch_bins", {onEvent: onEvent,});

    }


    return {
        getbin,
        datasources,
        loadDs,
        defaultDsName,
        defaultDs,
        activeDs,
        addDatasource,
        editDatasource,
        deleteDatasource
    }
})