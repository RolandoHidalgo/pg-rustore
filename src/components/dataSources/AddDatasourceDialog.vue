<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { computed, ref, toValue, watchEffect } from 'vue'
import { ReloadIcon } from '@radix-icons/vue'
import { z } from 'zod'
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/zod'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from '@/components/ui/form'



import { CardContent } from '@/components/ui/card'
import { Input } from '@/components/ui/input'

import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger
} from '@/components/ui/sheet'
import { useAppStore } from '@/stores/appStore'
import {useDatasourceStore} from "@/stores/datasourceStore.ts";
import BinarySelect from "@/components/dataSources/BinarySelect.vue";
import {useDbStore} from "@/stores/dbStore.ts";

const store = useAppStore()
const dbStore = useDbStore();
const dsStore = useDatasourceStore()

const updating = ref(false)
const isEdit = ref(false)
const isSSH = ref(false)

const baseSchema = z.object({
  password: z
    .string({
      required_error: 'Requerido.'
    })
    .min(1, { message: 'no vacio' }),
  user: z
    .string({
      required_error: 'Requerido.'
    })
    .min(1, { message: 'no vacio' }),
  port: z
    .number({
      required_error: 'Requerido.'
    })
    .min(1, { message: 'no vacio' }),
  name: z
    .string({
      required_error: 'Requerido.'
    })
    .min(1, { message: 'no vacio' })
})

const noSSHSchema = z.object({
  bin: z.string({
    required_error: 'Requerido.'
  }),

  host: z
    .string({
      required_error: 'Requerido.'
    })
    .min(1, { message: 'no vacio' })
})

const sshShema = z.object({
  sshPassword: z
    .string({
      required_error: 'Requerido.'
    })
    .min(1, { message: 'no vacio' }),
  sshUsername: z
    .string({
      required_error: 'Requerido.'
    })
    .min(1, { message: 'no vacio' }),

  sshHost: z
    .string({
      required_error: 'Requerido.'
    })
    .min(1, { message: 'no vacio' }),
  sshPort: z
    .number({
      required_error: 'Requerido.'
    })
    .min(1, { message: 'no vacio' })
})

const finalSchema = computed(() => {
  let schema = toValue(baseSchema)
  if (isSSH.value) {
    schema = schema.merge(sshShema)
  } else {
    schema = schema.merge(noSSHSchema)
  }

  return schema.passthrough()
})

const initVals = {
  port: 5432,
  host: 'localhost',
  user: 'postgres'
}
const { handleSubmit, resetForm,  setValues } = useForm({
  validationSchema: computed(() => toTypedSchema(finalSchema.value)),
  initialValues: initVals,
  keepValuesOnUnmount: true
})

const onSubmit = handleSubmit(async (values) => {
  const finalValues = isSSH.value
    ? { ...values, isSSH: true, host: 'localhost', binary: '' }
    : {
        ...values,
        isSSH: false
      }


  // await dsStore.loadDataSources()

  if (isEdit.value) {
    //useConexionStore().loadDbs()
    await dsStore.editDatasource(finalValues)
    dbStore.loadDbs(dsStore.activeDs!.name);

  }else{
    await dsStore.addDatasource(finalValues)
  }
  store.isDataSourceFormOpen = false
  resetForm()

})
watchEffect(() => {
  if (!store.isDataSourceFormOpen) {
    store.currentDsForm = null
    resetForm()
  }
  // else {
  //   if (store.currentDsForm) {

  //   }
  // }
})

store.$onAction(({ name, after }) => {
  after(() => {
    if (name === 'openDataSourceForm') {
      setValues({ ...store.currentDsForm })
      
      isEdit.value = store.currentDsForm !== null
      //resetForm({ values:  })
      store.currentDsForm = null
    }
  })
})
</script>

<template>
  <Sheet v-model:open="store.isDataSourceFormOpen" class="p-0!">
    <SheetTrigger></SheetTrigger>
    <SheetContent side="bottom" class="rounded-t-lg">
      <SheetHeader>
        <SheetTitle>{{ `${isEdit ? 'Editar' : 'Adicionar'}` }} datasource</SheetTitle>
        <SheetDescription> Parámetros de conexión.</SheetDescription>
      </SheetHeader>
      <form class="w-full flex flex-col" @submit="onSubmit">
        <CardContent class="grid grid-cols-2 gap-2 overflow-y-auto pb-4">
<!--          <div class="col-span-2">-->
<!--            <FormField name="isSSH">-->
<!--              <FormItem class="flex flex-row items-center justify-between rounded-lg border p-4">-->
<!--                <div class="space-y-0.5">-->
<!--                  <FormLabel class="text-base"> SSH</FormLabel>-->
<!--                  <FormDescription> Establecer esta conexión por SSH.</FormDescription>-->
<!--                </div>-->
<!--                <FormControl>-->
<!--                  <Switch id="new-db" v-model:checked="isSSH" />-->
<!--                </FormControl>-->
<!--              </FormItem>-->
<!--            </FormField>-->
<!--          </div>-->
          <div v-if="!isSSH">
            <BinarySelect />
          </div>
          <div>
            <FormField v-slot="{ componentField }" name="name">
              <FormItem>
                <FormLabel>Nombre</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" :disabled="isEdit" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </div>
          <div>
            <FormField v-slot="{ componentField }" name="password">
              <FormItem>
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input type="password" v-bind="componentField" />
                </FormControl>

                <FormMessage />
              </FormItem>
            </FormField>
          </div>
          <div>
            <FormField v-slot="{ componentField }" name="user">
              <FormItem>
                <FormLabel>User</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </div>
          <div v-if="!isSSH">
            <FormField v-slot="{ componentField }" name="host">
              <FormItem>
                <FormLabel>Host</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </div>
          <div>
            <FormField v-slot="{ componentField }" name="port">
              <FormItem>
                <FormLabel>Port</FormLabel>
                <FormControl>
                  <Input type="number" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </div>
          <template v-if="isSSH">
            <div>
              <FormField v-slot="{ componentField }" name="sshPassword">
                <FormItem>
                  <FormLabel>SSH Password</FormLabel>
                  <FormControl>
                    <Input type="password" v-bind="componentField" />
                  </FormControl>

                  <FormMessage />
                </FormItem>
              </FormField>
            </div>
            <div>
              <FormField v-slot="{ componentField }" name="sshUsername">
                <FormItem>
                  <FormLabel>SSH User</FormLabel>
                  <FormControl>
                    <Input type="text" v-bind="componentField" />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              </FormField>
            </div>
            <div>
              <FormField v-slot="{ componentField }" name="sshHost">
                <FormItem>
                  <FormLabel>SSH Host</FormLabel>
                  <FormControl>
                    <Input type="text" v-bind="componentField" />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              </FormField>
            </div>
            <div>
              <FormField v-slot="{ componentField }" name="sshPort">
                <FormItem>
                  <FormLabel>SSH Port</FormLabel>
                  <FormControl>
                    <Input type="number" v-bind="componentField" />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              </FormField>
            </div>
          </template>
        </CardContent>
      </form>
      <SheetFooter >
        <Button @click="onSubmit" :disabled="updating">
          <ReloadIcon class="w-4 h-4 mr-2 animate-spin" v-if="updating" />
          Aceptar
        </Button>
      </SheetFooter>
    </SheetContent>
  </Sheet>
</template>

<style scoped></style>
