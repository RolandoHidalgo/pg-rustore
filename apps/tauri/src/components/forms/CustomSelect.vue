<script setup lang="ts">

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,

  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'

const props = defineProps<{
  name: string,
  items: Array<string>
  label:string
}>()
</script>

<template>
  <FormField
    v-slot="{ componentField }"
    :name="props.name"
  >
    <FormItem>
      <FormLabel>{{ props.label }}</FormLabel>

      <Select v-bind="componentField" class="min-w-full">
        <SelectTrigger class="min-w-full">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
<!--            <SelectLabel>Versiones de postgres instaladas</SelectLabel>-->
            <SelectItem
              v-for="item in items"
              :key="item"
              :value="item"
            >
              {{ item }}
            </SelectItem>

          </SelectGroup>
        </SelectContent>
      </Select>

      <FormMessage />
    </FormItem>
  </FormField>
</template>

<style scoped>

</style>
