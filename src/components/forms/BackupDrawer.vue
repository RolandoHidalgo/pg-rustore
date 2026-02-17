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
import {Ref, ref} from 'vue'
//import RestoreConsole from '@renderer/components/restore-console.vue'
import {DatabaseBackup, Unplug, Network} from 'lucide-vue-next'
import {Channel, invoke} from "@tauri-apps/api/core";
import {DownloadEvent} from "@/types";
import RestoreConsole from "@/components/forms/restore-console.vue";


const store = useAppStore()
const isConsoleOpen = ref(false)
const isBackingUp = ref(false)

let onEvent = new Channel<DownloadEvent>();

async function handleBackup() {
  if (!isConsoleOpen.value) {
    isBackingUp.value = true;
    isConsoleOpen.value = true;


    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    await invoke("backup", {
      dsName: store.currentOptions.dsName,
      dbName: store.currentOptions.dbName,
      schemaName: store.currentOptions.schema,
      onEvent: onEvent,
    });

  } else {
    store.isBackupOpen = false;
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
function handleOpenChange(open:boolean){
  if(!open){
    isConsoleOpen.value = false;
    isBackingUp.value = false;
    onEvent = new Channel<DownloadEvent>();
  }
}
</script>

<template>
  <Sheet v-model:open="store.isBackupOpen" @update:open="handleOpenChange($event)">
    <SheetTrigger></SheetTrigger>
    <SheetContent side="bottom" class="rounded-t-lg">
      <SheetHeader>
        <SheetTitle>Crear backup</SheetTitle>
        <SheetDescription> Se creará un backup con estos parámetros.</SheetDescription>
      </SheetHeader>
      <form class="grid gap-4 px-4">
        <div class="grid grid-cols-2 gap-2 overflow-y-auto pb-0" v-if="!isConsoleOpen">
          <div class=" flex items-center justify-center">
            <Unplug class="h-6 w-6 text-primary mr-2"/>
            <div class="grid flex-1 text-left text-sm leading-tight">
              <span class="truncate font-semibold">DataSource</span>
              <span class="truncate text-xs">{{ store.currentOptions.dsName }}</span>
            </div>
          </div>
          <div class=" flex items-center justify-center">
            <DatabaseBackup class="h-6 w-6 text-primary mr-2"/>
            <div class="grid flex-1 text-left text-sm leading-tight">
              <span class="truncate font-semibold">DB</span>
              <span class="truncate text-xs">{{ store.currentOptions.dbName }}</span>
            </div>
          </div>
          <div class=" flex items-center justify-center">
            <Network class="h-6 w-6 text-primary mr-2"/>
            <div
                class="grid flex-1 text-left text-sm leading-tight"

            >
              <span class="truncate font-semibold">schema</span>
              <span class="truncate text-xs">{{
                  store.currentOptions?.schema !== '' ? store.currentOptions.schema : '-'
                }}</span>
            </div>
          </div>
        </div>
        <RestoreConsole v-else @done="isBackingUp=false" :channel="onEvent"/>

      </form>
      <SheetFooter>
        <Button @click.prevent="handleBackup" :disabled="isBackingUp || isConsoleOpen"> Crear backup</Button>
      </SheetFooter>
    </SheetContent>
  </Sheet>
</template>

<style scoped></style>
