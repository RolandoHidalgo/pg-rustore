<script setup lang="ts">
import {Button} from '@/components/ui/button'
import {computed, ref, toValue, watchEffect} from 'vue'
import {z} from 'zod'
import {useForm} from 'vee-validate'
import {toTypedSchema} from '@vee-validate/zod'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from '@/components/ui/form'

import {CardContent} from '@/components/ui/card'
import {Input} from '@/components/ui/input'
import {useAppStore} from '@/stores/appStore'
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger
} from '@/components/ui/sheet'


import {Switch} from '@/components/ui/switch'


import NewDbForm from "@/components/forms/NewDbForm.vue";
import DatasourceSelect from "@/components/dataSources/DatasourceSelect.vue";
import DbSelect from "@/components/forms/DbSelect.vue";
import FileDialogField from "@/components/forms/FileDialogField.vue";
import {Channel, invoke} from "@tauri-apps/api/core";
import {DownloadEvent, NewDbOptions, RestoreOptions} from "@/types";
import RestoreConsole from "@/components/forms/restore-console.vue";


const store = useAppStore()
const isConsoleOpen = ref(false)
const newDb = ref(false)
const isRestoring = ref(false)

const newDbSchema = z.object({
  encoding: z.string({
    required_error: 'Requerido.'
  }),
  template: z.string({
    required_error: 'Requerido.'
  }),
  collate: z.string({
    required_error: 'Requerido.'
  }),
  ctype: z.string({
    required_error: 'Requerido.'
  }),
  tablespace: z.string({
    required_error: 'Requerido.'
  })
})

const coneccionSchema = z.object({
  backupPath: z
      .string({
        required_error: 'Requerido.'
      }),
  dbName: z
      .string({
        required_error: 'Requerido.'
      })
      .min(1, {message: 'no vacio'})
})

const onInitConnSchema = z.object({

  dsName: z
      .string({
        required_error: 'Requerido.'
      })
      .min(1, {message: 'no vacio'})
})


const currentSchema = ref(coneccionSchema);

const finalSchema = computed(() => {
  let schema = toValue(currentSchema)
  if (store.currentOptions.dsName.length === 0) {
    schema = schema.merge(onInitConnSchema)
  }

  if (newDb.value) {
    schema = schema.merge(newDbSchema)
  }

  return schema.passthrough()
})
const {handleSubmit, values, resetForm, setFieldValue} = useForm({
  validationSchema: computed(() => toTypedSchema(finalSchema.value)),
  keepValuesOnUnmount: true
})
// const {isRestoreClone} = useRestoreCloneApi(currentSchema,newDb);
const isRestoreClone = ref(false);
// const backupInfo = useBackupInfo()
// const {isRestoreOnInit} = useRestoreOnInitApi(currentSchema)
const isRestoreOnInit = store.currentOptions.dsName.length === 0;


const onEvent = new Channel<DownloadEvent>();
const onSubmit = handleSubmit(async (values) => {


  const dbOptions = newDb.value ? ({
    character_type: values.ctype,
    tablespace: values.tablespace,
    collation: values.collate,
    template: values.template,
    encoding: values.encoding
  } as NewDbOptions) : undefined;

  const params: {
    dsName: string,
    restoreOptions: RestoreOptions,
    onEvent: Channel<DownloadEvent>
  } = {
    dsName: currentDsName.value,
    restoreOptions: {
      backup: values.backupPath,
      db_name: values.dbName,
      new_db_options: dbOptions
    },
    onEvent: onEvent
  }
  isConsoleOpen.value = true;
  isRestoring.value = true;
  console.log(params)
  await invoke("restore", params);
})

watchEffect(() => {
  if (!store.isRestoreOpen) {
    isConsoleOpen.value = false
    resetForm()
    //setFieldValue('backupPath',undefined)
    isRestoring.value = false
    newDb.value = false
  }
})

watchEffect(() => {
  if (store.currentOptions.backupPath && store.currentOptions.backupPath !== '') {
    setFieldValue('backupPath', store.currentOptions.backupPath)
  }
})
const currentDsName = computed<string>(() => {

  return store.currentOptions.dsName.length > 0 ? store.currentOptions.dsName : values.dsName as string;
})


</script>

<template>
  <Sheet v-model:open="store.isRestoreOpen">
    <SheetTrigger></SheetTrigger>
    <SheetContent side="bottom" class="rounded-t-lg">
      <SheetHeader class="pb-0">
        <SheetTitle>Restaurar backup</SheetTitle>
        <SheetDescription> LLene los campos</SheetDescription>
<!--        <div v-if="store.currentOptions.backupPath">{{ store.currentOptions.backupPath }}</div>-->
      </SheetHeader>
      <form class="w-full flex flex-col pb-0" @submit="onSubmit">
        <CardContent class="grid grid-cols-2 gap-2 overflow-y-auto pb-0" v-if="!isConsoleOpen">
          <div class="col-span-2">
            <FormField name="newDb">
              <FormItem class="flex flex-row items-center justify-between rounded-lg border p-4">
                <div class="space-y-0.5">
                  <FormLabel class="text-base"> Nueva</FormLabel>
                  <FormDescription> Crear una nueva db.</FormDescription>
                </div>
                <FormControl>
                  <Switch
                      id="new-db"
                      v-model="newDb"

                  />
                </FormControl>
              </FormItem>
            </FormField>
          </div>
          <div v-if="isRestoreOnInit" class="col-span-2">
            <DatasourceSelect/>
          </div>
          <div v-if="!newDb" class="col-span-2">
            <DbSelect :ds-name="currentDsName ?? ''"/>
          </div>

          <div v-else>
            <FormField v-slot="{ componentField }" name="dbName">
              <FormItem>
                <FormLabel>DB</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField"/>
                </FormControl>
                <FormMessage/>
              </FormItem>
            </FormField>
          </div>

          <NewDbForm v-if="newDb"/>

          <div class="col-span-2" v-if="!isRestoreClone">

            <FormField name="backupPath">
              <FileDialogField name="backupPath" label="Backup file"/>
            </FormField>
          </div>
        </CardContent>
        <RestoreConsole v-else @done="isRestoring=false" :channel="onEvent"/>
      </form>

      <SheetFooter class="pt-0">
        <Button @click="onSubmit" :disabled="isRestoring || isConsoleOpen"> Aceptar</Button>
      </SheetFooter>
    </SheetContent>
  </Sheet>
</template>

<style scoped></style>
