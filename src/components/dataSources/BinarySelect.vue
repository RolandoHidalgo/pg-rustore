<script setup lang="ts">
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";


import {FormItem, FormLabel, FormField, FormMessage} from "@/components/ui/form"
import {onMounted, ref} from "vue";
import {useAppStore} from "@/stores/appStore.ts";
import {BinaryInfo} from "@/types";



const binaries = ref<BinaryInfo[]>([]);
const store = useAppStore()
onMounted(async () => {
  binaries.value = (await store.getBinaries()).map(e=>{
    return {...e,binary:e.binary.replace(/\\/g,'/').replace(/\/\//,'/')}
  });
  console.log(binaries.value)
})

</script>

<template>
  <FormField
      v-slot="{ componentField }"
      name="bin"
  >
    <FormItem>
      <FormLabel>Binario.</FormLabel>

      <Select v-bind="componentField">
        <SelectTrigger class="min-w-full">
          <SelectValue/>
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectLabel>Versiones de postgres instaladas</SelectLabel>
            <SelectItem
                v-for="binary in binaries"
                :key="binary.binary"
                :value="binary.binary"
            >
              {{ binary.version }} - {{ binary.arq }}
            </SelectItem>

          </SelectGroup>
        </SelectContent>
      </Select>

      <FormMessage/>
    </FormItem>
  </FormField>
</template>

<style scoped>

</style>
