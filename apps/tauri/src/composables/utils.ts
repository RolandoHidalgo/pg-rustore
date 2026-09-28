import {MaybeRefOrGetter, ref, Ref, toValue, watchEffect} from "vue";
import {useDbStore} from "@/stores/dbStore.ts";

const useSchemas = (_dsName: MaybeRefOrGetter<string>, _dbName: MaybeRefOrGetter<string>) => {
    const dbStore = useDbStore()
    const schemas: Ref<string[]> = ref([])
    const loadingSchemas = ref(false)

    watchEffect(async () => {
        const dsName = toValue(_dsName)
        const dbName = toValue(_dbName)
        if (dsName && dsName !== '' && dbName && dbName !== '') {
            loadingSchemas.value = true
            schemas.value = await dbStore.getSchemasOfDbInDataSource(dsName, dbName)
            loadingSchemas.value = false
        }
    })
    return { schemas, loadingSchemas }
}
export {useSchemas}