<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { useField } from 'vee-validate'
import {FormControl, FormItem, FormLabel, FormMessage} from "@/components/ui/form";

const props = defineProps({
  name: { type: String, required: true },
  label: { type: String, default: 'Select file' }
})

const { value, errorMessage, handleChange } = useField(props.name)

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
</script>

<template>
  <FormItem>
    <FormLabel>{{ props.label }}</FormLabel>
    <FormControl>
      <button
          type="button"
          class="shadcn-btn"
          @click="selectFile"
      >
        Abrir dialog
      </button>
      <span v-if="value">{{ value }}</span>
    </FormControl>
    <FormMessage>{{ errorMessage }}</FormMessage>
  </FormItem>
</template>
