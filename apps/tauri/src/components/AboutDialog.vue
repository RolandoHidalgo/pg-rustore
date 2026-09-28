<script setup lang="ts">


import {GithubLogoIcon} from '@radix-icons/vue'
import {onMounted, ref} from 'vue'
import {ReloadIcon} from '@radix-icons/vue'
import {
  Sheet,
  SheetContent,
  SheetDescription, SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger
} from '@/components/ui/sheet'

import {useAppStore} from '@/stores/appStore'
import {Progress} from '@/components/ui/progress'
import {Button} from "@/components/ui/button";
import {Avatar, AvatarFallback} from "@/components/ui/avatar";
import {check} from '@tauri-apps/plugin-updater';
import {relaunch} from '@tauri-apps/plugin-process';
import {getVersion} from '@tauri-apps/api/app';

const store = useAppStore()

const messages = ref('')
const updating = ref(false)
const version = ref('');

const downloaded = ref(0);
const contentLength = ref(0);




const porciento = ref(0);
onMounted(async () => {
  version.value = await getVersion();
})
const checkUpdates = async () => {

  porciento.value = 0;
  downloaded.value = 0;
  contentLength.value = 0;
  messages.value = '';
  const update = await check();
  if (update) {
    console.log(
        `found update ${update.version} from ${update.date}`
    );
    updating.value = true
    messages.value = `found update ${update.version} from ${update.date}`;

    // alternatively we could also call update.download() and update.install() separately
    try {
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength.value = event.data.contentLength!;
            console.log(`started downloading ${event.data.contentLength} bytes`);
            messages.value = `started downloading ${event.data.contentLength} bytes`;
            break;
          case 'Progress':

            downloaded.value += event.data.chunkLength;
            console.log(`downloaded ${downloaded} from ${contentLength}`);

            porciento.value = downloaded.value * 100 / contentLength.value;


            messages.value = `downloaded ${downloaded.value} from ${contentLength.value}`;
            break;
          case 'Finished':
            console.log('download finished');
            break;
        }
      });
    } catch (e) {
      messages.value = e + 'asdasdad';
    }

    console.log('update installed');
    messages.value = 'instalo'
    await relaunch();
  } else {
    console.log("no hay nada")
    messages.value = 'no hay nada';
  }

}

</script>

<template>
  <Sheet v-model:open="store.isAboutOpen" class="p-0!">
    <SheetTrigger></SheetTrigger>
    <SheetContent side="bottom" class="rounded-t-lg">
      <SheetHeader>
        <SheetTitle>Acerca de ...</SheetTitle>
        <SheetDescription> Infomación sobre esta versión. siuu</SheetDescription>
      </SheetHeader>
      <div class="flex items-center gap-4  justify-between space-x-4 px-2">
        <div class="flex items-center  space-x-4">
          <Avatar>

            <AvatarFallback>PG</AvatarFallback>
          </Avatar>
          <div>
            <p class="text-sm font-medium leading-none">
              PG-RUSTORE
            </p>
            <p class="text-sm text-muted-foreground">
              version {{ version }}
            </p>
          </div>
        </div>
        <a href="https://github.com/RolandoHidalgo/pg-restore-electron" target="_blank">
          <Button variant="outline">
            <GithubLogoIcon class="mr-2 h-4 w-4"/>
            GitHub
          </Button>
        </a>
      </div>
      <div v-if="updating" class="px-2">
        <p class="text-sm font-medium leading-none">
          <Progress :model-value="porciento"/>
        </p>
        <p class="text-sm text-muted-foreground">
          {{ messages }}
        </p>
      </div>
      <template v-else>
        {{ messages }}
      </template>
      <div class="flex justify-end space-x-4 text-sm text-muted-foreground mb-0 px-3">
        <div>
          Created and maintained by
          <a
              href="https://github.com/RolandoHidalgo"
              target="_blank"
              class="text-primary"
          >
            @SkidRow
          </a>
        </div>

      </div>
      <SheetFooter>
        <Button
            @click="checkUpdates"
            :disabled="updating"
        >
          <ReloadIcon
              class="w-4 h-4 mr-2 animate-spin"
              v-if="updating"
          />
          Actualizar
        </Button>
      </SheetFooter>
    </SheetContent>
  </Sheet>

</template>

<style scoped>

</style>
