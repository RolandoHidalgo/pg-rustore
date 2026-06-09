<script setup lang="ts">

import MainPage from "@/components/MainPage.vue";
import BackupDrawer from "@/components/forms/BackupDrawer.vue";
import AddDatasourceDialog from "@/components/dataSources/AddDatasourceDialog.vue";
import BinaryInstaller from "@/components/forms/BinaryInstaller.vue";
import RestoreDrawer from "@/components/forms/RestoreDrawer.vue";
import {listen} from '@tauri-apps/api/event';
import {onMounted} from "vue";
import {useAppStore} from "@/stores/appStore.ts";
import AboutDialog from "@/components/AboutDialog.vue";
import DropDrawer from "@/components/forms/DropDrawer.vue";

const store = useAppStore();
onMounted(() => {
  listen('open-file', (event) => {
    console.log(
        `event`, event
    );
    store.openRestore('', event.payload)
  });
})
</script>

<template>
  <div class="bg-muted/40 h-full flex flex-col px-2 py-0">
    <!--    <div>-->
    <!--      <RouterLink to="/splashscreen">splash</RouterLink>-->
    <!--      <RouterLink to="/Login">Login</RouterLink>-->
    <!--    </div>-->
    <MainPage/>

    <BackupDrawer/>
    <DropDrawer/>
    <RestoreDrawer/>
    <AddDatasourceDialog/>
    <BinaryInstaller/>
    <!--    <DeleteDatasourceDialog />-->
    <!--    <AboutDialog />-->
    <AboutDialog/>
    <!--    <SyncDialog />-->
  </div>
</template>
