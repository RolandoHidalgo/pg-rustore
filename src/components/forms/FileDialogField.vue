<script setup lang="ts">
import {open} from "@tauri-apps/plugin-dialog";
import {useField} from 'vee-validate'
import {FormControl, FormItem, FormLabel, FormMessage} from "@/components/ui/form";
import {Button} from "@/components/ui/button";
import {computed} from "vue";
import {HoverCard, HoverCardContent, HoverCardTrigger} from '@/components/ui/hover-card'
import {CalendarIcon, Info} from 'lucide-vue-next'
import useBackupInfo from "@/components/forms/useBackupInfo.ts";

const props = defineProps({
  name: {type: String, required: true},
  label: {type: String, default: 'Select file'}
})

const {value, errorMessage, handleChange} = useField<string>(props.name)


const valueShow = computed(() => {
  if (value.value) {

    const base = value.value.split('.backup')[0];

    if (base.includes("\\")) {
      const paths = base.split("\\");
      return paths[paths.length - 1];
    }
    return base;
  }
  return '';
})
const selectFile = async () => {
  const selected = await open({
    multiple: false,
    directory: false
  })
  if (selected) {
    // setea el valor en vee-validate
    handleChange(selected)
  }
}

const backupInfo = useBackupInfo(value);

</script>

<template>
  <FormItem>
    <FormLabel>{{ props.label }}</FormLabel>
    <FormControl>
      <div class="flex gap-2 justify-start items-center">
                <span
                    v-if="value"
                    class="text-sm text-muted-foreground"
                >{{ valueShow }}</span>
        <HoverCard v-if="backupInfo.db_name !== ''">
          <HoverCardTrigger as-child>
            <Info class="text-muted-foreground size-4"/>
          </HoverCardTrigger>
          <HoverCardContent class="w-78 mx-2  ">
            <div class="flex justify-between space-x-4">
              <!--                        <div-->
              <!--                          class="flex aspect-square size-8 items-center justify-center rounded-lg bg-blue-500 text-white"-->
              <!--                        >-->
              <!--                          <DatabaseZap />-->
              <!--                        </div>-->
              <div class="space-y-1">
                <h4 class="text-sm font-semibold">Backup info:</h4>
                <p class="text-sm">
                  Backup file of db
                  <span class="font-bold">{{ backupInfo.db_name }}</span> created using
                  postgres
                  <span class="font-bold">{{ backupInfo.db_version }}</span> and pg_dump
                  <span class="font-bold">{{ backupInfo.pg_dump_version }}</span>
                </p>
                <div class="flex items-center pt-2">
                  <CalendarIcon class="mr-2 h-4 w-4 opacity-70"/>
                  <span class="text-xs text-muted-foreground">
                              Created at {{ backupInfo.fecha }}
                            </span>
                </div>
              </div>
            </div>

          </HoverCardContent>
        </HoverCard>

      </div>

      <Button

          type="button"
          variant="outline"
          size="sm"
          @click="selectFile"
      >
        Seleccionar Backup
      </Button>

    </FormControl>
    <FormMessage>{{ errorMessage }}</FormMessage>
  </FormItem>
</template>
