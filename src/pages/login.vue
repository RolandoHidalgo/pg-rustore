<script setup lang="ts">
import {computed, HTMLAttributes, ref} from "vue"
import {cn} from "@/lib/utils"
import {Button} from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import {
  Field,

  FieldGroup,
  FieldLabel,
} from "@/components/ui/field"
import {Input} from "@/components/ui/input"
import {useAppStore} from "@/stores/appStore.ts";
import {useRouter} from "vue-router";

const props = defineProps<{
  class?: HTMLAttributes["class"]
}>()

const router = useRouter();
const store = useAppStore();
const passwd = ref('')
const isValidForm = computed(() => {
  return passwd.value !== '';
})
const loading = ref(false);

async function login() {
  loading.value = true;
  const valid = await store.login(passwd.value);
  console.log(valid);
  loading.value = false;
  if (valid) {
    console.log("valid")
    router.push("/")
  }
}

async function deletePassword() {
  await store.deletePass()
}
</script>

<template>
  <div class="flex min-h-svh w-full items-center justify-center p-6 md:p-10">
    <div class="w-full max-w-sm">
      <!--     form-->
      <div :class="cn('flex flex-col gap-6', props.class)">
        <Card>
          <CardHeader>
            <CardTitle>Login</CardTitle>
            <CardDescription>
              Ponga un password
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
                  />
                </Field>
                <Field>
                  <Button type="submit" :disabled="!isValidForm">
                    Login
                  </Button>
                  <Button variant="outline" type="button" @click="deletePassword">
                    delete passwrod
                  </Button>

                  <!--                  <FieldDescription class="text-center">-->
                  <!--                    Don't have an account?-->
                  <!--                    <a href="#">-->
                  <!--                      Sign up-->
                  <!--                    </a>-->
                  <!--                  </FieldDescription>-->
                </Field>
              </FieldGroup>
            </form>
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>
