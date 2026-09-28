<script setup lang="ts">
import {computed, HTMLAttributes, ref} from "vue"
import {cn} from "@/lib/utils"


import {GalleryVerticalEnd, LoaderIcon} from 'lucide-vue-next'
import {Button} from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import {
  Field, FieldDescription,

  FieldGroup,
  FieldLabel,
} from "@/components/ui/field"
import {Input} from "@/components/ui/input"
import {useAppStore} from "@/stores/appStore.ts";
import {useRouter} from "vue-router";

const props = defineProps<{
  class?: HTMLAttributes["class"]
}>()
const deleting = ref(false);
const router = useRouter();
const store = useAppStore();
const passwd = ref('')
const isValidForm = computed(() => {
  return passwd.value !== '';
})
const loading = ref(false);

async function login() {
  loading.value = true;
  try {
    const valid = await store.login(passwd.value);
    console.log(valid);

    if (valid) {
      console.log("valid")
      router.push("/")
      //loading.value = false;
    }
  } finally {
    loading.value = false
  }
}

async function deletePassword() {
  try {
    deleting.value = true
    await store.deletePass()
    deleting.value = false;
  } catch (e) {

  } finally {

    deleting.value = false;
  }
}
</script>

<template>
  <div class="bg-muted flex min-h-svh flex-col items-center justify-center gap-6 p-4 md:p-10">
    <div class="flex w-full max-w-sm flex-col gap-6">
      <a href="#" class="flex items-center gap-2 self-center font-medium">
        <div class="bg-primary text-primary-foreground flex size-6 items-center justify-center rounded-md">
          <GalleryVerticalEnd class="size-4"/>
        </div>
        PG_RUSTORE.
      </a>
      <div :class="cn('flex flex-col gap-6', props.class)">
        <Card>
          <CardHeader class="text-center">
            <CardTitle class="text-xl">
              Bienvenido
            </CardTitle>
            <CardDescription>
              Use su clave maestra para acceder.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <form @submit.prevent="login">
              <FieldGroup>


                <Field>
                  <div class="flex items-center">
                    <FieldLabel for="password">
                      Password
                    </FieldLabel>

                  </div>
                  <Input
                      id="password"
                      type="password"
                      v-model="passwd"
                      required/>
                </Field>
                <Field>
                  <Button type="submit" :disabled="!isValidForm">
                    <LoaderIcon v-if="loading"
                                role="status"
                                aria-label="Loading"
                                class="size-4 animate-spin"
                    />
                    Login
                  </Button>
                  <Button type="button" variant="outline" @click="deletePassword">
                    <LoaderIcon v-if="deleting"
                                role="status"
                                aria-label="Loading"
                                class="size-4 animate-spin"
                    />
                    Delete password
                  </Button>
                  <FieldDescription class="text-center">
                    La primera vez se establecerá la contraseña maestra

                  </FieldDescription>
                </Field>
              </FieldGroup>
            </form>
          </CardContent>
        </Card>

      </div>
    </div>
  </div>
</template>
