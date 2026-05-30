<script setup lang="ts">
import {open} from "@tauri-apps/plugin-dialog";
import {useField} from 'vee-validate'
import {FormControl, FormItem, FormLabel, FormMessage} from "@/components/ui/form";
import {Button} from "@/components/ui/button";
import {computed} from "vue";

const props = defineProps({
  name: {type: String, required: true},
  label: {type: String, default: 'Select file'}
})

const {value, errorMessage, handleChange} = useField(props.name)
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
</script>

<template>
  <FormItem>
    <FormLabel>{{ props.label }}</FormLabel>
    <FormControl>
      <span v-if="value">Backup: {{ valueShow }}</span>
      <Button

          type="button"
          variant="ghost"
          size="sm"
          @click="selectFile"
      >
        Seleccionar Backup
      </Button>

    </FormControl>
    <FormMessage>{{ errorMessage }}</FormMessage>
  </FormItem>
</template>
