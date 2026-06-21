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


import {Label} from '@/components/ui/label'
import {computed, ref} from 'vue'
//import RestoreConsole from '@renderer/components/restore-console.vue'
import {DatabaseBackup, Unplug, Network} from 'lucide-vue-next'
import {Channel, invoke} from "@tauri-apps/api/core";
import {DownloadEvent} from "@/types";
import RestoreConsole from "@/components/forms/restore-console.vue";
import {Checkbox} from "@/components/ui/checkbox";
import {FormControl, FormField, FormItem, FormMessage} from "@/components/ui/form";
import {toTypedSchema} from "@vee-validate/zod"
import {z} from "zod";
import {useForm} from "vee-validate";
import {Field} from "@/components/ui/field";


const formats = [
  {
    id: 'custom',
    label: 'Formato Backup',
    desc: 'Exportar en formato .backup',
    def: true
  },
  {
    id: 'sql',
    label: 'Formato Sql',
    desc: 'Exportar en formato .sql',
    def: false
  },
] as const

const store = useAppStore()
const isConsoleOpen = ref(false)
const isBackingUp = ref(false)

let onEvent = new Channel<DownloadEvent>();


const formSchema = z.object({

  formats: z
      .array(z.string())
      .min(1, 'Debe seleccionar al menos un formato.')
      .refine(
          value => value.every(format => formats.some(t => t.id === format)),
          {
            message: 'Invalid notification type selected.',
          },
      ),
});


const {handleSubmit} = useForm({
  validationSchema: computed(() => toTypedSchema(formSchema)),
  keepValuesOnUnmount: true,
  initialValues:{
    formats:['custom']
  }
})


const onSubmit = handleSubmit(async (values) => {
  if (!isConsoleOpen.value) {
    isBackingUp.value = true;
    isConsoleOpen.value = true;

    //console.log('siuuuuuu', values.formats)
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    await invoke("backup", {
      dsName: store.currentOptions.dsName,
      dbName: store.currentOptions.dbName,
      schemaName: store.currentOptions.schema,
      onEvent: onEvent,
      formats: values.formats
    });



  } else {
    store.isBackupOpen = false;
  }
})

function handleOpenChange(open: boolean) {
  if (!open) {
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
          <div class="col-span-2 flex flex-col gap-2 ">
            <FormField v-slot="{ field,errors }" name="formats">
              <FormItem :data-invalid="!!errors.length">


                <Field
                    :data-invalid="!!errors.length"
                    v-for="format in formats"
                    :key="format.id"
                >
                  <Label


                      class="hover:bg-accent/50 flex items-start gap-3 rounded-lg border p-3 has-[[aria-checked=true]]:border-blue-600 has-[[aria-checked=true]]:bg-blue-50 dark:has-[[aria-checked=true]]:border-blue-900 dark:has-[[aria-checked=true]]:bg-blue-950"
                  >
                    <FormControl>
                      <Checkbox
                          :aria-invalid="!!errors.length"
                          :id="format.id"
                          :model-value="field.value?.includes(format.id)"
                          :default-value="format.def"
                          @update:modelValue="(checked)=>{

                                    const currentTasks = field.value || []
              const newValue = checked
                ? [...currentTasks, format.id]
                : currentTasks.filter(id => id !== format.id)
              field.onChange(newValue)
                    }"
                          class="data-[state=checked]:border-blue-600 data-[state=checked]:bg-blue-600 data-[state=checked]:text-white dark:data-[state=checked]:border-blue-700 dark:data-[state=checked]:bg-blue-700"


                      />
                      <div class="grid gap-1.5 font-normal">
                        <p class="text-sm leading-none font-medium">
                          {{ format.label }}
                        </p>
                        <p class="text-muted-foreground text-xs">
                          {{ format.desc }}
                        </p>
                      </div>
                    </FormControl>
                  </Label>
                </Field>


                <FormMessage/>
              </FormItem>

              <!--                <FieldError v-if="errors.length" :errors="errors"/>-->
            </FormField>
          </div>

        </div>
        <RestoreConsole v-else @done="isBackingUp=false" :channel="onEvent"/>

      </form>


      <SheetFooter>
        <Button @click="onSubmit" :disabled="isBackingUp || isConsoleOpen"> Crear backup</Button>
      </SheetFooter>
    </SheetContent>
  </Sheet>
</template>

<style scoped></style>
