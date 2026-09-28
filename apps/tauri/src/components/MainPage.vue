<script setup lang="ts">
import {ref} from "vue";
import {
  Sidebar,
  SidebarContent, SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader, SidebarMenu, SidebarProvider
} from "@/components/ui/sidebar";
import DataSourceSwitcher from "@/components/dataSources/DataSourceSwitcher.vue";
import DBList from "@/components/dataSources/DBList.vue";
import SearchForm from "@/components/forms/SearchForm.vue";
import OptionsMenu from "@/components/OptionsMenu.vue";

const currentDs = ref('')
const search = ref('')
</script>

<template>
  <SidebarProvider>
    <Sidebar collapsible="none" variant="sidebar" class="w-100 h-screen">
      <SidebarHeader>
        <DataSourceSwitcher @change="currentDs=$event"/>
      </SidebarHeader>
      <SearchForm @input="search=$event"></SearchForm>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Databases</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <DBList :ds-name="currentDs" :search="search"></DBList>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <OptionsMenu/>

      </SidebarFooter>
    </Sidebar>
  </SidebarProvider>



</template>

<style scoped></style>
