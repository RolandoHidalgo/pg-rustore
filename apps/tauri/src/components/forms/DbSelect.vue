<script setup lang="ts">
import {Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue} from '@/components/ui/select'
import {FormItem, FormLabel, FormField, FormMessage} from '@/components/ui/form'
import {useDbStore} from "@/stores/dbStore.ts";
import {watchEffect} from "vue";



const props =defineProps<{
  dsName: string
}>()
const dbStore = useDbStore();
watchEffect(()=>{
  if(props.dsName && props.dsName !=''){
    dbStore.loadDbs(props.dsName)
  }
})


</script>

<template>
  <FormField
    v-slot="{ componentField }"
    name="dbName"
    class="w-full"
  >
    <FormItem>
      <FormLabel>Db name.</FormLabel>

      <Select v-bind="componentField">
        <SelectTrigger class="min-w-full">
          <SelectValue/>
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectLabel>Bases de datos en {{props.dsName}}</SelectLabel>
            <SelectItem
              v-for="db in dbStore.dbs"
              :key="db"
              :value="db"
            >
              {{ db }}
            </SelectItem>
            <!--              <SelectItem value="banana">-->
            <!--                Banana-->
            <!--              </SelectItem>-->
            <!--              <SelectItem value="blueberry">-->
            <!--                Blueberry-->
            <!--              </SelectItem>-->
            <!--              <SelectItem value="grapes">-->
            <!--                Grapes-->
            <!--              </SelectItem>-->
            <!--              <SelectItem value="pineapple">-->
            <!--                Pineapple-->
            <!--              </SelectItem>-->
          </SelectGroup>
        </SelectContent>
      </Select>

      <FormMessage/>
    </FormItem>
  </FormField>
</template>

<style scoped>

</style>
