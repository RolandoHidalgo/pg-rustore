<script setup lang="ts">
import {Button} from '@/components/ui/button'
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
import {ref} from 'vue'
//import RestoreConsole from '@renderer/components/restore-console.vue'
import {DatabaseBackup, Unplug, Network} from 'lucide-vue-next'
import {Channel, invoke} from "@tauri-apps/api/core";
import {DownloadEvent} from "@/types";
import RestoreConsole from "@/components/forms/restore-console.vue";
import {useDbStore} from "@/stores/dbStore.ts";


const store = useAppStore()
const dbStore = useDbStore();
const isConsoleOpen = ref(false)
const isDropping = ref(false)

let onEvent = new Channel<DownloadEvent>();

async function handleDrop() {
  if (!isConsoleOpen.value) {
    isDropping.value = true;
    isConsoleOpen.value = true;

    let ch = new Channel<DownloadEvent>();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    onEvent = ch;
    await invoke("drop", {
      dsName: store.currentOptions.dsName,
      dbName: store.currentOptions.dbName,
      schemaName: store.currentOptions.schema,
      onEvent: ch,
    });

  } else {
    store.isDropOpen = false;
  }

}

// const coneccionSchema = z.object({
//   schemma: z
//     .string({
//       required_error: 'Requerido.'
//     })
//     .optional()
// })
// const { handleSubmit } = useForm({
//   validationSchema: computed(() => toTypedSchema(coneccionSchema.passthrough())),
//   keepValuesOnUnmount: true
// })

// const onSubmit = handleSubmit((values) => {
//   if(!isConsoleOpen.value) {
//     handleBackup()
//   }else {
//     store.isBackupOpen = false;
//   }
// })
function handleOpenChange(open: boolean) {
  if (!open) {
    isConsoleOpen.value = false;
    isDropping.value = false;
    //onEvent = new Channel<DownloadEvent>();
  }
}

function onFinish() {
  isDropping.value = false
  dbStore.loadDbs(store.currentOptions.dsName)
  store.isDropOpen = false;
  isConsoleOpen.value = false;


}

</script>

<template>
  <Sheet v-model:open="store.isDropOpen" @update:open="handleOpenChange($event)">
    <SheetTrigger></SheetTrigger>
    <SheetContent side="bottom" class="rounded-t-lg">
      <SheetHeader>
        <SheetTitle>Eliminar Db</SheetTitle>
        <SheetDescription> Se eliminará la db con estos parámetros.</SheetDescription>
      </SheetHeader>
      <form class="grid gap-4 px-4">
        <div class="grid grid-cols-2 gap-2 overflow-y-auto pb-0" v-if="!isConsoleOpen">
          <div class=" flex items-center justify-center">
            <Unplug class="h-6 w-6 text-primary mr-2"/>
            <div class="grid flex-1 text-left text-sm leading-tight">
              <span class="truncate font-semibold">{{ store.currentOptions.dsName }}</span>
              <span class="truncate text-xs">datasource</span>
            </div>
          </div>
          <div class=" flex items-center justify-center">
            <DatabaseBackup class="h-6 w-6 text-primary mr-2"/>
            <div class="grid flex-1 text-left text-sm leading-tight">
              <span class="truncate font-semibold">{{ store.currentOptions.dbName }}</span>
              <span class="truncate text-xs">db</span>
            </div>
          </div>
          <div class=" flex items-center justify-center">
            <Network class="h-6 w-6 text-primary mr-2"/>
            <div
                class="grid flex-1 text-left text-sm leading-tight"

            >
              <span class="truncate font-semibold">{{
                  store.currentOptions?.schema !== '' ? store.currentOptions.schema : '-'
                }}</span>
              <span class="truncate text-xs">
                schema
              </span>
            </div>
          </div>
        </div>
        <RestoreConsole v-else @done="onFinish" :channel="onEvent"/>

      </form>
      <SheetFooter>
        <Button variant="destructive" @click.prevent="handleDrop" :disabled="isDropping || isConsoleOpen"> Eliminar db
        </Button>
      </SheetFooter>
    </SheetContent>
  </Sheet>
</template>

<style scoped></style>
